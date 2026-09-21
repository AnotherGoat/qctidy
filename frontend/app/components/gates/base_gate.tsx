import { useDraggable } from "@dnd-kit/core";
import { Button } from "../ui/button";
import type { MultiQubitRole } from "~/types/circuit";

export enum GateColor {
  Gray = "gray",
  Red = "red",
  Green = "green",
  Blue = "blue",
  Pink = "pink",
  Orange = "orange",
  Yellow = "yellow",
  Cyan = "cyan",
}

const GATE_STYLES: Record<GateColor, { idle: string; active: string }> = {
  [GateColor.Gray]: {
    idle: "bg-slate-500/85 text-white ring-slate-300/40 hover:bg-slate-500",
    active: "!bg-slate-500",
  },
  [GateColor.Red]: {
    idle: "bg-rose-500/90 text-white ring-rose-300/40 hover:bg-rose-500",
    active: "!bg-rose-500",
  },
  [GateColor.Green]: {
    idle: "bg-emerald-500/90 text-white ring-emerald-300/40 hover:bg-emerald-500",
    active: "!bg-emerald-500",
  },
  [GateColor.Blue]: {
    idle: "bg-blue-500/90 text-white ring-blue-300/40 hover:bg-blue-500",
    active: "!bg-blue-500",
  },
  [GateColor.Pink]: {
    idle: "bg-fuchsia-500/90 text-white ring-fuchsia-300/40 hover:bg-fuchsia-500",
    active: "!bg-fuchsia-500",
  },
  [GateColor.Orange]: {
    idle: "bg-orange-500/90 text-white ring-orange-300/40 hover:bg-orange-500",
    active: "!bg-orange-500",
  },
  [GateColor.Yellow]: {
    idle: "bg-amber-400/95 text-amber-950 ring-amber-200/50 hover:bg-amber-400",
    active: "!bg-amber-400",
  },
  [GateColor.Cyan]: {
    idle: "bg-cyan-500/90 text-white ring-cyan-300/40 hover:bg-cyan-500",
    active: "!bg-cyan-500",
  },
};

export enum GateShape {
  Circle = "circle",
  RoundedRect = "roundedRect",
  Square = "square",
}

export interface GateProps {
  id: string;
  label: string;
  color: GateColor;
  onSidebar?: boolean;
  shape?: GateShape;
  subtitle?: string;
  isOverlay?: boolean;
  onClick?: () => void;
  multiQubitRole?: MultiQubitRole;
}

/** A single draggable gate tile, rendered both in the sidebar and on the grid. */
export const Gate = ({
  id,
  label,
  color,
  onSidebar = true,
  shape = GateShape.RoundedRect,
  subtitle,
  isOverlay = false,
  onClick,
  multiQubitRole,
}: GateProps) => {
  const isCzDot =
    label === "CZ" &&
    ["target", "qubit1", "qubit2"].includes(multiQubitRole ?? "");

  const draggable = useDraggable({
    id,
    disabled: isOverlay,
    data: {
      source: onSidebar ? "sidebar" : "circuit",
      gateKey: onSidebar ? id : undefined,
    },
  });

  const { attributes, listeners, setNodeRef, isDragging } = draggable;

  const style = isOverlay
    ? { pointerEvents: "none" as const }
    : onSidebar
      ? undefined
      : {
          opacity: isDragging ? 0.3 : 1,
          zIndex: isDragging ? 1000 : undefined,
          position: isDragging ? "relative" : undefined,
          pointerEvents: isDragging ? "none" : "auto",
        };

  const baseClasses =
    "w-14 h-14 relative font-semibold text-lg flex flex-col items-center justify-center gap-0 border border-white/15 ring-1 ring-inset shadow-lg shadow-black/30 transition-all duration-150 hover:-translate-y-0.5 hover:brightness-110 active:translate-y-0";
  const shapeClasses =
    shape === GateShape.Circle
      ? "rounded-full"
      : shape === GateShape.Square
        ? "rounded-md"
        : "rounded-xl";
  const colorStyles =
    isDragging || isOverlay
      ? GATE_STYLES[color].active
      : GATE_STYLES[color].idle;

  const draggableProps = isOverlay ? {} : { ...listeners, ...attributes };

  return (
    <div
      ref={isOverlay ? undefined : setNodeRef}
      style={style as React.CSSProperties}
      className={
        isOverlay ? "w-20 h-20 flex items-center justify-center" : undefined
      }
    >
      <Button
        id={id}
        variant="default"
        className={`${baseClasses} ${shapeClasses} ${colorStyles} ${
          !onSidebar &&
          (multiQubitRole?.startsWith("control") ||
            multiQubitRole === "qubit1" ||
            multiQubitRole === "qubit2" ||
            multiQubitRole === "qubit3" ||
            isCzDot ||
            label === "SWAP" ||
            label === "CSWAP")
            ? "!w-8 !h-8 rounded-full !p-0"
            : ""
        }`}
        onClick={onClick}
        {...draggableProps}
      >
        {!onSidebar &&
        (multiQubitRole?.startsWith("control") ||
          isCzDot ||
          (label === "CCZ" &&
            multiQubitRole?.startsWith("qubit"))) ? null : !onSidebar &&
          (label === "CX" || label === "CCX") &&
          multiQubitRole === "target" ? (
          <span className="text-xl">X</span>
        ) : !onSidebar &&
          (label === "SWAP" || label === "CSWAP") &&
          (multiQubitRole?.startsWith("qubit") ||
            multiQubitRole?.startsWith("target")) ? (
          <svg
            className="h-6 w-6 text-white"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="3"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <line x1="4" y1="4" x2="20" y2="20" />
            <line x1="20" y1="4" x2="4" y2="20" />
          </svg>
        ) : (
          <>
            <span
              className={
                onSidebar && label === "CSWAP"
                  ? "text-sm"
                  : onSidebar && label === "SWAP"
                    ? "text-lg"
                    : "text-xl"
              }
            >
              {!onSidebar && label === "CY"
                ? "Y"
                : !onSidebar && label === "CH"
                  ? "H"
                  : !onSidebar && label === "CP"
                    ? "P"
                    : label}
            </span>
            {!onSidebar && subtitle && (
              <span className="text-xs font-medium opacity-80">{subtitle}</span>
            )}
          </>
        )}
      </Button>
    </div>
  );
};
