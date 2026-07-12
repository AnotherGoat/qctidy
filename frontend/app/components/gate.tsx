import { useDraggable } from "@dnd-kit/core";
import { CSS } from "@dnd-kit/utilities";
import { Button } from "./ui/button";
import { v4 as uuidv4 } from "uuid";
import { formatAngle } from "~/utils/angle_format";

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

const COLOR_CLASSES: Record<GateColor, string> = {
  [GateColor.Gray]: "bg-gray-300 hover:bg-gray-500",
  [GateColor.Red]: "bg-red-300 hover:bg-red-500",
  [GateColor.Green]: "bg-green-300 hover:bg-green-500",
  [GateColor.Blue]: "bg-blue-300 hover:bg-blue-500",
  [GateColor.Pink]: "bg-pink-300 hover:bg-pink-500",
  [GateColor.Orange]: "bg-orange-300 hover:bg-orange-500",
  [GateColor.Yellow]: "bg-yellow-300 hover:bg-yellow-500",
  [GateColor.Cyan]: "bg-cyan-300 hover:bg-cyan-500",
};

const ACTIVE_COLOR_CLASSES: Record<GateColor, string> = {
  [GateColor.Gray]: "!bg-gray-500",
  [GateColor.Red]: "!bg-red-500",
  [GateColor.Green]: "!bg-green-500",
  [GateColor.Blue]: "!bg-blue-500",
  [GateColor.Pink]: "!bg-pink-500",
  [GateColor.Orange]: "!bg-orange-500",
  [GateColor.Yellow]: "!bg-yellow-500",
  [GateColor.Cyan]: "!bg-cyan-500",
};

export enum GateShape {
  Circle = "circle",
  RoundedRect = "roundedRect",
  Square = "square",
}

