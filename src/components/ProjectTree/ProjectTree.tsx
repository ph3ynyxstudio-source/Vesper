import "./ProjectTree.css";

export type ProjectBranchId = "docs" | "assets" | "features" | "archives";

export type ProjectGenealogyBranch = {
  id: ProjectBranchId;
  label: string;
  path?: string;
  exists: boolean;
};

type ProjectTreeProps = {
  projectName: string;
  projectPath: string;
  branches: ProjectGenealogyBranch[];
  onOpenBranch?: (branch: ProjectGenealogyBranch) => void;
  onOpenPlum3?: () => void;
};

const branchMeta: Record<
  ProjectBranchId,
  {
    title: string;
    subtitle: string;
    tone: "cyan" | "turquoise" | "magenta" | "gold";
  }
> = {
  docs: {
    title: "Docs",
    subtitle: "Documentation du projet",
    tone: "cyan",
  },
  assets: {
    title: "Assets",
    subtitle: "Ressources visuelles et médias",
    tone: "turquoise",
  },
  features: {
    title: "Features",
    subtitle: "Fonctionnalités et développement",
    tone: "magenta",
  },
  archives: {
    title: "Archives",
    subtitle: "Archives et anciennes versions",
    tone: "gold",
  },
};

function RootIcon() {
  return (
    <svg viewBox="0 0 80 80" role="img" aria-label="">
      <defs>
        <radialGradient id="rootGlow" cx="50%" cy="50%" r="55%">
          <stop offset="0%" stopColor="#ffffff" />
          <stop offset="42%" stopColor="#50d2ff" />
          <stop offset="100%" stopColor="#e057ff" />
        </radialGradient>
        <filter id="rootNeon" x="-80%" y="-80%" width="260%" height="260%">
          <feGaussianBlur stdDeviation="3.5" result="blur" />
          <feMerge>
            <feMergeNode in="blur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>
      <path
        d="M40 5 47 30 72 40 47 50 40 75 33 50 8 40 33 30Z"
        fill="none"
        stroke="url(#rootGlow)"
        strokeWidth="4"
        filter="url(#rootNeon)"
      />
      <path
        d="M40 22 44 36 58 40 44 44 40 58 36 44 22 40 36 36Z"
        fill="rgba(80, 210, 255, 0.12)"
        stroke="#50d2ff"
        strokeWidth="2"
      />
    </svg>
  );
}

function BranchIcon({ id }: { id: ProjectBranchId }) {
  if (id === "docs") {
    return (
      <svg viewBox="0 0 64 64" role="img" aria-label="">
        <defs>
          <linearGradient id="docsGradient" x1="8" y1="8" x2="56" y2="56">
            <stop stopColor="#50d2ff" />
            <stop offset="1" stopColor="#16f5ff" />
          </linearGradient>
        </defs>
        <path d="M18 8h22l10 10v38H18Z" fill="none" stroke="url(#docsGradient)" strokeWidth="4" />
        <path d="M40 8v12h12" fill="none" stroke="#b8fbff" strokeWidth="3" />
        <path d="M25 30h18M25 39h14M25 48h20" stroke="#50d2ff" strokeWidth="3" strokeLinecap="round" />
      </svg>
    );
  }

  if (id === "assets") {
    return (
      <svg viewBox="0 0 64 64" role="img" aria-label="">
        <defs>
          <linearGradient id="assetsGradient" x1="8" y1="56" x2="56" y2="8">
            <stop stopColor="#18f5d2" />
            <stop offset="1" stopColor="#50d2ff" />
          </linearGradient>
        </defs>
        <rect x="10" y="12" width="44" height="40" rx="5" fill="none" stroke="url(#assetsGradient)" strokeWidth="4" />
        <circle cx="24" cy="25" r="5" fill="none" stroke="#bafff5" strokeWidth="3" />
        <path d="M14 47 28 33l9 9 6-6 9 11" fill="none" stroke="#18f5d2" strokeWidth="4" strokeLinejoin="round" />
      </svg>
    );
  }

  if (id === "features") {
    return (
      <svg viewBox="0 0 64 64" role="img" aria-label="">
        <defs>
          <linearGradient id="featuresGradient" x1="22" y1="16" x2="44" y2="48">
            <stop stopColor="#e057ff" />
            <stop offset="1" stopColor="#ff1ee8" />
          </linearGradient>
        </defs>
        <path
          d="m25 17 15 15-15 15"
          fill="none"
          stroke="url(#featuresGradient)"
          strokeWidth="6"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      </svg>
    );
  }

  return (
    <svg viewBox="0 0 64 64" role="img" aria-label="">
      <defs>
        <linearGradient id="archivesGradient" x1="14" y1="10" x2="50" y2="56">
          <stop stopColor="#ffd447" />
          <stop offset="1" stopColor="#ff8c1a" />
        </linearGradient>
      </defs>
      <path d="m16 17 16-7 16 7-16 7Z" fill="none" stroke="url(#archivesGradient)" strokeWidth="4" strokeLinejoin="round" />
      <path d="m16 31 16-7 16 7-16 7Z" fill="none" stroke="#ffb21a" strokeWidth="4" strokeLinejoin="round" />
      <path d="m16 45 16-7 16 7-16 7Z" fill="none" stroke="#ffd447" strokeWidth="4" strokeLinejoin="round" />
    </svg>
  );
}

