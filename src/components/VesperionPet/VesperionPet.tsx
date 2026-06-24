import {
  useEffect,
  useState,
  type CSSProperties,
  type PointerEventHandler,
} from "react";
import "./VesperionPet.css";

export type VesperionAnimation =
  | "idle"
  | "dragging"
  | "running-left"
  | "running-right";

const animations = {
  idle: {
    row: 0,
    durations: [280, 110, 110, 140, 140, 320],
  },
  dragging: {
    row: 3,
    durations: [90, 90, 90, 90, 90, 90, 90, 160],
  },
  "running-right": {
    row: 1,
    durations: [120, 120, 120, 120, 120, 120, 120, 220],
  },
  "running-left": {
    row: 2,
    durations: [120, 120, 120, 120, 120, 120, 120, 220],
  },
} as const satisfies Record<
  VesperionAnimation,
  { row: number; durations: readonly number[] }
>;

type VesperionStyle = CSSProperties & {
  "--vesperion-frame": number;
  "--vesperion-row": number;
};

type VesperionPetProps = {
  animation?: VesperionAnimation;
  dragging?: boolean;
  shadow?: boolean;
  onPointerDown?: PointerEventHandler<HTMLDivElement>;
};

export function VesperionPet({
  animation = "idle",
  dragging = false,
  shadow = true,
  onPointerDown,
}: VesperionPetProps) {
  const [frame, setFrame] = useState(0);
  const [reduceMotion, setReduceMotion] = useState(false);
  const animationConfig = animations[animation];

  useEffect(() => {
    const mediaQuery = window.matchMedia("(prefers-reduced-motion: reduce)");
    const updatePreference = () => setReduceMotion(mediaQuery.matches);

    updatePreference();
    mediaQuery.addEventListener("change", updatePreference);

    return () => mediaQuery.removeEventListener("change", updatePreference);
  }, []);

  useEffect(() => {
    setFrame(0);
  }, [animation]);

  useEffect(() => {
    if (reduceMotion) {
      setFrame(0);
      return;
    }

    const timeout = window.setTimeout(() => {
      setFrame(
        (currentFrame) =>
          (currentFrame + 1) % animationConfig.durations.length,
      );
    }, animationConfig.durations[frame]);

    return () => window.clearTimeout(timeout);
  }, [animationConfig, frame, reduceMotion]);

  const style: VesperionStyle = {
    "--vesperion-frame": frame,
    "--vesperion-row": animationConfig.row,
  };

  return (
    <div
      className={[
        "vesperion-pet-shell",
        dragging ? "is-dragging" : "",
        shadow ? "" : "no-shadow",
      ]
        .filter(Boolean)
        .join(" ")}
      role="img"
      aria-label="VESPΣRION, compagnon de VespΣr"
      onPointerDown={onPointerDown}
    >
      <div className="vesperion-pet" style={style} />
    </div>
  );
}
