import { useEffect, useMemo, useState, type ReactNode } from "react";
import { invoke } from "@tauri-apps/api/core";
import { emitTo, listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { CockpitLayout } from "./components/CockpitLayout/CockpitLayout";
import { ContextPanel } from "./components/ContextPanel/ContextPanel";
import {
  ProjectCard,
  type ProjectStatus,
} from "./components/ProjectCard/ProjectCard";
import {
  HourglassIcon,
  MoonIcon,
  PaintbrushIcon,
  VesperionIcon as ProjectVesperionIcon,
} from "./components/ProjectCard/ProjectIcons";
import {
  ProjectTree,
  type ProjectGenealogyBranch,
} from "./components/ProjectTree/ProjectTree";
import vesperionIcon from "./assets/vesperion-icon-512.png";
import "./App.css";

type CopyState = "idle" | "copied" | "error";

type CompanionSettings = {
  visible: boolean;
  shadow: boolean;
};

type ProjectDirectory = {
  name: string;
  path: string;
  modified_at_epoch_seconds?: number;
  status: ProjectStatus;
};

type ProjectBranchResponse = {
  id: ProjectGenealogyBranch["id"];
  label: string;
  path?: string;
  exists: boolean;
};

type SessionSnapshot = {
  content: string;
  display_date: string;
};

type LocalProject = {
  name: string;
  icon: ReactNode;
  iconTone: "gold" | "purple" | "magenta" | "blue" | "default";
  type: string;
  description: string;
  status: ProjectStatus;
  lastSession: string;
  locationLabel: string;
  summary: string;
  sessionEnd: string;
};

const projectsRoot = "C:\\Ph3yNyx.OS\\05_⭐VESPΣR";
const companionSettingsStorageKey = "vesperion-companion-settings";
const companionWindowLabel = "vesperion";
const trayToggleCompanionEvent = "tray-toggle-companion";
const trayRefreshAppEvent = "tray-refresh-app";

const globalAccesses = [
  { label: "Obsidian", icon: "◈" },
  { label: "GitHub", icon: "◇" },
  { label: "Explorateur", icon: "□" },
  { label: "VS Code", icon: "⌁" },
  { label: "Terminal", icon: ">_" },
] as const;

function readCompanionSettings(): CompanionSettings {
  try {
    const storedSettings = localStorage.getItem(companionSettingsStorageKey);
    if (!storedSettings) return { visible: true, shadow: true };

    const parsedSettings = JSON.parse(storedSettings) as Partial<CompanionSettings>;

    return {
      visible:
        typeof parsedSettings.visible === "boolean"
          ? parsedSettings.visible
          : true,
      shadow:
        typeof parsedSettings.shadow === "boolean" ? parsedSettings.shadow : true,
    };
  } catch {
    return { visible: true, shadow: true };
  }
}

function formatModifiedDate(epochSeconds?: number) {
  if (!epochSeconds) return "Non disponible";

  return new Intl.DateTimeFormat("fr-CA", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  }).format(new Date(epochSeconds * 1000));
}

async function ensureCompanionWindow() {
  const existingWindow = await WebviewWindow.getByLabel(companionWindowLabel);
  if (existingWindow) {
    await existingWindow.show();
    return existingWindow;
  }

  const companionWindow = new WebviewWindow(companionWindowLabel, {
    title: "VESPΣRION",
    url: "index.html?window=vesperion",
    width: 380,
    height: 240,
    resizable: false,
    decorations: false,
    transparent: true,
    alwaysOnTop: true,
    skipTaskbar: true,
    shadow: false,
    focus: false,
    visible: true,
    parent: "main",
  });

  await new Promise<void>((resolve, reject) => {
    const handleCreated = () => resolve();
    const handleError = (event: { payload: unknown }) => reject(event.payload);

    void companionWindow.once("tauri://created", handleCreated);
    void companionWindow.once("tauri://error", handleError);
  });

  await companionWindow.show();

  return companionWindow;
}

async function destroyCompanionWindow() {
  const companionWindow = await WebviewWindow.getByLabel(companionWindowLabel);

  if (companionWindow) {
    await companionWindow.destroy();
  }
}

