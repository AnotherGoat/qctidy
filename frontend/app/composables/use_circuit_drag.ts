import {
  useEffect,
  useRef,
  useState,
  type Dispatch,
  type SetStateAction,
} from "react";
import {
  useSensor,
  useSensors,
  PointerSensor,
  type Active,
  type DragEndEvent,
  type DragMoveEvent,
  type DragStartEvent,
} from "@dnd-kit/core";
import type {
  GateCell,
  GateState,
  Grid,
  MultiQubitRole,
} from "~/types/circuit";
import {
  GATE_REGISTRY,
  THREE_QUBIT_GATES,
  TWO_QUBIT_GATES,
} from "~/components/gates";
import { alignGrid } from "~/utils/grid";

export interface DragPreview {
  rows: number[];
  column: number;
}

type GridSetter = Dispatch<SetStateAction<Grid>>;

const CELL_WIDTH = 80; // w-20
const CELL_GAP = 16; // w-4
const ROW_LEADING = 8; // w-2
const CELL_PITCH = CELL_WIDTH + CELL_GAP;
const ROW_HEIGHT = 80;

/** Number of rendered cells in a row (trailing empty slots are not rendered). */
function visibleCellCount(row: GateCell[] | undefined): number {
  if (!row) return 0;

  for (let index = row.length - 1; index >= 0; index--) {
    if (row[index]) return index + 1;
  }

  return 0;
}

/**
 * Find where a gate should be inserted in a row from the horizontal pointer
 * position, using the fixed cell geometry instead of DOM queries.
 */
function computeInsertIndex(
  cellCount: number,
  rowLeft: number,
  pointerX: number,
): number {
  if (cellCount === 0) return 0;

  const relativeX = pointerX - rowLeft - ROW_LEADING;
  const nearest = Math.round((relativeX - CELL_WIDTH / 2) / CELL_PITCH);
  const closest = Math.min(Math.max(nearest, 0), cellCount - 1);
  const rightEdge = ROW_LEADING + closest * CELL_PITCH + CELL_WIDTH;

  return relativeX < rightEdge ? closest : closest + 1;
}

/**
 * Drag-and-drop wiring for the circuit grid: tracks the active drag, computes
 * the insertion preview, and applies drops (new gates, moves and qubit edits).
 */
