import { useEffect, useRef, useState, type PointerEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  PhysicalPosition,
  currentMonitor,
  getCurrentWindow,
} from "@tauri-apps/api/window";
import {
  VesperionPet,
  type VesperionAnimation,
} from "./VesperionPet";
import "./VesperionOverlay.css";

const positionStorageKey = "vesperion-window-position";

type StoredPosition = {
  x: number;
  y: number;
};

type VesperionFeedback = {
  status: "success" | "error";
  accessLabel: string;
};

function readStoredPosition(): StoredPosition | null {
  try {
    const value = localStorage.getItem(positionStorageKey);
    if (!value) return null;

    const position = JSON.parse(value) as Partial<StoredPosition>;
    if (typeof position.x !== "number" || typeof position.y !== "number") {
      return null;
    }

    return { x: position.x, y: position.y };
  } catch {
    return null;
  }
}

export function VesperionOverlay() {
  const [animation, setAnimation] = useState<VesperionAnimation>("idle");
  const [isDragging, setIsDragging] = useState(false);
  const [feedback, setFeedback] = useState<VesperionFeedback | null>(null);
  const dragging = useRef(false);
  const lastX = useRef<number | null>(null);
  const feedbackTimeout = useRef<number | undefined>(undefined);

  useEffect(() => {
    let disposed = false;
    let stopListening: (() => void) | undefined;

    void listen<VesperionFeedback>("vesperion-feedback", ({ payload }) => {
      if (
        (payload.status !== "success" && payload.status !== "error") ||
        typeof payload.accessLabel !== "string"
      ) {
        return;
      }

      setFeedback(payload);
      window.clearTimeout(feedbackTimeout.current);
      feedbackTimeout.current = window.setTimeout(
        () => setFeedback(null),
        payload.status === "success" ? 3000 : 5000,
      );
    }).then((unlisten) => {
      if (disposed) unlisten();
      else stopListening = unlisten;
    });

    return () => {
      disposed = true;
      window.clearTimeout(feedbackTimeout.current);
      stopListening?.();
    };
  }, []);

  useEffect(() => {
    const overlayWindow = getCurrentWindow();
    let disposed = false;
    let stopListening: (() => void) | undefined;
    let settleTimeout: number | undefined;

    async function initializePosition() {
      const storedPosition = readStoredPosition();

      if (storedPosition) {
        await overlayWindow.setPosition(
          new PhysicalPosition(storedPosition.x, storedPosition.y),
        );
      } else {
        const monitor = await currentMonitor();
        const windowSize = await overlayWindow.outerSize();

        if (monitor) {
          const sideMargin = Math.round(24 * monitor.scaleFactor);
          const taskbarMargin = Math.round(72 * monitor.scaleFactor);
          const x =
            monitor.position.x + monitor.size.width - windowSize.width - sideMargin;
          const y =
            monitor.position.y + monitor.size.height - windowSize.height - taskbarMargin;

          await overlayWindow.setPosition(new PhysicalPosition(x, y));
        }
      }

      const unlisten = await overlayWindow.onMoved(({ payload }) => {
        localStorage.setItem(positionStorageKey, JSON.stringify(payload));

        if (lastX.current !== null && payload.x !== lastX.current) {
          setAnimation(
            payload.x > lastX.current ? "running-right" : "running-left",
          );
        }

        lastX.current = payload.x;
        window.clearTimeout(settleTimeout);
        settleTimeout = window.setTimeout(
          () => setAnimation(dragging.current ? "dragging" : "idle"),
          360,
        );
      });

      if (disposed) {
        unlisten();
      } else {
        stopListening = unlisten;
      }
    }

    void initializePosition().catch((error: unknown) => {
      console.error("Unable to initialize the VESPΣRION overlay", error);
    });

    return () => {
      disposed = true;
      window.clearTimeout(settleTimeout);
      stopListening?.();
    };
  }, []);

  async function startDragging(event: PointerEvent<HTMLDivElement>) {
    if (event.button !== 0 || dragging.current) return;

    dragging.current = true;
    setIsDragging(true);
    setAnimation("dragging");

    try {
      await invoke("drag_vesperion");
    } catch (error: unknown) {
      console.error("Unable to drag VESPΣRION across the desktop", error);
    } finally {
      dragging.current = false;
      setIsDragging(false);
      setAnimation("idle");
    }
  }

  return (
    <main className="vesperion-overlay">
      {feedback ? (
        <div
          className={`vesperion-feedback is-${feedback.status}`}
          role="status"
          aria-live="polite"
        >
          <strong>{feedback.status === "success" ? "Ouvert" : "Échec"}</strong>
          <span>{feedback.accessLabel}</span>
        </div>
      ) : null}
      <VesperionPet
        animation={animation}
        dragging={isDragging}
        onPointerDown={startDragging}
      />
    </main>
  );
}
