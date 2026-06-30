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
}

const COLOR_CLASSES: Record<GateColor, string> = {
  [GateColor.Gray]: "bg-gray-300 hover:bg-gray-500",
  [GateColor.Red]: "bg-red-300 hover:bg-red-500",
  [GateColor.Green]: "bg-green-300 hover:bg-green-500",
  [GateColor.Blue]: "bg-blue-300 hover:bg-blue-500",
  [GateColor.Pink]: "bg-pink-300 hover:bg-pink-500",
  [GateColor.Orange]: "bg-orange-300 hover:bg-orange-500",
  [GateColor.Yellow]: "bg-yellow-300 hover:bg-yellow-500",
};

const ACTIVE_COLOR_CLASSES: Record<GateColor, string> = {
  [GateColor.Gray]: "!bg-gray-500",
  [GateColor.Red]: "!bg-red-500",
  [GateColor.Green]: "!bg-green-500",
  [GateColor.Blue]: "!bg-blue-500",
  [GateColor.Pink]: "!bg-pink-500",
  [GateColor.Orange]: "!bg-orange-500",
  [GateColor.Yellow]: "!bg-yellow-500",
};

export enum GateShape {
  Circle = "circle",
  RoundedRect = "roundedRect",
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
          transform: CSS.Translate.toString(transform),
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
        className={`${baseClasses} ${shapeClasses} ${colorClasses}`}
        onClick={onClick}
        {...draggableProps}
      >
        <span className="text-xl">{label}</span>
        {!onSidebar && subtitle && (
          <span className="text-xs font-medium opacity-80">{subtitle}</span>
        )}
      </Button>
    </div>
  );
};

export interface GateData {
  id?: string;
  onSidebar?: boolean;
  angle?: number;
  classicalBit?: number;
  isOverlay?: boolean;
  onClick?: () => void;
}

interface GateConfig {
  label: string;
  color: GateColor;
  shape?: GateShape;
  usesAngle?: boolean;
  usesClassicalBit?: boolean;
}

const createGate = (config: GateConfig): React.FC<GateData> => {
  const GateComponent: React.FC<GateData> = ({
    id,
    onSidebar = true,
    angle = 0,
    classicalBit = 0,
    isOverlay = false,
    onClick,
  }) => {
    const finalId = onSidebar ? config.label : (id ?? uuidv4());

    let subtitle = undefined;
    if (config.usesAngle) subtitle = formatAngle(angle);
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
      />
    );
  };
  GateComponent.displayName = config.label;
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

export const SdgGate = createGate({ label: "Sdg", color: GateColor.Blue });

export const SxGate = createGate({ label: "Sx", color: GateColor.Red });

export const SyGate = createGate({ label: "Sy", color: GateColor.Green });

export const TGate = createGate({ label: "T", color: GateColor.Yellow });

export const TdgGate = createGate({ label: "Tdg", color: GateColor.Yellow });

export const MeasureGate = createGate({
  label: "M",
  color: GateColor.Gray,
  usesClassicalBit: true,
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
  M: MeasureGate,
};
