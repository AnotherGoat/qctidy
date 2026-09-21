import React, { useState, useEffect } from "react";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import type { GateData, GateState, Grid } from "~/types/circuit";

interface GateEditorProps {
  gate: GateState;
  grid?: Grid;
  onSave: (props: GateData, moveInstructions?: Record<string, number>) => void;
  onClose: () => void;
}

interface NumberFieldProps {
  label: string;
  value: string;
  hint?: string;
  min?: number;
  max?: number;
  onChange: (value: string) => void;
}

const NumberField: React.FC<NumberFieldProps> = ({
  label,
  value,
  hint,
  min,
  max,
  onChange,
}) => (
  <div className="mb-5 flex flex-col gap-2">
    <Label className="text-sm font-medium text-muted-foreground">{label}</Label>
    <Input
      type="number"
      value={value}
      min={min}
      max={max}
      onChange={(event) => onChange(event.target.value)}
    />
    {hint && <p className="text-xs text-muted-foreground/70">{hint}</p>}
  </div>
);

export const GateEditor: React.FC<GateEditorProps> = ({
  gate,
  grid,
  onSave,
  onClose,
}) => {
  const gateName = gate.type.displayName || gate.id;
  const isAngleGate = ["P", "Rx", "Ry", "Rz", "CP"].includes(gateName);
  const isUGate = gateName === "U";
  const isMeasureGate = gateName === "M";

  const [angleStr, setAngleStr] = useState(String(gate.props.angle ?? 0));
  const [phiStr, setPhiStr] = useState(String(gate.props.phi ?? 0));
  const [lambdaStr, setLambdaStr] = useState(String(gate.props.lambda ?? 0));
  const [classicalBitStr, setClassicalBitStr] = useState(
    String(gate.props.classicalBit ?? 0),
  );

  const isMultiQubit = !!gate.props.multiQubitId;
  const [rowOverrides, setRowOverrides] = useState<Record<string, string>>({});
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  useEffect(() => {
    if (!isMultiQubit || !grid) return;

    const initial: Record<string, string> = {};
    for (let row = 0; row < grid.length; row++) {
      for (let column = 0; column < grid[row].length; column++) {
        const cell = grid[row][column];
        if (!cell) continue;

        if (
          cell.props.multiQubitId === gate.props.multiQubitId &&
          cell.props.multiQubitRole
        ) {
          initial[cell.props.multiQubitRole] = String(row);
        }
      }
    }
    setRowOverrides(initial);
  }, [gate, grid, isMultiQubit]);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") onClose();
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [onClose]);

  const handleSave = () => {
    setErrorMsg(null);
    const newProps: GateData = { ...gate.props };

    if (isAngleGate || isUGate) {
      const angleValue = Number(angleStr);
      if (Number.isNaN(angleValue) || angleValue < 0 || angleValue > 720) {
        setErrorMsg("The angle (Theta) must be between 0 and 720 degrees.");
        return;
      }
      newProps.angle = angleValue;
    }

    if (isUGate) {
      const phiValue = Number(phiStr);
      if (Number.isNaN(phiValue) || phiValue < 0 || phiValue > 720) {
        setErrorMsg("The angle (Phi) must be between 0 and 720 degrees.");
        return;
      }
      newProps.phi = phiValue;

      const lambdaValue = Number(lambdaStr);
      if (Number.isNaN(lambdaValue) || lambdaValue < 0 || lambdaValue > 720) {
        setErrorMsg("The angle (Lambda) must be between 0 and 720 degrees.");
        return;
      }
      newProps.lambda = lambdaValue;
    }

    if (isMeasureGate) {
      const bitValue = Number(classicalBitStr);
      if (
        Number.isNaN(bitValue) ||
        !Number.isInteger(bitValue) ||
        bitValue < 0
      ) {
        setErrorMsg(
          "The classical bit must be an integer greater than or equal to 0.",
        );
        return;
      }
      newProps.classicalBit = bitValue;
    }

    let moveInstructions: Record<string, number> | undefined;
    if (isMultiQubit) {
      moveInstructions = {};
      const targetRows: number[] = [];

      for (const [role, rowStr] of Object.entries(rowOverrides)) {
        const row = Number(rowStr);
        if (Number.isNaN(row) || row < 0 || !Number.isInteger(row)) {
          setErrorMsg("Qubits must be integers greater than or equal to 0.");
          return;
        }
        moveInstructions[role] = row;
        targetRows.push(row);
      }

      if (new Set(targetRows).size !== targetRows.length) {
        setErrorMsg("Two nodes cannot be on the same qubit.");
        return;
      }
    }

    onSave(newProps, moveInstructions);
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm animate-in fade-in duration-200"
      onClick={onClose}
    >
      <div
        className="glass-panel w-full max-w-md rounded-2xl p-6 animate-in zoom-in-95 duration-200"
        onClick={(event) => event.stopPropagation()}
      >
        <div className="mb-5 flex items-center gap-2">
          <span className="grid size-8 place-items-center rounded-lg bg-primary/15 text-sm font-semibold text-primary">
            {gateName}
          </span>
          <h3 className="text-lg font-semibold tracking-tight">
            Gate settings
          </h3>
        </div>

        {errorMsg && (
          <div className="mb-4 rounded-lg border border-destructive/40 bg-destructive/15 px-3 py-2 text-sm text-red-200">
            {errorMsg}
          </div>
        )}

        {(isAngleGate || isUGate) && (
          <NumberField
            label={isUGate ? "Theta (degrees)" : "Angle (degrees)"}
            value={angleStr}
            hint="Value between 0 and 720 degrees."
            min={0}
            max={720}
            onChange={setAngleStr}
          />
        )}

        {isUGate && (
          <>
            <NumberField
              label="Phi (degrees)"
              value={phiStr}
              hint="Value between 0 and 720 degrees."
              min={0}
              max={720}
              onChange={setPhiStr}
            />
            <NumberField
              label="Lambda (degrees)"
              value={lambdaStr}
              hint="Value between 0 and 720 degrees."
              min={0}
              max={720}
              onChange={setLambdaStr}
            />
          </>
        )}

        {isMeasureGate && (
          <NumberField
            label="Classical bit (index)"
            value={classicalBitStr}
            hint="Target classical register to store the result."
            min={0}
            onChange={setClassicalBitStr}
          />
        )}

        {isMultiQubit &&
          Object.entries(rowOverrides).map(([role, value]) => (
            <NumberField
              key={role}
              label={`${role.replace(/([A-Z0-9])/g, " $1").trim()} qubit (row)`}
              value={value}
              min={0}
              onChange={(next) =>
                setRowOverrides((previous) => ({ ...previous, [role]: next }))
              }
            />
          ))}

        {!isAngleGate && !isUGate && !isMeasureGate && !isMultiQubit && (
          <p className="mb-5 text-sm italic text-muted-foreground">
            This gate does not have editable properties.
          </p>
        )}

        <div className="mt-2 flex justify-end gap-3">
          <Button variant="ghost" onClick={onClose}>
            Cancel
          </Button>
          {(isAngleGate || isUGate || isMeasureGate || isMultiQubit) && (
            <Button onClick={handleSave}>Save</Button>
          )}
        </div>
      </div>
    </div>
  );
};
