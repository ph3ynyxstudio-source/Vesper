import type { ProjectStatus } from "../ProjectCard/ProjectCard";
import "./ContextPanel.css";

type ContextPanelProps = {
  name: string;
  type: string;
  status: ProjectStatus;
  locationLabel: string;
  lastSession: string;
  summary: string;
  sessionEnd: string;
  contextCopyState?: "idle" | "copied" | "error";
  sessionCopyState?: "idle" | "copied" | "error";
  onCopyContext?: () => void;
  onCopySession?: () => void;
};

const statusLabels: Record<ProjectStatus, string> = {
  active: "Actif",
  paused: "En pause",
  concept: "Concept",
  archived: "Archivé",
};

export function ContextPanel({
  name,
  type,
  status,
  locationLabel,
  lastSession,
  summary,
  sessionEnd,
  contextCopyState = "idle",
  sessionCopyState = "idle",
  onCopyContext,
  onCopySession,
}: ContextPanelProps) {
  const contextCopyLabel = {
    idle: "Copier le contexte",
    copied: "Contexte copié",
    error: "Copie impossible",
  }[contextCopyState];
  const sessionCopyLabel = {
    idle: "Copier le markdown",
    copied: "Markdown copié",
    error: "Copie impossible",
  }[sessionCopyState];

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
          <dt>Emplacement</dt>
          <dd>{locationLabel}</dd>
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

      <button
        className="context-panel-copy"
        type="button"
        onClick={onCopyContext}
        disabled={!onCopyContext}
        data-state={contextCopyState}
      >
        {contextCopyLabel}
      </button>

      <section className="context-panel-section context-panel-session">
        <div className="context-panel-session-heading">
          <span>Fin de session chr0</span>
          <small>Markdown</small>
        </div>
        <pre>{sessionEnd}</pre>
        <button
          className="context-panel-session-copy"
          type="button"
          onClick={onCopySession}
          disabled={!onCopySession}
          data-state={sessionCopyState}
        >
          {sessionCopyLabel}
        </button>
      </section>
      <span className="context-panel-copy-status" aria-live="polite">
        {contextCopyState === "copied" ? "Le contexte est copié. " : ""}
        {sessionCopyState === "copied" ? "Le markdown est copié." : ""}
        {contextCopyState === "error" || sessionCopyState === "error"
          ? "Le presse-papiers n’est pas accessible."
          : ""}
      </span>
    </aside>
  );
}
