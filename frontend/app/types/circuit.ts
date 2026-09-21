import type { ComponentType } from "react";

export type MultiQubitRole =
  | "control"
  | "target"
  | "qubit1"
  | "qubit2"
  | "control1"
  | "control2"
  | "target1"
  | "target2"
  | "qubit3";

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
  multiQubitRole?: MultiQubitRole;
  controlRow?: number;
  targetRow?: number;
}

/** A gate placed on the circuit grid, with a component and its data. */
export interface GateState {
  id: string;
  type: ComponentType<GateData>;
  props: GateData;
}

/** A grid slot. Empty time steps are represented by `null`. */
export type GateCell = GateState | null;

/** The circuit grid: one row per qubit, one column per time step. */
export type Grid = GateCell[][];
