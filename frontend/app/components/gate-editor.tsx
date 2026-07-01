import React, { useState, useEffect } from "react";
import { Button } from "./ui/button";
import type { GateState } from "~/routes/home";

interface GateEditorProps {
  gate: GateState;
  onSave: (props: any) => void;
  onClose: () => void;
}

export const GateEditor: React.FC<GateEditorProps> = ({
  gate,
  onSave,
  onClose,
}) => {
  const gateName = gate.type.displayName || gate.id;
  const isAngleGate = ["P", "Rx", "Ry", "Rz"].includes(gateName);
  const isUGate = gateName === "U";
  const isMeasureGate = gateName === "M";

  const [angleStr, setAngleStr] = useState(String(gate.props.angle ?? 0));
  const [phiStr, setPhiStr] = useState(String(gate.props.phi ?? 0));
  const [lambdaStr, setLambdaStr] = useState(String(gate.props.lambda ?? 0));
  const [classicalBitStr, setClassicalBitStr] = useState(
    String(gate.props.classicalBit ?? 0),
  );
  
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [onClose]);

  const handleSave = () => {
    setErrorMsg(null);
    const newProps = { ...gate.props };

    if (isAngleGate || isUGate) {
      const angleVal = Number(angleStr);
      if (isNaN(angleVal) || angleVal < 0 || angleVal > 720) {
        setErrorMsg("El ángulo (Theta) debe estar entre 0 y 720 grados.");
        return;
      }
      newProps.angle = angleVal;
    }

    if (isUGate) {
      const phiVal = Number(phiStr);
      if (isNaN(phiVal) || phiVal < 0 || phiVal > 720) {
        setErrorMsg("El ángulo (Phi) debe estar entre 0 y 720 grados.");
        return;
      }
      newProps.phi = phiVal;

      const lambdaVal = Number(lambdaStr);
      if (isNaN(lambdaVal) || lambdaVal < 0 || lambdaVal > 720) {
        setErrorMsg("El ángulo (Lambda) debe estar entre 0 y 720 grados.");
        return;
      }
      newProps.lambda = lambdaVal;
    }

    if (isMeasureGate) {
      const bitVal = Number(classicalBitStr);
      if (isNaN(bitVal) || !Number.isInteger(bitVal) || bitVal < 0) {
        setErrorMsg("El bit clásico debe ser un número entero mayor o igual a 0.");
        return;
      }
      newProps.classicalBit = bitVal;
    }
    onSave(newProps);
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm animate-in fade-in duration-200"
      onClick={onClose}
    >
      <div
        className="bg-slate-900 border border-white/20 p-6 rounded-xl shadow-2xl min-w-[320px] text-white"
        onClick={(event) => event.stopPropagation()}
      >
        <h3 className="text-2xl font-bold mb-4 flex items-center gap-2">
          <span className="text-blue-400">Settings:</span> {gateName} Gate
        </h3>

        {errorMsg && (
          <div className="mb-4 p-2 bg-red-900/50 border border-red-500 rounded text-red-200 text-sm">
            {errorMsg}
          </div>
        )}

        {(isAngleGate || isUGate) && (
          <div className="flex flex-col gap-2 mb-6">
            <label className="text-sm font-semibold text-gray-300">
              {isUGate ? "Theta (Degrees)" : "Angle (Degrees)"}
            </label>
            <input
              type="number"
              value={angleStr}
              onChange={(e) => setAngleStr(e.target.value)}
              className="bg-slate-800 border border-slate-600 rounded-md p-2 text-white text-lg focus:outline-none focus:border-blue-500 transition-colors"
              min={0}
              max={720}
            />
            <p className="text-xs text-gray-500">
              Value between 0 and 720 degrees.
            </p>
          </div>
        )}

        {isUGate && (
          <>
            <div className="flex flex-col gap-2 mb-6">
              <label className="text-sm font-semibold text-gray-300">
                Phi (Degrees)
              </label>
              <input
                type="number"
                value={phiStr}
                onChange={(e) => setPhiStr(e.target.value)}
                className="bg-slate-800 border border-slate-600 rounded-md p-2 text-white text-lg focus:outline-none focus:border-blue-500 transition-colors"
                min={0}
                max={720}
              />
              <p className="text-xs text-gray-500">
                Value between 0 and 720 degrees.
              </p>
            </div>
            <div className="flex flex-col gap-2 mb-6">
              <label className="text-sm font-semibold text-gray-300">
                Lambda (Degrees)
              </label>
              <input
                type="number"
                value={lambdaStr}
                onChange={(e) => setLambdaStr(e.target.value)}
                className="bg-slate-800 border border-slate-600 rounded-md p-2 text-white text-lg focus:outline-none focus:border-blue-500 transition-colors"
                min={0}
                max={720}
              />
              <p className="text-xs text-gray-500">
                Value between 0 and 720 degrees.
              </p>
            </div>
          </>
        )}

        {isMeasureGate && (
          <div className="flex flex-col gap-2 mb-6">
            <label className="text-sm font-semibold text-gray-300">
              Classical Bit (Index)
            </label>
            <input
              type="number"
              value={classicalBitStr}
              onChange={(e) => setClassicalBitStr(e.target.value)}
              className="bg-slate-800 border border-slate-600 rounded-md p-2 text-white text-lg focus:outline-none focus:border-blue-500 transition-colors"
              min={0}
            />
            <p className="text-xs text-gray-500">
              Target classical register to store the result.
            </p>
          </div>
        )}

        {!isAngleGate && !isUGate && !isMeasureGate && (
          <div className="mb-6 text-gray-400 italic">
            This gate does not have editable properties.
          </div>
        )}

        <div className="flex justify-end gap-3 mt-2">
          <Button
            variant="outline"
            onClick={onClose}
            className="bg-transparent border-gray-600 text-gray-300 hover:bg-gray-800 hover:text-white"
          >
            Cancel
          </Button>
          {(isAngleGate || isUGate || isMeasureGate) && (
            <Button
              onClick={handleSave}
              className="bg-blue-600 hover:bg-blue-500 text-white font-bold px-6"
            >
              Save
            </Button>
          )}
        </div>
      </div>
    </div>
  );
};
