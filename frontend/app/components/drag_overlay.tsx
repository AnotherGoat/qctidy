import type { Active } from "@dnd-kit/core";
import {
  GATE_REGISTRY,
  THREE_QUBIT_GATES,
  TWO_QUBIT_GATES,
} from "~/components/gates";
import type { GateState, Grid } from "~/types/circuit";

interface DragOverlayContentProps {
  active: Active | null;
  grid: Grid;
}

const CONNECTOR_CLASS =
  "pointer-events-none absolute w-1 rounded-full bg-violet-400 shadow-[0_0_10px_rgba(167,139,250,0.85)]";

/** Renders a preview of the gate being dragged, including its connector line. */
export const DragOverlayContent: React.FC<DragOverlayContentProps> = ({
  active,
  grid,
}) => {
  if (!active) return null;

  const source = active.data.current?.source;

  if (source === "sidebar") {
    const key = active.data.current?.gateKey;
    const GateComponent = GATE_REGISTRY[key];
    if (!GateComponent) return null;

    if (THREE_QUBIT_GATES.includes(key)) {
      let role0:
        | "control"
        | "target"
        | "qubit1"
        | "qubit2"
        | "control1"
        | "control2"
        | "target1"
        | "target2"
        | "qubit3"
        | undefined;
      let role1: typeof role0;
      let role2: typeof role0;
      if (key === "CCX") {
        role0 = "control1";
        role1 = "control2";
        role2 = "target";
      } else if (key === "CCZ") {
        role0 = "qubit1";
        role1 = "qubit2";
        role2 = "qubit3";
      } else {
        role0 = "control";
        role1 = "target1";
        role2 = "target2";
      }

      return (
        <div className="relative flex flex-col">
          <div
            className={CONNECTOR_CLASS}
            style={{ height: "160px", top: "40px", left: "38px", zIndex: -1 }}
          />
          <div className="flex h-20 w-20 items-center justify-center">
            <GateComponent
              id={key}
              isOverlay={true}
              onSidebar={false}
              multiQubitRole={role0}
            />
          </div>
          <div className="flex h-20 w-20 items-center justify-center">
            <GateComponent
              id={key}
              isOverlay={true}
              onSidebar={false}
              multiQubitRole={role1}
            />
          </div>
          <div className="flex h-20 w-20 items-center justify-center">
            <GateComponent
              id={key}
              isOverlay={true}
              onSidebar={false}
              multiQubitRole={role2}
            />
          </div>
        </div>
      );
    }

    if (TWO_QUBIT_GATES.includes(key)) {
      const isSwap = key === "SWAP";
      return (
        <div className="relative flex flex-col">
          <div
            className={CONNECTOR_CLASS}
            style={{ height: "80px", top: "40px", left: "38px", zIndex: -1 }}
          />
          <div className="flex h-20 w-20 items-center justify-center">
            <GateComponent
              id={key}
              isOverlay={true}
              onSidebar={false}
              multiQubitRole={isSwap ? "qubit1" : "control"}
            />
          </div>
          <div className="flex h-20 w-20 items-center justify-center">
            <GateComponent
              id={key}
              isOverlay={true}
              onSidebar={false}
              multiQubitRole={isSwap ? "qubit2" : "target"}
            />
          </div>
        </div>
      );
    }

    return (
      <div className="flex h-20 w-20 items-center justify-center">
        <GateComponent id={key} isOverlay={true} onSidebar={false} />
      </div>
    );
  }

  const id = String(active.id);
  let movingGate: GateState | null = null;
  let activeRow = 0;
  let activeColumn = 0;

  for (let row = 0; row < grid.length; row++) {
    for (let column = 0; column < grid[row].length; column++) {
      if (grid[row][column]?.id === id) {
        movingGate = grid[row][column];
        activeRow = row;
        activeColumn = column;
      }
    }
  }

  if (!movingGate) return null;

  if (movingGate.props.multiQubitId) {
    const group: { row: number; cell: GateState }[] = [];
    for (let row = 0; row < grid.length; row++) {
      const cell = grid[row][activeColumn];
      if (cell?.props.multiQubitId === movingGate.props.multiQubitId) {
        group.push({ row, cell });
      }
    }

    if (group.length > 0) {
      group.sort((a, b) => a.row - b.row);
      const minRow = group[0].row;
      const maxRow = group[group.length - 1].row;

      return (
        <div className="relative h-20 w-20">
          {minRow !== maxRow && (
            <div
              className={CONNECTOR_CLASS}
              style={{
                height: `${(maxRow - minRow) * 80}px`,
                top: `${(minRow - activeRow) * 80 + 40}px`,
                left: "38px",
                zIndex: -1,
              }}
            />
          )}
          {group.map((item) => {
            const Component = item.cell.type;
            return (
              <div
                key={item.cell.id}
                className="absolute left-0 flex h-20 w-20 items-center justify-center"
                style={{ top: `${(item.row - activeRow) * 80}px` }}
              >
                <Component
                  {...item.cell.props}
                  id={item.cell.id}
                  isOverlay={true}
                  onSidebar={false}
                />
              </div>
            );
          })}
        </div>
      );
    }
  }

  const Component = movingGate.type;
  return (
    <div className="flex h-20 w-20 items-center justify-center">
      <Component
        {...movingGate.props}
        id={movingGate.id}
        isOverlay={true}
        onSidebar={false}
      />
    </div>
  );
};