export function useCircuitDrag(grid: Grid, setGrid: GridSetter) {
  const [activeDrag, setActiveDrag] = useState<Active | null>(null);
  const [preview, setPreview] = useState<DragPreview | null>(null);
  // Deepest row reached during a single-qubit drag, and whether the pointer is
  // past it. Only then is one extra empty row rendered to extend into.
  const [deepestRow, setDeepestRow] = useState(-1);
  const [showExtraRow, setShowExtraRow] = useState(false);
  const mouseX = useRef(0);
  const mouseY = useRef(0);
  const lastRealRowBottom = useRef<number | null>(null);
  const deepestRowRef = useRef(-1);
  const isSingleRowDrag = useRef(true);

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 5,
      },
    }),
  );

  useEffect(() => {
    const handler = (event: MouseEvent) => {
      mouseX.current = event.clientX;
      mouseY.current = event.clientY;
    };
    window.addEventListener("pointermove", handler);
    return () => window.removeEventListener("pointermove", handler);
  }, []);

  const updateExtraRows = (hoveredRow: number | null) => {
    if (!isSingleRowDrag.current) {
      setShowExtraRow(false);
      return;
    }

    if (hoveredRow !== null && hoveredRow !== deepestRowRef.current) {
      deepestRowRef.current = hoveredRow;
      setDeepestRow(hoveredRow);
    }

    const cachedBottom = lastRealRowBottom.current;
    if (cachedBottom === null) {
      setShowExtraRow(false);
      return;
    }

    const deepestBottom =
      cachedBottom + (deepestRowRef.current - (grid.length - 1)) * ROW_HEIGHT;

    setShowExtraRow(mouseY.current > deepestBottom);
  };

  const handleDragStart = (event: DragStartEvent) => {
    setActiveDrag(event.active);

    const source = event.active.data.current?.source;
    const gateKey = event.active.data.current?.gateKey;

    if (source === "sidebar") {
      isSingleRowDrag.current =
        !TWO_QUBIT_GATES.includes(gateKey) &&
        !THREE_QUBIT_GATES.includes(gateKey);
    } else {
      const activeId = String(event.active.id);
      isSingleRowDrag.current = !grid.some((row) =>
        row.some((cell) => cell?.id === activeId && !!cell.props.multiQubitId),
      );
    }

    lastRealRowBottom.current = null;
    deepestRowRef.current = -1;
    setDeepestRow(-1);
    setShowExtraRow(false);
  };

  const handleDragMove = (event: DragMoveEvent) => {
    const { over } = event;

    if (!over || typeof over.id !== "string") {
      setPreview(null);
      updateExtraRows(null);
      return;
    }

    const match = over.id.match(/^row-(\d+)$/);
    if (!match) {
      setPreview(null);
      updateExtraRows(null);
      return;
    }

    const rowIndex = Number(match[1]);

    // Remember where the last real qubit row ends, so we can tell when the
    // pointer goes past it.
    if (rowIndex < grid.length) {
      lastRealRowBottom.current =
        over.rect.bottom + (grid.length - 1 - rowIndex) * ROW_HEIGHT;
    }

    const insertIndex = computeInsertIndex(
      visibleCellCount(grid[rowIndex]),
      over.rect.left,
      mouseX.current,
    );

    const source = event.active.data.current?.source;
    const gateKey = event.active.data.current?.gateKey;
    let previewRows = [rowIndex];

    if (source === "circuit") {
      let movingGate: GateState | null = null;
      let activeRowIndex = 0;

      for (let row = 0; row < grid.length; row++) {
        const column = grid[row].findIndex(
          (cell) => cell?.id === event.active.id,
        );
        if (column !== -1) {
          movingGate = grid[row][column];
          activeRowIndex = row;
          break;
        }
      }

      if (movingGate?.props.multiQubitId) {
        previewRows = grid
          .map((row, index) =>
            row.some(
              (cell) =>
                cell?.props.multiQubitId === movingGate!.props.multiQubitId,
            )
              ? rowIndex + index - activeRowIndex
              : null,
          )
          .filter((row): row is number => row !== null);
      }
    } else if (THREE_QUBIT_GATES.includes(gateKey)) {
      previewRows = [rowIndex, rowIndex + 1, rowIndex + 2];
    } else if (TWO_QUBIT_GATES.includes(gateKey)) {
      previewRows = [rowIndex, rowIndex + 1];
    }

    const minRow = Math.min(...previewRows);
    const normalizedRows =
      minRow < 0 ? previewRows.map((row) => row - minRow) : previewRows;

    setPreview({
      rows: normalizedRows.filter((row) => row >= 0),
      column: insertIndex,
    });
    updateExtraRows(rowIndex);
  };

  const handleDragCancel = () => {
    setActiveDrag(null);
    setPreview(null);
    setShowExtraRow(false);
    lastRealRowBottom.current = null;
    deepestRowRef.current = -1;
    setDeepestRow(-1);
  };

  const handleDragEnd = (event: DragEndEvent) => {
    setActiveDrag(null);
    setPreview(null);
    setShowExtraRow(false);
    lastRealRowBottom.current = null;
    deepestRowRef.current = -1;
    setDeepestRow(-1);
    const { over, active } = event;

    if (!over) {
      const source = active.data.current?.source;
      if (source === "circuit") {
        setGrid((previousGrid) => {
          const newGrid = previousGrid.map((row) => [...row]);
          for (let row = 0; row < newGrid.length; row++) {
            const index = newGrid[row].findIndex(
              (gate) => gate?.id === active.id,
            );
            if (index !== -1) {
              const cell = newGrid[row][index];
              if (cell?.props.multiQubitId) {
                for (let r = 0; r < newGrid.length; r++) {
                  if (r === row) continue;
                  const partnerIndex = newGrid[r].findIndex(
                    (gate) =>
                      gate?.props.multiQubitId === cell.props.multiQubitId,
                  );
                  if (partnerIndex !== -1) {
                    newGrid[r][partnerIndex] = null;
                  }
                }
              }
              newGrid[row][index] = null;
            }
          }
          return alignGrid(newGrid);
        });
      }
      return;
    }

    setGrid((previousGrid) => {
      const newGrid = previousGrid.map((row) => [...row]);
      const activeId = String(active.id);
      const overId = String(over.id);

      if (!overId.startsWith("row-")) {
        return newGrid;
      }

      const rowIndex = Number(overId.split("-")[1]);
      const insertIndex = computeInsertIndex(
        visibleCellCount(previousGrid[rowIndex]),
        over.rect.left,
        mouseX.current,
      );

      const source = active.data.current?.source;

      const padRow = (targetGrid: Grid, row: number, length: number) => {
        while (targetGrid.length <= row) {
          targetGrid.push([]);
        }
        while (targetGrid[row].length < length) {
          targetGrid[row].push(null);
        }
      };

      const placeNodes = (
        targetGrid: Grid,
        targetNodes: { r: number; node: GateState }[],
        insertIdx: number,
      ) => {
        let collision = false;
        for (const item of targetNodes) {
          const existing = targetGrid[item.r]?.[insertIdx];
          if (existing !== null && existing !== undefined) {
            collision = true;
            break;
          }
        }

        if (collision) {
          if (targetNodes.length === 1) {
            const { r, node } = targetNodes[0];
            padRow(targetGrid, r, insertIdx);
            targetGrid[r].splice(insertIdx, 0, node);
            return;
          }

          for (let r = 0; r < targetGrid.length; r++) {
            padRow(targetGrid, r, insertIdx);
            const targetItem = targetNodes.find((target) => target.r === r);
            targetGrid[r].splice(
              insertIdx,
              0,
              targetItem ? targetItem.node : null,
            );
          }
        } else {
          for (const item of targetNodes) {
            padRow(targetGrid, item.r, insertIdx + 1);
            targetGrid[item.r][insertIdx] = item.node;
          }
        }
      };

      if (source === "sidebar") {
        const gateKey = active.data.current?.gateKey;
        const gateComponent = GATE_REGISTRY[gateKey];

        if (!gateComponent) return newGrid;

        if (THREE_QUBIT_GATES.includes(gateKey)) {
          const multiQubitId = crypto.randomUUID();
          const targetRowIndex1 = rowIndex + 1;
          let targetRowIndex2 = rowIndex + 2;

          while (targetRowIndex2 >= newGrid.length) {
            newGrid.push([]);
          }

          let role0: MultiQubitRole | undefined;
          let role1: MultiQubitRole | undefined;
          let role2: MultiQubitRole | undefined;
          if (gateKey === "CCX") {
            role0 = "control1";
            role1 = "control2";
            role2 = "target";
          } else if (gateKey === "CCZ") {
            role0 = "qubit1";
            role1 = "qubit2";
            role2 = "qubit3";
          } else {
            role0 = "control";
            role1 = "target1";
            role2 = "target2";
          }

          const node0: GateState = {
            id: crypto.randomUUID(),
            type: gateComponent,
            props: { multiQubitId, multiQubitRole: role0 },
          };
          const node1: GateState = {
            id: crypto.randomUUID(),
            type: gateComponent,
            props: { multiQubitId, multiQubitRole: role1 },
          };
          const node2: GateState = {
            id: crypto.randomUUID(),
            type: gateComponent,
            props: { multiQubitId, multiQubitRole: role2 },
          };

          const targetNodes = [
            { r: rowIndex, node: node0 },
            { r: targetRowIndex1, node: node1 },
            { r: targetRowIndex2, node: node2 },
          ];
          placeNodes(newGrid, targetNodes, insertIndex);
          return alignGrid(newGrid);
        }

        if (TWO_QUBIT_GATES.includes(gateKey)) {
          const isSwap = gateKey === "SWAP";
          const multiQubitId = crypto.randomUUID();

          const targetRowIndex = rowIndex + 1;
          while (targetRowIndex >= newGrid.length) {
            newGrid.push([]);
          }

          const controlNode: GateState = {
            id: crypto.randomUUID(),
            type: gateComponent,
            props: {
              multiQubitId,
              multiQubitRole: isSwap ? "qubit1" : "control",
            },
          };

          const targetNode: GateState = {
            id: crypto.randomUUID(),
            type: gateComponent,
            props: {
              multiQubitId,
              multiQubitRole: isSwap ? "qubit2" : "target",
            },
          };

          const targetNodes = [
            { r: rowIndex, node: controlNode },
            { r: targetRowIndex, node: targetNode },
          ];
          placeNodes(newGrid, targetNodes, insertIndex);
          return alignGrid(newGrid);
        }

        const newGate: GateState = {
          id: crypto.randomUUID(),
          type: gateComponent,
          props: {
            angle: 0,
            classicalBit: 0,
          },
        };

        const targetNodes = [{ r: rowIndex, node: newGate }];
        placeNodes(newGrid, targetNodes, insertIndex);
        return alignGrid(newGrid);
      }

      let movingGate: GateState | null = null;
      let fromRow = -1;
      let fromColumn = -1;

      for (let row = 0; row < newGrid.length; row++) {
        for (let column = 0; column < newGrid[row].length; column++) {
          if (newGrid[row][column]?.id === activeId) {
            fromRow = row;
            fromColumn = column;
            movingGate = newGrid[row][column];
          }
        }
      }

      if (!movingGate) {
        return newGrid;
      }

      if (movingGate.props.multiQubitId) {
        const group: { r: number; c: number; cell: GateState }[] = [];

        for (let r = 0; r < newGrid.length; r++) {
          for (let c = 0; c < newGrid[r].length; c++) {
            const cell = newGrid[r][c];
            if (cell?.props.multiQubitId === movingGate.props.multiQubitId) {
              group.push({ r, c, cell });
            }
          }
        }

        if (group.length > 1) {
          const rowDiffs = group.map((item) => item.r - fromRow);

          let targetRows = rowDiffs.map((diff) => rowIndex + diff);
          const minTarget = Math.min(...targetRows);
          if (minTarget < 0) {
            targetRows = targetRows.map((target) => target - minTarget);
          }
          const maxTarget = Math.max(...targetRows);
          while (maxTarget >= newGrid.length) {
            newGrid.push([]);
          }

          for (const item of group) {
            newGrid[item.r][item.c] = null;
          }

          const targetNodes = targetRows.map((target, index) => ({
            r: target,
            node: group[index].cell,
          }));
          placeNodes(newGrid, targetNodes, insertIndex);

          return alignGrid(newGrid);
        }
      }

      newGrid[fromRow][fromColumn] = null;
      const targetNodes = [{ r: rowIndex, node: movingGate }];
      placeNodes(newGrid, targetNodes, insertIndex);
      return alignGrid(newGrid);
    });
  };

  return {
    activeDrag,
    preview,
    deepestRow,
    showExtraRow,
    sensors,
    handleDragStart,
    handleDragMove,
    handleDragCancel,
    handleDragEnd,
  };
}
