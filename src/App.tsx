import { useState } from "react";
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
  type ProjectQuickAccess,
} from "./components/ProjectTree/ProjectTree";
import "./App.css";

type CopyState = "idle" | "copied" | "error";

type MockProject = {
  name: string;
  icon: string;
  type: string;
  description: string;
  status: ProjectStatus;
  lastSession: string;
  locationLabel: string;
  summary: string;
  sessionEnd: string;
  accesses: ProjectQuickAccess[];
};

const unavailableProjectAccesses: ProjectQuickAccess[] = [
  { id: "vscode", label: "Ouvrir dans VS Code", description: "Workspace du projet", icon: "⌁" },
  { id: "github", label: "Repository GitHub", description: "Dépôt dans le navigateur", icon: "◇" },
  { id: "terminal", label: "Ouvrir le terminal", description: "Dossier courant", icon: ">_" },
  { id: "root", label: "Dossier racine", description: "Explorateur local", icon: "□" },
  { id: "assets", label: "Dossier assets/", description: "Identité et ressources", icon: "▧" },
  { id: "docs", label: "Dossier docs/", description: "Documentation du projet", icon: "≡" },
  { id: "sessions", label: "Sessions chr0", description: "Mémoire récente", icon: "↻" },
  { id: "exports", label: "Exports", description: "Synthèses et sorties", icon: "⇱" },
];

const lunarmoodAccesses: ProjectQuickAccess[] = [
  { id: "vscode", label: "Ouvrir dans VS Code", description: "Workspace du projet", icon: "⌁" },
  { id: "github", label: "Repository GitHub", description: "Dépôt dans le navigateur", icon: "◇", destinationId: "lunarmood_github" },
  { id: "terminal", label: "Ouvrir le terminal", description: "Dossier courant", icon: ">_" },
  { id: "root", label: "Dossier racine", description: "Explorateur local", icon: "□", destinationId: "lunarmood_root" },
  { id: "assets", label: "Dossier assets/", description: "Identité et ressources", icon: "▧", destinationId: "lunarmood_assets" },
  { id: "docs", label: "Dossier docs/", description: "Documentation du projet", icon: "≡", destinationId: "lunarmood_docs" },
  { id: "sessions", label: "Sessions chr0", description: "Mémoire récente", icon: "↻", destinationId: "lunarmood_sessions" },
  { id: "exports", label: "Exports", description: "Synthèses et sorties", icon: "⇱" },
];

const globalAccesses = [
  { label: "Obsidian", icon: "◈" },
  { label: "GitHub Desktop", icon: "◇" },
  { label: "Explorateur", icon: "□" },
  { label: "VS Code", icon: "⌁" },
  { label: "Terminal", icon: ">_" },
] as const;

const projects: MockProject[] = [
  {
    name: "Lun△rMood",
    icon: "☾",
    type: "Flutter",
    description: "Journal émotionnel local-first.",
    status: "active",
    lastSession: "2026-06-21",
    locationLabel: "Destination locale sécurisée",
    summary: "Suivi local des humeurs, cycles et notes quotidiennes.",
    sessionEnd: "Navigation principale validée. Prochaine étape : consolider les vues de synthèse.",
    accesses: lunarmoodAccesses,
  },
  {
    name: "chr0",
    icon: "↻",
    type: "Tauri",
    description: "Mémoire projet et synthèses.",
    status: "active",
    lastSession: "2026-06-22",
    locationLabel: "Non configuré",
    summary: "Capture les sessions de travail et maintient la mémoire des projets PH3YNYX.OS.",
    sessionEnd: "Pipeline de synthèse stabilisé. Les sorties restent validées manuellement.",
    accesses: unavailableProjectAccesses,
  },
  {
    name: "VespΣr",
    icon: "✦",
    type: "Tauri",
    description: "Cockpit de contexte.",
    status: "concept",
    lastSession: "Aujourd'hui",
    locationLabel: "Non configuré",
    summary: "Cockpit local-first pour retrouver les projets, leur documentation et leur contexte.",
    sessionEnd: "Structure visuelle initiale en cours. Backend volontairement hors périmètre.",
    accesses: unavailableProjectAccesses,
  },
];

function App() {
  const [activeProjectName, setActiveProjectName] = useState(projects[0].name);
  const [contextCopyState, setContextCopyState] = useState<CopyState>("idle");
  const [sessionCopyState, setSessionCopyState] = useState<CopyState>("idle");
  const activeProject =
    projects.find((project) => project.name === activeProjectName) ?? projects[0];

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

  async function openProjectAccess(access: ProjectQuickAccess) {
    if (!access.destinationId) return;

    try {
      await invoke("open_known_destination", {
        destinationId: access.destinationId,
      });
      await notifyVesperion("success", access.label);
    } catch {
      await notifyVesperion("error", access.label);
    }
  }

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
                <li><span className="active" />Actifs<strong>2</strong></li>
                <li><span className="pause" />En pause<strong>0</strong></li>
                <li><span className="concept" />Concepts / futurs<strong>1</strong></li>
                <li><span className="archived" />Archivés<strong>0</strong></li>
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
              <span aria-label={`${projects.length} projets`}>
                {projects.length}
              </span>
            </div>

            <div className="project-list">
              {projects.map((project) => (
                <ProjectCard
                  key={project.name}
                  name={project.name}
                  icon={project.icon}
                  type={project.type}
                  description={project.description}
                  status={project.status}
                  lastSession={project.lastSession}
                  isActive={project.name === activeProject.name}
                  onClick={() => selectProject(project.name)}
                />
              ))}
            </div>
          </>
        }
        projectAccess={
          <ProjectTree
            projectName={activeProject.name}
            accesses={activeProject.accesses}
            onAccess={openProjectAccess}
          />
        }
        contextPanel={
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
        }
      />
    </main>
  );
}

export default App;
