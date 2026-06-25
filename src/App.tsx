import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { emitTo } from "@tauri-apps/api/event";
import { CockpitLayout } from "./components/CockpitLayout/CockpitLayout";
import { ContextPanel } from "./components/ContextPanel/ContextPanel";
import {
  ProjectCard,
  type ProjectStatus,
} from "./components/ProjectCard/ProjectCard";
import {
  ProjectTree,
  type ProjectGenealogyBranch,
} from "./components/ProjectTree/ProjectTree";
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

type LocalProject = {
  name: string;
  icon: string;
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

const globalAccesses = [
  { label: "Obsidian", icon: "◈" },
  { label: "GitHub Desktop", icon: "◇" },
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

function getProjectIcon(name: string) {
  if (name.toLowerCase().includes("vesp")) return "✦";
  return "□";
}

function toLocalProject(directory: ProjectDirectory): LocalProject {
  const lastSession = formatModifiedDate(directory.modified_at_epoch_seconds);

  return {
    name: directory.name,
    icon: getProjectIcon(directory.name),
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
    localStorage.setItem(
      companionSettingsStorageKey,
      JSON.stringify(companionSettings),
    );

    void invoke("apply_companion_settings", {
      settings: companionSettings,
    }).catch((error: unknown) => {
      console.error("Unable to apply VESPΣRION settings", error);
    });
  }, [companionSettings]);

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

  function copyProjectContext() {
    if (!activeProject) return;

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

  function copySessionMarkdown() {
    if (!activeProject) return;

    return copyText(
      `# Fin de session — ${activeProject.name}\n\n${activeProject.sessionEnd}`,
      setSessionCopyState,
    );
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
              <h1>VespΣr</h1>
              <p>Context cockpit local-first</p>
            </header>

            <nav className="global-navigation" aria-labelledby="global-navigation-title">
              <h2 id="global-navigation-title">Accès globaux</h2>
              <ul>
                {globalAccesses.map((access) => (
                  <li key={access.label}>
                    <button type="button" disabled>
                      <span aria-hidden="true">{access.icon}</span>
                      <span>{access.label}</span>
                      <small>Bientôt</small>
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
              lastSession={activeProject.lastSession}
              summary={activeProject.summary}
              sessionEnd={activeProject.sessionEnd}
              contextCopyState={contextCopyState}
              sessionCopyState={sessionCopyState}
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