interface GateProps {
  id: string;
  label: string;
  color: GateColor;
  onSidebar?: boolean;
  shape?: GateShape;
  subtitle?: string;
  isOverlay?: boolean;
  onClick?: () => void;
  multiQubitRole?: "control" | "target" | "qubit1" | "qubit2" | "control1" | "control2" | "target1" | "target2" | "qubit3";
}

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
  const draggable = useDraggable({
    id,
    data: {
      source: onSidebar ? "sidebar" : "circuit",
      gateKey: onSidebar ? id : undefined,
    },
  });

  const { attributes, listeners, setNodeRef, transform, isDragging } =
    draggable;

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
    "w-14 h-14 relative font-bold text-lg flex flex-col items-center justify-center gap-0 border-2 border-white/20 shadow-lg text-black transition-transform";
  const shapeClasses =
    shape === GateShape.Circle ? "rounded-full" : "rounded-xl";
  const colorClasses =
    isDragging || isOverlay
      ? ACTIVE_COLOR_CLASSES[color]
      : COLOR_CLASSES[color];

  const draggableProps = isOverlay ? {} : { ...listeners, ...attributes };

  return (
    <div
      ref={isOverlay ? undefined : setNodeRef}
      style={style as React.CSSProperties}
    >
      <Button
        id={id}
        variant="default"
        className={`${baseClasses} ${shapeClasses} ${
          !onSidebar && (label === "SWAP" || label === "CSWAP")
            ? "bg-transparent border-none shadow-none text-cyan-400 drop-shadow-[0_0_8px_rgba(34,211,238,0.8)] hover:bg-white/5" 
            : colorClasses
        } ${
          !onSidebar && (multiQubitRole?.startsWith("control") || multiQubitRole === "qubit1" || multiQubitRole === "qubit2" || multiQubitRole === "qubit3" || (label === "CZ" && multiQubitRole === "target") || label === "SWAP" || label === "CSWAP") ? "w-8 h-8 rounded-full !p-0" : ""
        }`}
        onClick={onClick}
        {...draggableProps}
      >
        {!onSidebar && (multiQubitRole?.startsWith("control") || (label === "CZ" && multiQubitRole === "target") || (label === "CCZ" && multiQubitRole?.startsWith("qubit"))) ? (
          <div className="w-4 h-4 bg-cyan-400 rounded-full shadow-[0_0_8px_rgba(34,211,238,0.8)]" />
        ) : !onSidebar && ((label === "CX" || label === "CCX") && multiQubitRole === "target") ? (
          <svg className="w-8 h-8 text-cyan-400 drop-shadow-[0_0_8px_rgba(34,211,238,0.8)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <circle cx="12" cy="12" r="10" />
            <line x1="12" y1="2" x2="12" y2="22" />
            <line x1="2" y1="12" x2="22" y2="12" />
          </svg>
        ) : !onSidebar && (label === "SWAP" || label === "CSWAP") && (multiQubitRole?.startsWith("qubit") || multiQubitRole?.startsWith("target")) ? (
          <svg className="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <line x1="4" y1="4" x2="20" y2="20" />
            <line x1="20" y1="4" x2="4" y2="20" />
          </svg>
        ) : (
          <>
            <span className="text-xl">
              {!onSidebar && label === "CY" ? "Y" : !onSidebar && label === "CH" ? "H" : !onSidebar && label === "CP" ? "P" : label}
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

export interface GateData {
  id?: string;
  onSidebar?: boolean;
  angle?: number;
  phi?: number;
  lambda?: number;
  classicalBit?: number;
  isOverlay?: boolean;
  onClick?: () => void;
  multiQubitId?: string;
  multiQubitRole?: "control" | "target" | "qubit1" | "qubit2" | "control1" | "control2" | "target1" | "target2" | "qubit3";
  controlRow?: number;
  targetRow?: number;
}

interface GateConfig {
  id?: string;
  label: string;
  color: GateColor;
  shape?: GateShape;
  usesAngle?: boolean;
  usesThreeAngles?: boolean;
  usesClassicalBit?: boolean;
}

const createGate = (config: GateConfig): React.FC<GateData> => {
  const GateComponent: React.FC<GateData> = ({
    id,
    onSidebar = true,
    angle = 0,
    phi = 0,
    lambda = 0,
    classicalBit = 0,
    isOverlay = false,
    onClick,
    multiQubitRole,
  }) => {
    const finalId = onSidebar ? (config.id || config.label) : (id ?? uuidv4());

    let subtitle = undefined;
    if (config.usesThreeAngles) {
      subtitle = `${formatAngle(angle)}, ${formatAngle(phi)}, ${formatAngle(lambda)}`;
    } else if (config.usesAngle) {
      subtitle = formatAngle(angle);
    }
    if (config.usesClassicalBit) subtitle = `c[${classicalBit}]`;

    return (
      <Gate
        id={finalId}
        label={config.label}
        color={config.color}
        shape={config.shape}
        onSidebar={onSidebar}
        subtitle={subtitle}
        isOverlay={isOverlay}
        onClick={onClick}
        multiQubitRole={multiQubitRole}
      />
    );
  };
  GateComponent.displayName = config.id || config.label;
  return GateComponent;
};

export const IdGate = createGate({
  label: "I",
  color: GateColor.Gray,
  shape: GateShape.Circle,
});

export const HGate = createGate({ label: "H", color: GateColor.Pink });

export const XGate = createGate({
  label: "X",
  color: GateColor.Red,
  shape: GateShape.Circle,
});

export const YGate = createGate({
  label: "Y",
  color: GateColor.Green,
  shape: GateShape.Circle,
});

export const ZGate = createGate({
  label: "Z",
  color: GateColor.Blue,
  shape: GateShape.Circle,
});

export const PGate = createGate({
  label: "P",
  color: GateColor.Orange,
  usesAngle: true,
});

export const RxGate = createGate({
  label: "Rx",
  color: GateColor.Red,
  shape: GateShape.Circle,
  usesAngle: true,
});

export const RyGate = createGate({
  label: "Ry",
  color: GateColor.Green,
  shape: GateShape.Circle,
  usesAngle: true,
});

export const RzGate = createGate({
  label: "Rz",
  color: GateColor.Blue,
  shape: GateShape.Circle,
  usesAngle: true,
});

export const SGate = createGate({ label: "S", color: GateColor.Blue });

export const SdgGate = createGate({ id: "Sdg", label: "S\u{2020}", color: GateColor.Blue });

export const SxGate = createGate({ id: "Sx", label: "\u{221A}X", color: GateColor.Red });

export const SyGate = createGate({ id: "Sy", label: "\u{221A}Y", color: GateColor.Green });

export const TGate = createGate({ label: "T", color: GateColor.Yellow });

export const TdgGate = createGate({ id: "Tdg", label: "T\u{2020}", color: GateColor.Yellow });

export const UGate = createGate({
  id: "U",
  label: "U",
  color: GateColor.Orange,
  usesThreeAngles: true,
});

export const MeasureGate = createGate({
  label: "M",
  color: GateColor.Gray,
  usesClassicalBit: true,
});

export const CXGate = createGate({
  id: "CX",
  label: "CX",
  color: GateColor.Blue,
  shape: GateShape.Circle,
});

export const CYGate = createGate({
  id: "CY",
  label: "CY",
  color: GateColor.Green,
});

export const CZGate = createGate({
  id: "CZ",
  label: "CZ",
  color: GateColor.Blue,
});

export const CHGate = createGate({
  id: "CH",
  label: "CH",
  color: GateColor.Pink,
});

export const CPGate = createGate({
  id: "CP",
  label: "CP",
  color: GateColor.Orange,
  usesAngle: true,
});

export const CCXGate = createGate({
  id: "CCX",
  label: "CCX",
  color: GateColor.Red,
  shape: GateShape.Circle,
});

export const CCZGate = createGate({
  id: "CCZ",
  label: "CCZ",
  color: GateColor.Blue,
  shape: GateShape.Circle,
});

export const CSWAPGate = createGate({
  id: "CSWAP",
  label: "CSWAP",
  color: GateColor.Cyan,
  shape: GateShape.Square,
});

export const SWAPGate = createGate({
  id: "SWAP",
  label: "SWAP",
  color: GateColor.Blue,
});

export const GATE_REGISTRY: Record<string, React.ComponentType<GateData>> = {
  I: IdGate,
  H: HGate,
  X: XGate,
  Y: YGate,
  Z: ZGate,
  P: PGate,
  Rx: RxGate,
  Ry: RyGate,
  Rz: RzGate,
  S: SGate,
  Sdg: SdgGate,
  Sx: SxGate,
  Sy: SyGate,
  T: TGate,
  Tdg: TdgGate,
  U: UGate,
  CX: CXGate,
  CY: CYGate,
  CZ: CZGate,
  CH: CHGate,
  CP: CPGate,
  SWAP: SWAPGate,
  CCX: CCXGate,
  CCZ: CCZGate,
  CSWAP: CSWAPGate,
  M: MeasureGate,
};
