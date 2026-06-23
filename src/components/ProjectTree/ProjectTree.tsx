import "./ProjectTree.css";

export type ProjectQuickAccess = {
  id: string;
  label: string;
  description: string;
  icon: string;
  destinationId?: KnownDestinationId;
};

export type KnownDestinationId =
  | "lunarmood_github"
  | "lunarmood_root"
  | "lunarmood_assets"
  | "lunarmood_docs"
  | "lunarmood_sessions";

type ProjectTreeProps = {
  projectName: string;
  accesses: ProjectQuickAccess[];
  onAccess?: (access: ProjectQuickAccess) => void;
};

export function ProjectTree({ projectName, accesses, onAccess }: ProjectTreeProps) {
  return (
    <section className="project-tree" aria-labelledby="project-access-title">
      <div className="project-tree-heading">
        <span>Entrer dans le projet</span>
        <strong>{projectName}</strong>
      </div>

      <h2 id="project-access-title">Accès rapides</h2>
      <p className="project-tree-intro">
        Ouvrir directement les outils et emplacements utiles.
      </p>

      <ul className="project-tree-actions">
        {accesses.map((access) => (
          <li key={access.id}>
            <button
              type="button"
              onClick={() => onAccess?.(access)}
              disabled={!access.destinationId}
            >
              <span className="project-tree-action-icon" aria-hidden="true">
                {access.icon}
              </span>
              <span>
                <strong>{access.label}</strong>
                <small>
                  {access.destinationId ? access.description : "Bientôt"}
                </small>
              </span>
              <span className="project-tree-action-arrow" aria-hidden="true">
                ↗
              </span>
            </button>
          </li>
        ))}
      </ul>
    </section>
  );
}
