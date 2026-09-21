import React from "react";
import {
  IdGate,
  HGate,
  XGate,
  YGate,
  ZGate,
  PGate,
  RxGate,
  RyGate,
  RzGate,
  SGate,
  SdgGate,
  SxGate,
  SyGate,
  TGate,
  TdgGate,
  UGate,
  CXGate,
  CYGate,
  CZGate,
  CHGate,
  CPGate,
  SWAPGate,
  CCXGate,
  CCZGate,
  CSWAPGate,
  MeasureGate,
} from "./gates";

interface GateGroup {
  title: string;
  accent: string;
  gates: React.ComponentType[];
}

const GATE_GROUPS: GateGroup[] = [
  {
    title: "Basic & Clifford",
    accent: "text-violet-300",
    gates: [IdGate, HGate, XGate, YGate, ZGate, SGate, SdgGate],
  },
  {
    title: "Rotations & Phase",
    accent: "text-blue-300",
    gates: [
      PGate,
      RxGate,
      RyGate,
      RzGate,
      SxGate,
      SyGate,
      TGate,
      TdgGate,
      UGate,
    ],
  },
  {
    title: "Multi-Qubit",
    accent: "text-cyan-300",
    gates: [
      CXGate,
      CYGate,
      CZGate,
      CHGate,
      CPGate,
      SWAPGate,
      CCXGate,
      CCZGate,
      CSWAPGate,
    ],
  },
  {
    title: "Operations",
    accent: "text-muted-foreground",
    gates: [MeasureGate],
  },
];

export const GateSidebar: React.FC = () => {
  return (
    <div className="flex flex-col gap-6 p-1">
      <div className="flex items-center justify-between border-b border-white/10 pb-3">
        <h2 className="text-sm font-semibold uppercase tracking-widest text-muted-foreground">
          Toolbox
        </h2>
        <span className="chip">Drag to place</span>
      </div>

      {GATE_GROUPS.map((group) => (
        <div key={group.title} className="flex flex-col gap-3">
          <h3
            className={`text-xs font-semibold uppercase tracking-wider ${group.accent}`}
          >
            {group.title}
          </h3>
          <div className="grid grid-cols-3 gap-3">
            {group.gates.map((GateComponent, index) => (
              <GateComponent key={index} />
            ))}
          </div>
        </div>
      ))}
    </div>
  );
};
