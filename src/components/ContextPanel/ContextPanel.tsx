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
  sessionSourceLabel?: string;
  sessionSourceState?: "idle" | "selecting" | "error";
  sessionSourceError?: string;
  onOpenVsCode?: () => void;
  onOpenGithub?: () => void;
  onCopyContext?: () => void;
  onCopySession?: () => void;
  onSelectSessionDirectory?: () => void;
  onOpenChronos?: () => void;
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
  sessionSourceLabel,
  sessionSourceState = "idle",
  sessionSourceError,
  onOpenVsCode,
  onOpenGithub,
  onCopyContext,
  onCopySession,
  onSelectSessionDirectory,
  onOpenChronos,
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
        <div className="context-panel-last-session">
          <dt>Dernière session</dt>
          <dd>{lastSession}</dd>
        </div>
      </dl>

      <section className="context-panel-section">
        <h3>Résumé contexte</h3>
        <p>{summary}</p>
      </section>

      <div className="context-panel-actions">
        <button
          className="context-panel-action-button"
          type="button"
          onClick={onOpenVsCode}
          disabled={!onOpenVsCode}
        >
          Ouvrir dans VS Code
        </button>
        <button
          className="context-panel-action-button"
          type="button"
          onClick={onOpenGithub}
          disabled={!onOpenGithub}
        >
          Ouvrir GitHub
        </button>
      </div>

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
          <span>Fin de session</span>
          <div>
            <small>Markdown</small>
            <button
              className="context-panel-session-launch"
              type="button"
              onClick={onOpenChronos}
              disabled={!onOpenChronos}
            >
              Ouvrir Chr0
            </button>
          </div>
        </div>
        <div className="context-panel-session-source">
          <button
            type="button"
            onClick={onSelectSessionDirectory}
            disabled={!onSelectSessionDirectory || sessionSourceState === "selecting"}
          >
            {sessionSourceState === "selecting"
              ? "Ouverture..."
              : "Choisir le dossier Chr0"}
          </button>
          <small title={sessionSourceLabel}>
            {sessionSourceLabel ?? "Association automatique actuelle"}
          </small>
        </div>
        {sessionSourceError ? (
          <p className="context-panel-session-source-error" role="alert">
            {sessionSourceError}
          </p>
        ) : null}
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