function getProjectIcon(name: string) {
  const normalizedName = name.toLowerCase();

  if (normalizedName.includes("hr0nos") || normalizedName.includes("chr0nos")) {
    return {
      icon: <HourglassIcon />,
      iconTone: "gold" as const,
    };
  }

  if (normalizedName.includes("lun")) {
    return {
      icon: <MoonIcon />,
      iconTone: "purple" as const,
    };
  }

  if (normalizedName.includes("astr4l")) {
    return {
      icon: <PaintbrushIcon />,
      iconTone: "blue" as const,
    };
  }

  if (normalizedName.includes("vesp")) {
    return {
      icon: <ProjectVesperionIcon />,
      iconTone: "magenta" as const,
    };
  }

  return {
    icon: "□",
    iconTone: "default" as const,
  };
}

function toLocalProject(directory: ProjectDirectory): LocalProject {
  const lastSession = formatModifiedDate(directory.modified_at_epoch_seconds);
  const projectIcon = getProjectIcon(directory.name);

  return {
    name: directory.name,
    icon: projectIcon.icon,
    iconTone: projectIcon.iconTone,
    type: "Dossier local",
    description: "Projet reflété depuis le dossier VESPΣR.",
    status: directory.status,
    lastSession,
    locationLabel: directory.path,
    summary: `Dossier projet présent dans ${projectsRoot}.`,
    sessionEnd: `Dernier état local observé : ${lastSession}.`,
  };
}

