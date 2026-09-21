import type { ComponentType, FC } from "react";
import { Gate, GateColor, GateShape } from "./base_gate";
import { formatAngle } from "~/utils/angle_format";
import type { GateData } from "~/types/circuit";

interface GateConfig {
  id?: string;
  label: string;
  color: GateColor;
  shape?: GateShape;
  usesAngle?: boolean;
  usesThreeAngles?: boolean;
  usesClassicalBit?: boolean;
}

const createGate = (config: GateConfig): FC<GateData> => {
  const GateComponent: FC<GateData> = ({
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
    const finalId = onSidebar
      ? config.id || config.label
      : (id ?? crypto.randomUUID());

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

export const SdgGate = createGate({
  id: "Sdg",
  label: "S\u{2020}",
  color: GateColor.Blue,
});

export const SxGate = createGate({
  id: "Sx",
  label: "\u{221A}X",
  color: GateColor.Red,
});

export const SyGate = createGate({
  id: "Sy",
  label: "\u{221A}Y",
  color: GateColor.Green,
});

export const TGate = createGate({ label: "T", color: GateColor.Yellow });

export const TdgGate = createGate({
  id: "Tdg",
  label: "T\u{2020}",
  color: GateColor.Yellow,
});

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
  color: GateColor.Red,
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

export const GATE_REGISTRY: Record<string, ComponentType<GateData>> = {
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