function getBranch(branches: ProjectGenealogyBranch[], id: ProjectBranchId) {
  return (
    branches.find((branch) => branch.id === id) ?? {
      id,
      label: branchMeta[id].title,
      exists: false,
    }
  );
}

export function ProjectTree({
  projectName,
  projectPath,
  branches,
  onOpenBranch,
  onOpenPlum3,
}: ProjectTreeProps) {
  const orderedBranches: ProjectBranchId[] = [
    "docs",
    "assets",
    "features",
    "archives",
  ];

  return (
    <section className="project-tree" aria-labelledby="project-tree-title">
      <div className="project-tree-heading">
        <div>
          <span>Arbre généalogique du projet</span>
          <h2 id="project-tree-title">{projectName}</h2>
          <p>{projectPath}</p>
        </div>
        <strong>Local</strong>
      </div>

      <div className="project-genealogy-map">
        <div className="project-tree-root-node">
          <span className="project-tree-root-orb">
            <RootIcon />
          </span>
          <strong>{projectName}</strong>
          <small>Racine du projet</small>
        </div>

        <svg
          className="project-genealogy-lines"
          viewBox="0 0 1000 92"
          preserveAspectRatio="none"
          aria-hidden="true"
        >
          <defs>
            <linearGradient id="treeLineGradient" x1="150" y1="0" x2="850" y2="0" gradientUnits="userSpaceOnUse">
              <stop stopColor="#50d2ff" />
              <stop offset="0.48" stopColor="#7c4dff" />
              <stop offset="0.72" stopColor="#ff1ee8" />
              <stop offset="1" stopColor="#ffb21a" />
            </linearGradient>
            <filter id="treeLineGlow" x="-20%" y="-80%" width="140%" height="260%">
              <feGaussianBlur stdDeviation="3" result="blur" />
              <feMerge>
                <feMergeNode in="blur" />
                <feMergeNode in="SourceGraphic" />
              </feMerge>
            </filter>
          </defs>
          <path d="M500 0v28" className="tree-line-main" />
          <path d="M145 72v-24c0-12 10-20 22-20h666c12 0 22 8 22 20v24" className="tree-line-branch" />
          <path d="M333 28v44" className="tree-line-branch" />
          <path d="M667 28v44" className="tree-line-branch" />
        </svg>

        <div className="project-tree-branches">
          {orderedBranches.map((id) => {
            const branch = getBranch(branches, id);
            const meta = branchMeta[id];

            return (
              <button
                className={`project-tree-node tone-${meta.tone}`}
                type="button"
                key={id}
                disabled={!branch.exists || !branch.path}
                aria-label={
                  branch.exists && branch.path
                    ? `Ouvrir ${meta.title}`
                    : `${meta.title} indisponible`
                }
                onClick={() => onOpenBranch?.(branch)}
              >
                <span className="project-tree-node-icon">
                  <BranchIcon id={id} />
                </span>
              </button>
            );
          })}
        </div>

        <button
          className="project-tree-plum3-launch"
          type="button"
          onClick={onOpenPlum3}
          disabled={!onOpenPlum3}
        >
          Plum3
        </button>
      </div>
    </section>
  );
}
