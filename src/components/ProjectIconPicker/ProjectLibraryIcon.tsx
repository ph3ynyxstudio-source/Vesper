import type { CSSProperties } from "react";
import { projectIconById } from "./project-icons";

export function ProjectLibraryIcon({ iconId }: { iconId: string }) {
  const icon = projectIconById.get(iconId);
  if (!icon) return null;

  const style = {
    "--project-icon-mask": `url("${icon.url}")`,
  } as CSSProperties;

  return <span className="project-library-icon" style={style} aria-hidden="true" />;
}
