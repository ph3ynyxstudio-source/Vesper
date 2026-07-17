const iconUrls = import.meta.glob("../../assets/project-icons/*.svg", {
  eager: true,
  query: "?url",
  import: "default",
}) as Record<string, string>;

const iconLabels: Record<string, string> = {
  architecture: "Architecture",
  "archive-box": "Archives",
  "book-open": "Livre",
  brain: "Cerveau",
  briefcase: "Mallette",
  clock: "Horloge",
  cockpit: "Cockpit",
  "code-editor": "Code",
  context: "Contexte",
  "crystal-ball": "Cristal",
  desktop: "Bureau",
  "document-stack": "Documents",
  filesystem: "Fichiers",
  "folder-open": "Dossier",
  gear: "Réglages",
  "git-repository": "Git",
  "glowing-star": "Étoile",
  "hammer-wrench": "Outils",
  hourglass: "Sablier",
  idea: "Idée",
  laptop: "Portable",
  markdown: "Markdown",
  memory: "Mémoire",
  moon: "Lune",
  "network-globe": "Réseau",
  package: "Paquet",
  paintbrush: "Pinceau",
  palette: "Palette",
  "project-tree": "Arbre",
  robot: "Robot",
  session: "Session",
  sparkles: "Étincelles",
  terminal: "Terminal",
  vesperion: "Vesperion",
};

export type ProjectIconOption = {
  id: string;
  label: string;
  url: string;
};

export const projectIconOptions = Object.entries(iconUrls)
  .map(([path, url]) => {
    const id = path.split("/").pop()?.replace(/\.svg$/i, "") ?? path;

    return {
      id,
      label: iconLabels[id] ?? id,
      url,
    };
  })
  .sort((left, right) => left.label.localeCompare(right.label, "fr"));

export const projectIconById = new Map(
  projectIconOptions.map((icon) => [icon.id, icon]),
);

export function isProjectIconId(value?: string): value is string {
  return typeof value === "string" && projectIconById.has(value);
}
