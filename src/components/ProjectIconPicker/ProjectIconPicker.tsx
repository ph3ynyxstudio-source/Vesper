import { useEffect } from "react";
import { createPortal } from "react-dom";
import { ProjectLibraryIcon } from "./ProjectLibraryIcon";
import { projectIconOptions } from "./project-icons";
import "./ProjectIconPicker.css";

type ProjectIconPickerProps = {
  projectName: string;
  selectedIconId?: string;
  error?: string;
  onClose: () => void;
  onSelect: (iconId: string) => void;
};

export function ProjectIconPicker({
  projectName,
  selectedIconId,
  error,
  onClose,
  onSelect,
}: ProjectIconPickerProps) {
  useEffect(() => {
    function closeOnEscape(event: KeyboardEvent) {
      if (event.key === "Escape") onClose();
    }

    document.addEventListener("keydown", closeOnEscape);
    return () => document.removeEventListener("keydown", closeOnEscape);
  }, [onClose]);

  return createPortal(
    <div
      className="project-icon-picker-backdrop"
      onPointerDown={(event) => {
        if (event.target === event.currentTarget) onClose();
      }}
    >
      <section
        className="project-icon-picker"
        role="dialog"
        aria-modal="true"
        aria-labelledby="project-icon-picker-title"
      >
        <header>
          <div>
            <span>Bibliothèque locale</span>
            <h2 id="project-icon-picker-title">Icône de {projectName}</h2>
          </div>
          <button type="button" onClick={onClose} aria-label="Fermer" autoFocus>
            ×
          </button>
        </header>

        <div className="project-icon-picker-grid">
          {projectIconOptions.map((icon) => (
            <button
              className={icon.id === selectedIconId ? "selected" : ""}
              type="button"
              key={icon.id}
              title={icon.label}
              aria-label={`Utiliser l’icône ${icon.label}`}
              aria-pressed={icon.id === selectedIconId}
              onClick={() => onSelect(icon.id)}
            >
              <ProjectLibraryIcon iconId={icon.id} />
              <span>{icon.label}</span>
            </button>
          ))}
        </div>

        {error ? <p role="alert">{error}</p> : null}
        <small>{projectIconOptions.length} icônes SVG intégrées</small>
      </section>
    </div>,
    document.body,
  );
}