function App() {
  const [projects, setProjects] = useState<LocalProject[]>([]);
  const [activeProjectName, setActiveProjectName] = useState<string>();
  const [projectLoadState, setProjectLoadState] =
    useState<"loading" | "ready" | "error">("loading");
  const [companionSettings, setCompanionSettings] = useState(
    readCompanionSettings,
  );
  const [projectBranches, setProjectBranches] = useState<ProjectGenealogyBranch[]>([]);
  const [projectTreeState, setProjectTreeState] =
    useState<"loading" | "ready" | "error">("loading");
  const [isCompanionPanelOpen, setIsCompanionPanelOpen] = useState(false);
  const [officialContextContent, setOfficialContextContent] = useState<string>();
  const [latestSessionMarkdown, setLatestSessionMarkdown] = useState<string>();
  const [latestSessionDate, setLatestSessionDate] = useState<string>();
  const [sessionPreviewState, setSessionPreviewState] = useState<
    "loading" | "ready" | "empty"
  >("loading");
  const [contextCopyState, setContextCopyState] = useState<CopyState>("idle");
  const [sessionCopyState, setSessionCopyState] = useState<CopyState>("idle");
  const activeProject = useMemo(
    () =>
      projects.find((project) => project.name === activeProjectName) ??
      projects[0],
    [activeProjectName, projects],
  );

  useEffect(() => {
    let isMounted = true;

    async function loadProjects() {
      try {
        const directories = await invoke<ProjectDirectory[]>(
          "list_project_directories",
        );
        const nextProjects = directories.map(toLocalProject);

        if (!isMounted) return;
        setProjects(nextProjects);
        setActiveProjectName((currentProjectName) => {
          if (
            currentProjectName &&
            nextProjects.some((project) => project.name === currentProjectName)
          ) {
            return currentProjectName;
          }

          return nextProjects[0]?.name;
        });
        setProjectLoadState("ready");
      } catch (error: unknown) {
        console.error("Unable to read local project directories", error);
        if (isMounted) setProjectLoadState("error");
      }
    }

    loadProjects();

    return () => {
      isMounted = false;
    };
  }, []);

  useEffect(() => {
    void getCurrentWindow().setIcon(vesperionIcon).catch((error: unknown) => {
      console.error("Unable to set window icon", error);
    });
  }, []);

  useEffect(() => {
    localStorage.setItem(
      companionSettingsStorageKey,
      JSON.stringify(companionSettings),
    );

    void (async () => {
      try {
        if (!companionSettings.visible) {
          await destroyCompanionWindow();
          return;
        }

        const companionWindow = await ensureCompanionWindow();
        await companionWindow.emit("vesperion-settings", companionSettings);
      } catch (error: unknown) {
        console.error("Unable to apply VESPΣRION settings", error);
      }
    })();
  }, [companionSettings]);

  useEffect(() => {
    let isMounted = true;
    const stopListening: Array<() => void> = [];

    void listen(trayToggleCompanionEvent, () => {
      if (!isMounted) return;

      setCompanionSettings((currentSettings) => ({
        ...currentSettings,
        visible: !currentSettings.visible,
      }));
    }).then((unlisten) => {
      if (!isMounted) {
        unlisten();
        return;
      }

      stopListening.push(unlisten);
    });

    void listen(trayRefreshAppEvent, () => {
      window.location.reload();
    }).then((unlisten) => {
      if (!isMounted) {
        unlisten();
        return;
      }

      stopListening.push(unlisten);
    });

    return () => {
      isMounted = false;
      stopListening.forEach((unlisten) => unlisten());
    };
  }, []);

  useEffect(() => {
    let isMounted = true;

    async function loadProjectBranches() {
      if (!activeProject) {
        setProjectBranches([]);
        setProjectTreeState("ready");
        return;
      }

      setProjectTreeState("loading");

      try {
        const branches = await invoke<ProjectBranchResponse[]>(
          "list_project_genealogy",
          {
            projectPath: activeProject.locationLabel,
          },
        );

        if (!isMounted) return;
        setProjectBranches(branches);
        setProjectTreeState("ready");
      } catch (error: unknown) {
        console.error("Unable to read project genealogy", error);
        if (!isMounted) return;
        setProjectBranches([]);
        setProjectTreeState("error");
      }
    }

    loadProjectBranches();

    return () => {
      isMounted = false;
    };
  }, [activeProject]);

  useEffect(() => {
    let isMounted = true;

    async function loadOfficialContext() {
      if (!activeProject) {
        setOfficialContextContent(undefined);
        return;
      }

      try {
        const officialContext = await invoke<string>("read_project_official_context", {
          projectPath: activeProject.locationLabel,
        });

        if (!isMounted) return;
        setOfficialContextContent(officialContext);
      } catch (error: unknown) {
        console.error("Unable to load the official project context", error);
        if (!isMounted) return;
        setOfficialContextContent(undefined);
      }
    }

    async function loadLatestSessionMarkdown() {
      if (!activeProject) {
        setLatestSessionMarkdown(undefined);
        setLatestSessionDate(undefined);
        setSessionPreviewState("empty");
        return;
      }

      setSessionPreviewState("loading");

      try {
        const latestSession = await invoke<SessionSnapshot>(
          "read_latest_project_session_snapshot",
          {
            projectName: activeProject.name,
          },
        );

        if (!isMounted) return;
        setLatestSessionMarkdown(latestSession.content);
        setLatestSessionDate(latestSession.display_date);
        setSessionPreviewState("ready");
      } catch (error: unknown) {
        console.error("Unable to load the latest project session markdown", error);
        if (!isMounted) return;
        setLatestSessionMarkdown(undefined);
        setLatestSessionDate(undefined);
        setSessionPreviewState("empty");
      }
    }

    loadOfficialContext();
    loadLatestSessionMarkdown();

    return () => {
      isMounted = false;
    };
  }, [activeProject]);

  function fallbackCopyText(value: string) {
    const textarea = document.createElement("textarea");
    textarea.value = value;
    textarea.setAttribute("readonly", "");
    textarea.style.position = "fixed";
    textarea.style.opacity = "0";
    document.body.appendChild(textarea);
    textarea.select();
    const didCopy = document.execCommand("copy");
    document.body.removeChild(textarea);
    return didCopy;
  }

  async function copyText(value: string, setState: (state: CopyState) => void) {
    let didCopy = false;

    try {
      await navigator.clipboard.writeText(value);
      didCopy = true;
    } catch {
      didCopy = fallbackCopyText(value);
    }

    setState(didCopy ? "copied" : "error");
  }

  async function copyProjectContext() {
    if (!activeProject) return;

    try {
      if (officialContextContent) {
        return copyText(officialContextContent, setContextCopyState);
      }

      const officialContext = await invoke<string>("read_project_official_context", {
        projectPath: activeProject.locationLabel,
      });

      return copyText(officialContext, setContextCopyState);
    } catch (error: unknown) {
      console.error("Unable to read the official project context", error);

      return copyText(
        [
          activeProject.name,
          activeProject.summary,
          `Emplacement : ${activeProject.locationLabel}`,
          `Dernière session : ${activeProject.lastSession}`,
        ].join("\n\n"),
        setContextCopyState,
      );
    }
  }

  async function copySessionMarkdown() {
    if (!activeProject) return;

    try {
      if (latestSessionMarkdown) {
        return copyText(latestSessionMarkdown, setSessionCopyState);
      }

      const latestSession = await invoke<SessionSnapshot>("read_latest_project_session_snapshot", {
        projectName: activeProject.name,
      });

      return copyText(latestSession.content, setSessionCopyState);
    } catch (error: unknown) {
      console.error("Unable to read the latest project session markdown", error);

      return copyText(
        `# Fin de session — ${activeProject.name}\n\n${activeProject.sessionEnd}`,
        setSessionCopyState,
      );
    }
  }

  function selectProject(projectName: string) {
    setActiveProjectName(projectName);
    setContextCopyState("idle");
    setSessionCopyState("idle");
  }

  function updateCompanionSetting<Key extends keyof CompanionSettings>(
    key: Key,
    value: CompanionSettings[Key],
  ) {
    setCompanionSettings((currentSettings) => ({
      ...currentSettings,
      [key]: value,
    }));
  }

  async function updateProjectStatus(projectName: string, status: ProjectStatus) {
    const project = projects.find((value) => value.name === projectName);
    if (!project) return;

    setProjects((currentProjects) =>
      currentProjects.map((project) =>
        project.name === projectName ? { ...project, status } : project,
      ),
    );

    try {
      await invoke("update_project_status", {
        projectPath: project.locationLabel,
        status,
      });
    } catch (error: unknown) {
      console.error("Unable to update project status", error);
    }
  }

  async function notifyVesperion(
    status: "success" | "error",
    accessLabel: string,
  ) {
    try {
      await emitTo("vesperion", "vesperion-feedback", { status, accessLabel });
    } catch (error: unknown) {
      console.error("Unable to notify VESPΣRION", error);
    }
  }

  async function openProjectBranch(branch: ProjectGenealogyBranch) {
    if (!branch.path) return;

    try {
      await invoke("open_project_directory", {
        path: branch.path,
      });
      await notifyVesperion("success", branch.label);
    } catch {
      await notifyVesperion("error", branch.label);
    }
  }

  async function openProjectInVsCode() {
    if (!activeProject) return;

    try {
      await invoke("open_project_vscode", {
        projectName: activeProject.name,
      });
      await notifyVesperion("success", "VS Code");
    } catch (error: unknown) {
      console.error("Unable to open the project in VS Code", error);
      await notifyVesperion("error", "VS Code");
    }
  }

  async function openProjectGithub() {
    if (!activeProject) return;

    try {
      await invoke("open_project_github", {
        projectPath: activeProject.locationLabel,
      });
      await notifyVesperion("success", "GitHub");
    } catch (error: unknown) {
      console.error("Unable to open the project GitHub page", error);
      await notifyVesperion("error", "GitHub");
    }
  }

  async function openGlobalAccess(
    accessLabel: (typeof globalAccesses)[number]["label"],
  ) {
    try {
      await invoke("open_global_access", {
        accessLabel,
      });
      await notifyVesperion("success", accessLabel);
    } catch (error: unknown) {
      console.error(`Unable to open global access: ${accessLabel}`, error);
      await notifyVesperion("error", accessLabel);
    }
  }

  const activeProjectCount = projects.filter(
    (project) => project.status === "active",
  ).length;
  const pauseProjectCount = projects.filter(
    (project) => project.status === "paused",
  ).length;
  const conceptProjectCount = projects.filter(
    (project) => project.status === "concept",
  ).length;
  const archivedProjectCount = projects.filter(
    (project) => project.status === "archived",
  ).length;

  return (
    <main className="vesper-app">
      <CockpitLayout
        navigation={
          <div className="cockpit-navigation-content">
            <header className="cockpit-brand">
              <span>PH3YNYX.OS // COCKPIT</span>
              <div className="cockpit-brand-title">
                <img
                  className="cockpit-brand-icon"
                  src={vesperionIcon}
                  alt="Icône VESPΣRION"
                />
                <h1>VespΣr</h1>
              </div>
              <p>Context cockpit local-first</p>
            </header>

            <nav className="global-navigation" aria-labelledby="global-navigation-title">
              <h2 id="global-navigation-title">Accès globaux</h2>
              <ul>
                {globalAccesses.map((access) => (
                  <li key={access.label}>
                    <button
                      type="button"
                      onClick={() => openGlobalAccess(access.label)}
                    >
                      <span aria-hidden="true">{access.icon}</span>
                      <span>{access.label}</span>
                    </button>
                  </li>
                ))}
              </ul>
            </nav>

            <section className="project-status-legend" aria-labelledby="project-status-title">
              <h2 id="project-status-title">Projets</h2>
              <ul>
                <li><span className="active" />Actifs<strong>{activeProjectCount}</strong></li>
                <li><span className="paused" />En pause<strong>{pauseProjectCount}</strong></li>
                <li><span className="concept" />Concepts / futurs<strong>{conceptProjectCount}</strong></li>
                <li><span className="archived" />Archivés<strong>{archivedProjectCount}</strong></li>
              </ul>
            </section>

            <footer className="cockpit-system-state">
              <span>Local first</span>
              <span>Lecture seule</span>
            </footer>
          </div>
        }
        projects={
          <>
            <div className="cockpit-section-heading">
              <div>
                <span>Écosystème local</span>
                <h2 id="projects-title">Projets</h2>
              </div>
              <div className="project-heading-actions">
                <div className="companion-settings">
                  <button
                    className="companion-settings-trigger"
                    type="button"
                    aria-expanded={isCompanionPanelOpen}
                    onClick={() =>
                      setIsCompanionPanelOpen((isOpen) => !isOpen)
                    }
                  >
                    Compagnon
                  </button>

                  {isCompanionPanelOpen ? (
                    <div className="companion-settings-panel">
                      <label>
                        <span>Afficher</span>
                        <input
                          type="checkbox"
                          checked={companionSettings.visible}
                          onChange={(event) =>
                            updateCompanionSetting(
                              "visible",
                              event.currentTarget.checked,
                            )
                          }
                        />
                      </label>
                      <label>
                        <span>Ombre</span>
                        <input
                          type="checkbox"
                          checked={companionSettings.shadow}
                          onChange={(event) =>
                            updateCompanionSetting(
                              "shadow",
                              event.currentTarget.checked,
                            )
                          }
                        />
                      </label>
                    </div>
                  ) : null}
                </div>

                <span aria-label={`${projects.length} projets`}>
                  {projects.length}
                </span>
              </div>
            </div>

            <div className="project-list">
              {projectLoadState === "loading" ? (
                <p className="project-list-message">Lecture des dossiers locaux...</p>
              ) : null}
              {projectLoadState === "error" ? (
                <p className="project-list-message">
                  Impossible de lire {projectsRoot}.
                </p>
              ) : null}
              {projectLoadState === "ready" && projects.length === 0 ? (
                <p className="project-list-message">
                  Aucun dossier projet dans {projectsRoot}.
                </p>
              ) : null}
              {projects.map((project) => (
                <ProjectCard
                  key={project.name}
                  name={project.name}
                  icon={project.icon}
                  iconTone={project.iconTone}
                  status={project.status}
                  isActive={project.name === activeProject?.name}
                  onClick={() => selectProject(project.name)}
                  onStatusChange={(status) =>
                    updateProjectStatus(project.name, status)
                  }
                />
              ))}
            </div>
          </>
        }
        projectAccess={
          activeProject ? (
            <ProjectTree
              projectName={activeProject.name}
              projectPath={activeProject.locationLabel}
              branches={projectBranches}
              onOpenBranch={openProjectBranch}
            />
          ) : (
            <div className="project-tree-placeholder">
              {projectTreeState === "error"
                ? "Impossible de lire l'arbre du projet."
                : "Sélectionne un projet pour voir son arbre."}
            </div>
          )
        }
        contextPanel={
          activeProject ? (
            <ContextPanel
              name={activeProject.name}
              type={activeProject.type}
              status={activeProject.status}
              locationLabel={activeProject.locationLabel}
              lastSession={
                sessionPreviewState === "loading"
                  ? "Chargement..."
                  : latestSessionDate ?? activeProject.lastSession
              }
              summary={activeProject.summary}
              sessionEnd={
                sessionPreviewState === "loading"
                  ? "Chargement de la dernière fin de session..."
                  : latestSessionMarkdown ??
                    "Aucune fin de session lisible trouvée pour ce projet."
              }
              contextCopyState={contextCopyState}
              sessionCopyState={sessionCopyState}
              onOpenVsCode={openProjectInVsCode}
              onOpenGithub={openProjectGithub}
              onCopyContext={copyProjectContext}
              onCopySession={copySessionMarkdown}
            />
          ) : null
        }
      />
    </main>
  );
}

export default App;
