import type { ProjectStatus } from "../ProjectCard/ProjectCard";
import "./ContextPanel.css";

type ContextPanelProps = {
  name: string;
  type: string;
  status: ProjectStatus;
  path: string;
  lastSession: string;
  summary: string;
  sessionEnd: string;
};

const statusLabels: Record<ProjectStatus, string> = {
  active: "Actif",
  pause: "En pause",
  concept: "Concept",
  archived: "Archivé",
};

export function ContextPanel({
  name,
  type,
  status,
  path,
  lastSession,
  summary,
  sessionEnd,
}: ContextPanelProps) {
  return (
    <aside className="context-panel" aria-labelledby="context-panel-title">
      <div className="context-panel-heading">
        <span>Contexte</span>
        <span className="context-panel-local">Local</span>
      </div>

      <h2 id="context-panel-title">{name}</h2>

      <dl className="context-panel-metadata">
        <div>
          <dt>Type</dt>
          <dd>{type}</dd>
        </div>
        <div>
          <dt>Statut</dt>
          <dd className={`context-panel-status ${status}`}>
            {statusLabels[status]}
          </dd>
        </div>
        <div className="context-panel-path">
          <dt>Chemin local</dt>
          <dd>{path}</dd>
        </div>
        <div>
          <dt>Dernière session</dt>
          <dd>{lastSession}</dd>
        </div>
      </dl>

      <section className="context-panel-section">
        <h3>Résumé contexte</h3>
        <p>{summary}</p>
      </section>

      <section className="context-panel-section context-panel-session">
        <h3>Fin de session chr0</h3>
        <p>{sessionEnd}</p>
      </section>

      <button className="context-panel-copy" type="button">
        Copier le contexte
      </button>
    </aside>
  );
}
