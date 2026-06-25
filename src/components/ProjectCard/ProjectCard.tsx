import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import "./ProjectCard.css";

export type ProjectStatus = "active" | "paused" | "concept" | "archived";

type ProjectCardProps = {
  name: string;
  icon?: string;
  status: ProjectStatus;
  isActive?: boolean;
  onClick?: () => void;
  onStatusChange?: (status: ProjectStatus) => void;
};

const statusOptions = [
  { value: "active", label: "Active", marker: "●" },
  { value: "paused", label: "Paused", marker: "●" },
  { value: "concept", label: "Concept", marker: "●" },
  { value: "archived", label: "Archived", marker: "●" },
] as const satisfies readonly {
  value: ProjectStatus;
  label: string;
  marker: string;
}[];

const statusLabels: Record<ProjectStatus, string> = {
  active: "Active",
  paused: "Paused",
  concept: "Concept",
  archived: "Archived",
};

const popoverMargin = 8;
const popoverWidth = 190;
const estimatedPopoverHeight = 176;

export function ProjectCard({
  name,
  icon = "□",
  status,
  isActive = false,
  onClick,
  onStatusChange,
}: ProjectCardProps) {
  const [isStatusMenuOpen, setIsStatusMenuOpen] = useState(false);
  const [menuPosition, setMenuPosition] = useState({ top: 0, left: 0 });
  const statusButtonRef = useRef<HTMLButtonElement | null>(null);
  const statusMenuRef = useRef<HTMLDivElement | null>(null);

  function positionStatusMenu(menuHeight = estimatedPopoverHeight) {
    const trigger = statusButtonRef.current;

    if (!trigger) {
      return;
    }

    const triggerRect = trigger.getBoundingClientRect();
    const viewportWidth = window.innerWidth;
    const viewportHeight = window.innerHeight;
    const preferredTop = triggerRect.bottom + popoverMargin;
    const opensAbove = preferredTop + menuHeight > viewportHeight - popoverMargin;
    const top = opensAbove
      ? Math.max(popoverMargin, triggerRect.top - menuHeight - popoverMargin)
      : preferredTop;
    const left = Math.min(
      Math.max(popoverMargin, triggerRect.right - popoverWidth),
      viewportWidth - popoverWidth - popoverMargin,
    );

    setMenuPosition({ top, left });
  }

  useLayoutEffect(() => {
    if (!isStatusMenuOpen) {
      return;
    }

    positionStatusMenu();
    requestAnimationFrame(() => {
      const menuHeight = statusMenuRef.current?.offsetHeight;

      if (menuHeight) {
        positionStatusMenu(menuHeight);
      }
    });
  }, [isStatusMenuOpen]);

  useEffect(() => {
    if (!isStatusMenuOpen) {
      return;
    }

    function closeOnOutsidePointer(event: PointerEvent) {
      const target = event.target as Node;

      if (
        statusButtonRef.current?.contains(target) ||
        statusMenuRef.current?.contains(target)
      ) {
        return;
      }

      setIsStatusMenuOpen(false);
    }

    function closeOnEscape(event: KeyboardEvent) {
      if (event.key === "Escape") {
        setIsStatusMenuOpen(false);
      }
    }

    function reposition() {
      const menuHeight = statusMenuRef.current?.offsetHeight;
      positionStatusMenu(menuHeight);
    }

    document.addEventListener("pointerdown", closeOnOutsidePointer);
    document.addEventListener("keydown", closeOnEscape);
    window.addEventListener("resize", reposition);
    window.addEventListener("scroll", reposition, true);

    return () => {
      document.removeEventListener("pointerdown", closeOnOutsidePointer);
      document.removeEventListener("keydown", closeOnEscape);
      window.removeEventListener("resize", reposition);
      window.removeEventListener("scroll", reposition, true);
    };
  }, [isStatusMenuOpen]);

  function selectStatus(nextStatus: ProjectStatus) {
    setIsStatusMenuOpen(false);
    onStatusChange?.(nextStatus);
  }

  return (
    <div
      className={`project-card ${isActive ? "active" : ""}`}
    >
      <button
        className="project-card-main"
        type="button"
        onClick={onClick}
        aria-pressed={isActive}
      >
        <div className="project-card-icon" aria-hidden="true">
          {icon}
        </div>

        <div className="project-card-title-group">
          <h3>{name}</h3>
        </div>
      </button>

      <div className="project-card-status-control">
        <button
          ref={statusButtonRef}
          className={`project-card-status ${status}`}
          type="button"
          aria-haspopup="menu"
          aria-expanded={isStatusMenuOpen}
          onClick={() => setIsStatusMenuOpen((isOpen) => !isOpen)}
        >
          <span aria-hidden="true">●</span>
          {statusLabels[status]}
        </button>

        {isStatusMenuOpen
          ? createPortal(
              <div
                ref={statusMenuRef}
                className="project-card-status-menu"
                role="menu"
                style={{
                  top: `${menuPosition.top}px`,
                  left: `${menuPosition.left}px`,
                }}
              >
                <strong>Change project status</strong>
                {statusOptions.map((option) => (
                  <button
                    className={`project-card-status-menu-item ${option.value}`}
                    type="button"
                    role="menuitem"
                    key={option.value}
                    onClick={() => selectStatus(option.value)}
                  >
                    <span aria-hidden="true">{option.marker}</span>
                    {option.label}
                  </button>
                ))}
              </div>,
              document.body,
            )
          : null}
      </div>
    </div>
  );
}
