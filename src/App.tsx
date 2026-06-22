import { useState } from "react";
import { ContextPanel } from "./components/ContextPanel/ContextPanel";
import {
  ProjectCard,
  type ProjectStatus,
} from "./components/ProjectCard/ProjectCard";
import {
  ProjectTree,
  type ProjectTreeSection,
} from "./components/ProjectTree/ProjectTree";
import "./App.css";

type MockProject = {
  name: string;
  icon: string;
  type: string;
  description: string;
  status: ProjectStatus;
  lastSession: string;
  path: string;
  summary: string;
  sessionEnd: string;
  tree: ProjectTreeSection[];
};

const commonTree: ProjectTreeSection[] = [
  { name: "Docs", items: ["README.md", "ARCHITECTURE.md", "CONTEXT.md"] },
  { name: "Assets", items: ["identity", "references", "icons"] },
  { name: "Source", items: ["components", "theme", "services"] },
  { name: "Sessions", items: ["2026-06-22.md", "2026-06-21.md"] },
  { name: "Exports", items: ["latest-summary.md", "project-map.json"] },
];

const projects: MockProject[] = [
  {
    name: "Lun△rMood",
    icon: "☾",
    type: "Flutter",
    description: "Journal émotionnel local-first.",
    status: "active",
    lastSession: "2026-06-21",
    path: "C:\\Ph3yNyx.OS\\Devs\\LunarMood",
    summary: "Suivi local des humeurs, cycles et notes quotidiennes.",
    sessionEnd: "Navigation principale validée. Prochaine étape : consolider les vues de synthèse.",
    tree: commonTree,
  },
  {
    name: "chr0",
    icon: "↻",
    type: "Tauri",
    description: "Mémoire projet et synthèses.",
    status: "active",
    lastSession: "2026-06-22",
    path: "C:\\Ph3yNyx.OS\\Devs\\chr0",
    summary: "Capture les sessions de travail et maintient la mémoire des projets PH3YNYX.OS.",
    sessionEnd: "Pipeline de synthèse stabilisé. Les sorties restent validées manuellement.",
    tree: commonTree,
  },
  {
    name: "VespΣr",
    icon: "✦",
    type: "Tauri",
    description: "Cockpit de contexte.",
    status: "concept",
    lastSession: "Aujourd'hui",
    path: "C:\\Ph3yNyx.OS\\Devs\\Vesper",
    summary: "Cockpit local-first pour retrouver les projets, leur documentation et leur contexte.",
    sessionEnd: "Structure visuelle initiale en cours. Backend volontairement hors périmètre.",
    tree: commonTree,
  },
];

function App() {
  const [activeProjectName, setActiveProjectName] = useState(projects[0].name);
  const activeProject =
    projects.find((project) => project.name === activeProjectName) ?? projects[0];

  return (
    <main className="vesper-app">
      <header className="vesper-header">
        <p>PH3YNYX.OS</p>
        <h1>VespΣr</h1>
        <span>Context Cockpit</span>
      </header>

      <div className="cockpit-layout">
        <aside className="project-sidebar" aria-labelledby="projects-title">
          <div className="cockpit-section-heading">
            <h2 id="projects-title">Projets</h2>
            <span>{projects.length}</span>
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
                onClick={() => setActiveProjectName(project.name)}
              />
            ))}
          </div>
        </aside>

        <ProjectTree
          projectName={activeProject.name}
          sections={activeProject.tree}
        />

        <ContextPanel
          name={activeProject.name}
          type={activeProject.type}
          status={activeProject.status}
          path={activeProject.path}
          lastSession={activeProject.lastSession}
          summary={activeProject.summary}
          sessionEnd={activeProject.sessionEnd}
        />
      </div>
    </main>
  );
}

export default App;
