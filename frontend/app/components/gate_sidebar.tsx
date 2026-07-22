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
} from "./gate";

export const GateSidebar: React.FC<{}> = () => {
  return (
    <div className="flex flex-col gap-6 text-white p-2">
      <h2 className="text-2xl font-bold text-center border-b border-white/20 pb-2 drop-shadow-md">
        Toolbox
      </h2>

      <div className="flex flex-col gap-3">
        <h3 className="font-bold text-sm uppercase tracking-wider text-blue-300">
          Basic & Clifford
        </h3>
        <div className="grid grid-cols-3 gap-3">
          <IdGate />
          <HGate />
          <XGate />
          <YGate />
          <ZGate />
          <SGate />
          <SdgGate />
        </div>
      </div>

      <div className="flex flex-col gap-3">
        <h3 className="font-bold text-sm uppercase tracking-wider text-purple-300">
          Rotations & Phase
        </h3>
        <div className="grid grid-cols-3 gap-3">
          <PGate />
          <RxGate />
          <RyGate />
          <RzGate />
          <SxGate />
          <SyGate />
          <TGate />
          <TdgGate />
          <UGate />
        </div>
      </div>

      <div className="flex flex-col gap-3">
        <h3 className="font-bold text-sm uppercase tracking-wider text-cyan-300">
          Multi-Qubit
        </h3>
        <div className="grid grid-cols-3 gap-3">
          <CXGate />
          <CYGate />
          <CZGate />
          <CHGate />
          <CPGate />
          <SWAPGate />
          <CCXGate />
          <CCZGate />
          <CSWAPGate />
        </div>
      </div>

      <div className="flex flex-col gap-3">
        <h3 className="font-bold text-sm uppercase tracking-wider text-gray-300">
          Operations
        </h3>
        <div className="grid grid-cols-3 gap-3">
          <MeasureGate />
        </div>
      </div>
    </div>
  );
};
