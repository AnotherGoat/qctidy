import React, { useState, useEffect } from "react";
import { Button } from "./ui/button";
import type { GateState } from "~/routes/home";

interface GateEditorProps {
  gate: GateState;
  onSave: (props: any) => void;
  onClose: () => void;
}

export const GateEditor: React.FC<GateEditorProps> = ({ gate, onSave, onClose }) => {
  const gateName = gate.type.displayName || "";
  const isAngleGate = ["P", "Rx", "Ry", "Rz"].includes(gateName);
  const isMeasureGate = gateName === "M";

  const [angleStr, setAngleStr] = useState(String(gate.props.angle ?? 0));
  const [classicalBitStr, setClassicalBitStr] = useState(String(gate.props.classicalBit ?? 0));

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [onClose]);

  const handleSave = () => {
    const newProps = { ...gate.props };
    if (isAngleGate) newProps.angle = Number(angleStr) || 0;
    if (isMeasureGate) newProps.classicalBit = Number(classicalBitStr) || 0;
    onSave(newProps);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm animate-in fade-in duration-200">
      <div className="bg-slate-900 border border-white/20 p-6 rounded-xl shadow-2xl min-w-[320px] text-white">
        <h3 className="text-2xl font-bold mb-4 flex items-center gap-2">
          <span className="text-blue-400">Settings:</span> {gateName} Gate
        </h3>
        
        {isAngleGate && (
          <div className="flex flex-col gap-2 mb-6">
            <label className="text-sm font-semibold text-gray-300">Angle (Degrees)</label>
            <input 
              type="number"
              value={angleStr}
              onChange={(e) => setAngleStr(e.target.value)}
              className="bg-slate-800 border border-slate-600 rounded-md p-2 text-white text-lg focus:outline-none focus:border-blue-500 transition-colors"
              min={0}
              max={720}
            />
            <p className="text-xs text-gray-500">Value between 0 and 720 degrees.</p>
          </div>
        )}

        {isMeasureGate && (
          <div className="flex flex-col gap-2 mb-6">
            <label className="text-sm font-semibold text-gray-300">Classical Bit (Index)</label>
            <input 
              type="number"
              value={classicalBitStr}
              onChange={(e) => setClassicalBitStr(e.target.value)}
              className="bg-slate-800 border border-slate-600 rounded-md p-2 text-white text-lg focus:outline-none focus:border-blue-500 transition-colors"
              min={0}
            />
            <p className="text-xs text-gray-500">Target classical register to store the result.</p>
          </div>
        )}

        {!isAngleGate && !isMeasureGate && (
          <div className="mb-6 text-gray-400 italic">
            This gate does not have editable properties.
          </div>
        )}

        <div className="flex justify-end gap-3 mt-2">
          <Button variant="outline" onClick={onClose} className="bg-transparent border-gray-600 text-gray-300 hover:bg-gray-800 hover:text-white">
            Cancel
          </Button>
          {(isAngleGate || isMeasureGate) && (
            <Button onClick={handleSave} className="bg-blue-600 hover:bg-blue-500 text-white font-bold px-6">
              Save
            </Button>
          )}
        </div>
      </div>
    </div>
  );
};
