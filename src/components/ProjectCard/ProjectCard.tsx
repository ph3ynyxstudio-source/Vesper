import "./ProjectCard.css";

export type ProjectStatus = "active" | "pause" | "concept" | "archived";

type ProjectCardProps = {
  name: string;
  icon?: string;
  type: string;
  description: string;
  status: ProjectStatus;
  lastSession?: string;
  isActive?: boolean;
  onClick?: () => void;
};

const statusLabels: Record<ProjectStatus, string> = {
  active: "Actif",
  pause: "En pause",
  concept: "Concept",
  archived: "Archivé",
};

export function ProjectCard({
  name,
  icon = "□",
  type,
  description,
  status,
  lastSession,
  isActive = false,
  onClick,
}: ProjectCardProps) {
  return (
    <button
      className={`project-card ${isActive ? "active" : ""}`}
      type="button"
      onClick={onClick}
      aria-pressed={isActive}
    >
      <div className="project-card-header">
        <div className="project-card-icon" aria-hidden="true">
          {icon}
        </div>

        <div className="project-card-title-group">
          <h3>{name}</h3>
          <span className="project-card-type">{type}</span>
        </div>

        <span className={`project-card-status ${status}`}>
          {statusLabels[status]}
        </span>
      </div>

      <p className="project-card-description">{description}</p>

      <div className="project-card-footer">
        <span>Dernière session</span>
        <strong>{lastSession ?? "Non disponible"}</strong>
      </div>
    </button>
  );
}
