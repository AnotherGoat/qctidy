import { Circuit, type Grid } from "~/components/circuit";
import type { Route } from "./+types/home";
import { GateSidebar } from "~/components/gate_sidebar";
import { useEffect, useState } from "react";
import {
  DndContext,
  DragOverlay,
  pointerWithin,
  useSensor,
  useSensors,
  PointerSensor,
  type DragEndEvent,
} from "@dnd-kit/core";
import { restrictToFirstScrollableAncestor } from "@dnd-kit/modifiers";
import { GATE_REGISTRY, type GateData } from "~/components/gate";
import React from "react";
import {
  buildJsonFromGrid,
  simplifyCircuit,
  buildGridFromJson,
} from "~/lib/api";
import { Button } from "~/components/ui/button";
import { GateEditor } from "~/components/gate-editor";

function alignGrid(grid: Grid): Grid {
  const newGrid = grid.map(row => [...row]);
  const multiQubits = new Map<string, {row: number, col: number}[]>();

  for (let r = 0; r < newGrid.length; r++) {
    for (let c = 0; c < newGrid[r].length; c++) {
      const cell = newGrid[r][c];
      if (cell?.props.multiQubitId) {
        if (!multiQubits.has(cell.props.multiQubitId)) {
          multiQubits.set(cell.props.multiQubitId, []);
        }
        multiQubits.get(cell.props.multiQubitId)!.push({row: r, col: c});
      }
    }
  }

  let changed = true;
  let iters = 0;
  while (changed && iters < 100) {
    changed = false;
    iters++;
    for (const [id, nodes] of multiQubits.entries()) {
      const currentNodes = nodes.map(n => {
        const c = newGrid[n.row].findIndex(g => g?.props.multiQubitId === id);
        return { row: n.row, col: c };
      });

      const maxCol = Math.max(...currentNodes.map(n => n.col));

      for (const node of currentNodes) {
        if (node.col < maxCol && node.col !== -1) {
          const diff = maxCol - node.col;
          for (let i = 0; i < diff; i++) {
             newGrid[node.row].splice(node.col, 0, null as any);
          }
          changed = true;
        }
      }
    }
  }

  const maxCols = Math.max(...newGrid.map(row => row.length), 0);
  for (let c = maxCols - 1; c >= 0; c--) {
    let isEmpty = true;
    for (let r = 0; r < newGrid.length; r++) {
      if (newGrid[r][c] !== null && newGrid[r][c] !== undefined) {
        isEmpty = false;
        break;
      }
    }
    if (isEmpty) {
      for (let r = 0; r < newGrid.length; r++) {
        if (newGrid[r].length > c) {
          newGrid[r].splice(c, 1);
        }
      }
    }
  }

  return newGrid;
}

export function meta({}: Route.MetaArgs) {
  return [
    { title: "QSimplify" },
    { name: "description", content: "Build and simplify quantum circuits" },
  ];
}

export interface GateState {
  id: string;
  type: React.ComponentType<any>;
  props: GateData;
}

export default function Home() {
  const [grid, setGrid] = useState<Grid>([[], []]);
  const [activeDrag, setActiveDrag] = useState<any>(null);
  const [mouseX, setMouseX] = useState(0);
  const [preview, setPreview] = useState<{
    rows: number[];
    column: number;
  } | null>(null);

  const [editingGate, setEditingGate] = useState<{
    row: number;
    column: number;
    gate: GateState;
  } | null>(null);

  const [simplifiedGrid, setSimplifiedGrid] = useState<Grid | null>(null);
  const [isSimplifying, setIsSimplifying] = useState(false);

  const handleSimplify = async () => {
    try {
      setIsSimplifying(true);
      const jsonRequest = buildJsonFromGrid(grid);
      const response = await simplifyCircuit(jsonRequest);
      const newGrid = alignGrid(buildGridFromJson(response));
      setSimplifiedGrid(newGrid);
    } catch (e) {
      console.error(e);
      alert("Error simplifying the circuit.");
    } finally {
      setIsSimplifying(false);
    }
  };

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 5,
      },
    }),
  );

  useEffect(() => {
    const handler = (e: MouseEvent) => setMouseX(e.clientX);
    window.addEventListener("pointermove", handler);
    return () => window.removeEventListener("pointermove", handler);
  }, []);

  const handleDragStart = (event: any) => {
    setActiveDrag(event.active);
  };

  const handleDragMove = (event: any) => {
    const { over } = event;

    if (!over || typeof over.id !== "string") {
      setPreview(null);
      return;
    }

    const match = over.id.match(/^row-(\d+)$/);
    if (!match) {
      setPreview(null);
      return;
    }

    const rowIndex = Number(match[1]);

    const rowElement = document.querySelector(
      `[data-row="${rowIndex}"]`,
    ) as HTMLElement | null;

    if (!rowElement) {
      setPreview(null);
      return;
    }

    const cells = Array.from(rowElement.querySelectorAll("[data-cell]"));

    let insertIndex = 0;

    if (cells.length === 0) {
      insertIndex = 0;
    } else {
      let closestIndex = 0;
      let closestDistance = Infinity;

      for (let i = 0; i < cells.length; i++) {
        const rect = (cells[i] as HTMLElement).getBoundingClientRect();
        const center = rect.left + rect.width / 2;

        const distance = Math.abs(mouseX - center);

        if (distance < closestDistance) {
          closestDistance = distance;
          closestIndex = i;
        }
      }

      const rect = (cells[closestIndex] as HTMLElement).getBoundingClientRect();

      insertIndex = mouseX < rect.right ? closestIndex : closestIndex + 1;
    }

    const source = event.active.data.current?.source;
    const gateKey = event.active.data.current?.gateKey;
    let previewRows = [rowIndex];

    if (source === "circuit") {
      let movingGate: GateState | null = null;
      let activeRowIndex = 0;

      for (let row = 0; row < grid.length; row++) {
        const column = grid[row].findIndex((cell) => cell?.id === event.active.id);
        if (column !== -1) {
          movingGate = grid[row][column];
          activeRowIndex = row;
          break;
        }
      }

      if (movingGate?.props.multiQubitId) {
        previewRows = grid
          .map((row, index) => row.some((cell) => cell?.props.multiQubitId === movingGate.props.multiQubitId) ? rowIndex + index - activeRowIndex : -1)
          .filter((row) => row !== -1);
      }
    } else if (["CCX", "CCZ", "CSWAP"].includes(gateKey)) {
      previewRows = [rowIndex, rowIndex + 1, rowIndex + 2];
    } else if (["CX", "CY", "CZ", "CH", "CP", "SWAP"].includes(gateKey)) {
      let targetRowIndex = rowIndex + 1;
      if (targetRowIndex >= grid.length) {
        targetRowIndex = Math.max(0, rowIndex - 1);
      }
      previewRows = [rowIndex, targetRowIndex];
    }

    setPreview({ rows: previewRows.filter((row) => row >= 0 && row < grid.length), column: insertIndex });
  };

  const handleDragCancel = () => {
    setActiveDrag(null);
    setPreview(null);
  };

  const handleDragEnd = (event: DragEndEvent) => {
    setActiveDrag(null);
    setPreview(null);
    const { over, active } = event;

    if (!over) {
      const source = active.data.current?.source;
      if (source === "circuit") {
        setGrid((prevGrid) => {
          const newGrid = prevGrid.map((row) => [...row]);
          for (let row = 0; row < newGrid.length; row++) {
            const idx = newGrid[row].findIndex((g) => g?.id === active.id);
            if (idx !== -1) {
              const cell = newGrid[row][idx];
              if (cell?.props.multiQubitId) {
                for (let r = 0; r < newGrid.length; r++) {
                  if (r === row) continue;
                  const partnerIdx = newGrid[r].findIndex(g => g?.props.multiQubitId === cell.props.multiQubitId);
                  if (partnerIdx !== -1) {
                    newGrid[r][partnerIdx] = null as any;
                  }
                }
              }
              newGrid[row][idx] = null as any;
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

      const rowElement = document.querySelector(
        `[data-row="${rowIndex}"]`,
      ) as HTMLElement | null;

      let insertIndex = 0;

      const cells = rowElement ? Array.from(rowElement.querySelectorAll("[data-cell]")) : [];

      if (cells.length === 0) {
        insertIndex = 0;
      } else {
        let closestIndex = 0;
        let closestDistance = Infinity;

        for (let i = 0; i < cells.length; i++) {
          const rect = (cells[i] as HTMLElement).getBoundingClientRect();
          const center = rect.left + rect.width / 2;

          const distance = Math.abs(mouseX - center);

          if (distance < closestDistance) {
            closestDistance = distance;
            closestIndex = i;
          }
        }

        const rect = (
          cells[closestIndex] as HTMLElement
        ).getBoundingClientRect();

        insertIndex = mouseX < rect.right ? closestIndex : closestIndex + 1;
      }

      const source = active.data.current?.source;

      const placeNodes = (grid: Grid, targetNodes: {r: number, node: GateState}[], insertIdx: number) => {
        let collision = false;
        for (const item of targetNodes) {
          const existing = grid[item.r]?.[insertIdx];
          if (existing !== null && existing !== undefined) {
            collision = true;
            break;
          }
        }

        if (collision) {
          if (targetNodes.length === 1) {
            const { r, node } = targetNodes[0];
            grid[r].splice(insertIdx, 0, node);
            return;
          }

          for (let r = 0; r < grid.length; r++) {
            const targetItem = targetNodes.find(t => t.r === r);
            if (targetItem) {
              grid[r].splice(insertIdx, 0, targetItem.node);
            } else {
              grid[r].splice(insertIdx, 0, null as any);
            }
          }
        } else {
          for (const item of targetNodes) {
            while (grid[item.r].length <= insertIdx) {
              grid[item.r].push(null as any);
            }
            grid[item.r][insertIdx] = item.node;
          }
        }
      };

      if (source === "sidebar") {
        const gateKey = active.data.current?.gateKey;
        const gateComponent = GATE_REGISTRY[gateKey];

        if (!gateComponent) return newGrid;

        if (["CCX", "CCZ", "CSWAP"].includes(gateKey)) {
          const multiQubitId = crypto.randomUUID();

          let targetRowIndex1 = rowIndex + 1;
          let targetRowIndex2 = rowIndex + 2;

          while (targetRowIndex2 >= newGrid.length) {
            newGrid.push([]);
          }

          let role0: "control" | "target" | "qubit1" | "qubit2" | "control1" | "control2" | "target1" | "target2" | "qubit3" | undefined;
          let role1: typeof role0;
          let role2: typeof role0;
          if (gateKey === "CCX") { role0 = "control1"; role1 = "control2"; role2 = "target"; }
          else if (gateKey === "CCZ") { role0 = "qubit1"; role1 = "qubit2"; role2 = "qubit3"; }
          else { role0 = "control"; role1 = "target1"; role2 = "target2"; }

          const node0: GateState = { id: crypto.randomUUID(), type: gateComponent, props: { multiQubitId, multiQubitRole: role0 } };
          const node1: GateState = { id: crypto.randomUUID(), type: gateComponent, props: { multiQubitId, multiQubitRole: role1 } };
          const node2: GateState = { id: crypto.randomUUID(), type: gateComponent, props: { multiQubitId, multiQubitRole: role2 } };

          const targetNodes = [
            { r: rowIndex, node: node0 },
            { r: targetRowIndex1, node: node1 },
            { r: targetRowIndex2, node: node2 }
          ];
          placeNodes(newGrid, targetNodes, insertIndex);
          return alignGrid(newGrid);
        }

        if (["CX", "CY", "CZ", "CH", "CP", "SWAP"].includes(gateKey)) {
          const isSwap = gateKey === "SWAP";
          const multiQubitId = crypto.randomUUID();

          let targetRowIndex = rowIndex + 1;
          if (targetRowIndex >= newGrid.length) {
            targetRowIndex = Math.max(0, rowIndex - 1);
          }
          if (targetRowIndex === rowIndex) {
            newGrid.push([]);
            targetRowIndex = 1;
          }

          const controlNode: GateState = {
            id: crypto.randomUUID(),
            type: gateComponent,
            props: { multiQubitId, multiQubitRole: isSwap ? "qubit1" : "control" },
          };

          const targetNode: GateState = {
            id: crypto.randomUUID(),
            type: gateComponent,
            props: { multiQubitId, multiQubitRole: isSwap ? "qubit2" : "target" },
          };

          const targetNodes = [
            { r: rowIndex, node: controlNode },
            { r: targetRowIndex, node: targetNode }
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
        let group: { r: number, c: number, cell: GateState }[] = [];

        for (let r = 0; r < newGrid.length; r++) {
          for (let c = 0; c < newGrid[r].length; c++) {
            if (newGrid[r][c]?.props.multiQubitId === movingGate.props.multiQubitId) {
              group.push({ r, c, cell: newGrid[r][c] });
            }
          }
        }

        if (group.length > 1) {
          const rowDiffs = group.map(g => g.r - fromRow);

          let targetRows = rowDiffs.map(diff => rowIndex + diff);
          const minTarget = Math.min(...targetRows);
          if (minTarget < 0) {
            targetRows = targetRows.map(tr => tr - minTarget);
          }
          const maxTarget = Math.max(...targetRows);
          while (maxTarget >= newGrid.length) {
            newGrid.push([]);
          }

          for (const item of group) {
            newGrid[item.r][item.c] = null as any;
          }

          const targetNodes = targetRows.map((tr, idx) => ({ r: tr, node: group[idx].cell }));
          placeNodes(newGrid, targetNodes, insertIndex);

          return alignGrid(newGrid);
        }
      }

      newGrid[fromRow][fromColumn] = null as any;
      const targetNodes = [{ r: rowIndex, node: movingGate }];
      placeNodes(newGrid, targetNodes, insertIndex);
      return alignGrid(newGrid);
    });
  };

  const addRowTop = () => {
    setGrid((prev) => [[], ...prev]);
  };

  const addRowBottom = () => {
    setGrid((prev) => [...prev, []]);
  };

  const deleteRow = (rowIndex: number) => {
    setGrid((previous) => {
      const rowToDelete = previous[rowIndex];
      if (!rowToDelete) return previous;

      const multiQubitIdsToRemove = new Set(
        rowToDelete.map(cell => cell?.props.multiQubitId).filter(Boolean)
      );

      return previous
        .filter((_, index) => index !== rowIndex)
        .map(row =>
          row.filter(cell => !cell?.props.multiQubitId || !multiQubitIdsToRemove.has(cell.props.multiQubitId))
        );
    });
  };

  const clearCircuit = () => {
    setGrid([[], []]);
    setSimplifiedGrid(null);
  };

  const handleGateClick = (row: number, column: number, gate: GateState) => {
    setEditingGate({ row, column, gate });
  };

  const handleGateSave = (newProps: any, moveInstructions?: Record<string, number>) => {
    if (!editingGate) return;
    setGrid((prevGrid) => {
      const newGrid = prevGrid.map((r) => [...r]);
      const { row, column, gate } = editingGate;

      if (moveInstructions && gate.props.multiQubitId) {
        let group: { r: number, c: number, cell: GateState }[] = [];
        for (let r=0; r<newGrid.length; r++) {
          for (let c=0; c<newGrid[r].length; c++) {
            if (newGrid[r][c]?.props.multiQubitId === gate.props.multiQubitId) {
               group.push({ r, c, cell: newGrid[r][c] });
            }
          }
        }

        if (group.length > 0) {
          const maxRowNeeded = Math.max(...Object.values(moveInstructions));
          while (maxRowNeeded >= newGrid.length) {
            newGrid.push([]);
          }

          for (const item of group) {
            newGrid[item.r][item.c] = null as any;
          }

          let collision = false;
          for (const item of group) {
            const role = item.cell.props.multiQubitRole;
            const newR = (role && moveInstructions[role] !== undefined) ? moveInstructions[role] : item.r;
            if (newGrid[newR][column] != null) collision = true;
          }

          if (collision) {
            for (let r = 0; r < newGrid.length; r++) {
              newGrid[r].splice(column, 0, null as any);
            }
          }

          for (const item of group) {
            const role = item.cell.props.multiQubitRole;
            const newR = (role && moveInstructions[role] !== undefined) ? moveInstructions[role] : item.r;
            item.cell.props = {
              ...item.cell.props,
              ...newProps,
              multiQubitRole: role
            };
            newGrid[newR][column] = item.cell;
          }

          return alignGrid(newGrid);
        }
      }

      newGrid[row][column] = {
        ...newGrid[row][column],
        props: newProps,
      };
      return alignGrid(newGrid);
    });
    setEditingGate(null);
  };

  return (
    <div className="flex h-screen min-w-0 flex-col overflow-hidden bg-transparent">
      <header className="m-2 flex flex-shrink-0 items-center justify-center rounded-xl border border-white/10 bg-slate-950/80 p-6 text-4xl font-extrabold tracking-tight text-white shadow-2xl backdrop-blur-md">
        <span className="bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent drop-shadow-md">
          QSimplify
        </span>
      </header>

      <DndContext
        sensors={sensors}
        collisionDetection={pointerWithin}
        onDragStart={handleDragStart}
        onDragMove={handleDragMove}
        onDragCancel={handleDragCancel}
        onDragEnd={handleDragEnd}
      >
        <div className="flex min-h-0 w-full flex-1 flex-row gap-4 overflow-hidden p-2">
          <aside
            className={`glass-panel flex w-72 flex-shrink-0 flex-col gap-4 overflow-x-hidden overscroll-contain p-4 ${activeDrag ? "overflow-y-hidden" : "overflow-y-auto"}`}
          >
            <GateSidebar />
          </aside>

          <main className="glass-panel flex min-h-0 min-w-0 flex-1 flex-col gap-4 overflow-y-auto overflow-x-hidden overscroll-contain p-6 text-white">
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-3xl font-bold tracking-tight">
                Original Circuit
              </h2>
              <div className="flex gap-3">
                <Button
                  onClick={clearCircuit}
                  variant="outline"
                  className="bg-red-500/10 hover:bg-red-500 hover:text-white border-red-500/50 text-red-200 font-bold py-2 px-6 rounded-full transition-all"
                >
                  Clear All
                </Button>
                <Button
                  onClick={handleSimplify}
                  disabled={isSimplifying}
                  className="bg-blue-600 hover:bg-blue-500 text-white font-bold py-2 px-8 rounded-full shadow-[0_0_15px_rgba(37,99,235,0.5)] transition-all animate-pulse hover:animate-none disabled:opacity-50"
                >
                  {isSimplifying ? "Simplifying..." : "Simplify Circuit ✨"}
                </Button>
              </div>
            </div>

            <Circuit
              grid={grid}
              preview={activeDrag ? preview : null}
              onAddRowTop={addRowTop}
              onAddRowBottom={addRowBottom}
              onRemoveRow={deleteRow}
              onGateClick={handleGateClick}
            />

            {simplifiedGrid && (
              <div className="mt-4 pt-4 animate-in fade-in slide-in-from-bottom-4 duration-500">
                <h2 className="text-3xl font-bold mb-6 text-purple-400 drop-shadow-md">
                  Simplified Circuit
                </h2>
                <div className="opacity-90 pointer-events-none">
                  <Circuit
                    grid={simplifiedGrid}
                    preview={null}
                    onAddRowTop={() => {}}
                    onAddRowBottom={() => {}}
                    onRemoveRow={() => {}}
                    readOnly={true}
                  />
                </div>
              </div>
            )}
          </main>
        </div>

        <DragOverlay dropAnimation={null}>
          {activeDrag
            ? (() => {
                const source = activeDrag.data.current?.source;

                if (source === "sidebar") {
                  const key = activeDrag.data.current?.gateKey;
                  const GateComponent = GATE_REGISTRY[key];
                  if (!GateComponent) return null;

                  if (["CCX", "CCZ", "CSWAP"].includes(key)) {
                    let role0: "control" | "target" | "qubit1" | "qubit2" | "control1" | "control2" | "target1" | "target2" | "qubit3" | undefined;
                    let role1: typeof role0;
                    let role2: typeof role0;
                    if (key === "CCX") { role0 = "control1"; role1 = "control2"; role2 = "target"; }
                    else if (key === "CCZ") { role0 = "qubit1"; role1 = "qubit2"; role2 = "qubit3"; }
                    else { role0 = "control"; role1 = "target1"; role2 = "target2"; }

                    return (
                      <div className="relative flex flex-col" style={{ gap: '0px' }}>
                        <div className="absolute w-1 bg-cyan-400 shadow-[0_0_8px_rgba(34,211,238,0.8)] pointer-events-none" style={{ height: '160px', top: '40px', left: '38px', zIndex: -1 }} />
                        <div className="h-20 w-20 flex items-center justify-center"><GateComponent id={key} isOverlay={true} onSidebar={false} multiQubitRole={role0} /></div>
                        <div className="h-20 w-20 flex items-center justify-center"><GateComponent id={key} isOverlay={true} onSidebar={false} multiQubitRole={role1} /></div>
                        <div className="h-20 w-20 flex items-center justify-center"><GateComponent id={key} isOverlay={true} onSidebar={false} multiQubitRole={role2} /></div>
                      </div>
                    );
                  }

                  if (["CX", "CY", "CZ", "CH", "CP", "SWAP"].includes(key)) {
                    const isSwap = key === "SWAP";
                    return (
                      <div className="relative flex flex-col" style={{ gap: '0px' }}>
                        <div className="absolute w-1 bg-cyan-400 shadow-[0_0_8px_rgba(34,211,238,0.8)] pointer-events-none" style={{ height: '80px', top: '40px', left: '38px', zIndex: -1 }} />
                        <div className="h-20 w-20 flex items-center justify-center"><GateComponent id={key} isOverlay={true} onSidebar={false} multiQubitRole={isSwap ? "qubit1" : "control"} /></div>
                        <div className="h-20 w-20 flex items-center justify-center"><GateComponent id={key} isOverlay={true} onSidebar={false} multiQubitRole={isSwap ? "qubit2" : "target"} /></div>
                      </div>
                    );
                  }

                  return (
                    <div className="h-20 w-20 flex items-center justify-center">
                      <GateComponent id={key} isOverlay={true} onSidebar={false} />
                    </div>
                  );
                }

                const id = String(activeDrag.id);
                let movingGate: GateState | null = null;
                let activeR = 0;
                let activeC = 0;

                for (let r = 0; r < grid.length; r++) {
                  for (let c = 0; c < grid[r].length; c++) {
                    if (grid[r][c]?.id === id) {
                      movingGate = grid[r][c];
                      activeR = r;
                      activeC = c;
                    }
                  }
                }

                if (movingGate) {
                  if (movingGate.props.multiQubitId) {
                    const group: { r: number; cell: GateState }[] = [];
                    for (let r = 0; r < grid.length; r++) {
                      const cell = grid[r][activeC];
                      if (cell?.props.multiQubitId === movingGate.props.multiQubitId) {
                        group.push({ r, cell });
                      }
                    }

                    if (group.length > 0) {
                      group.sort((a, b) => a.r - b.r);
                      const minR = group[0].r;
                      const maxR = group[group.length - 1].r;

                      return (
                        <div className="relative w-20 h-20">
                          {minR !== maxR && (
                            <div className="absolute w-1 bg-cyan-400 shadow-[0_0_8px_rgba(34,211,238,0.8)] pointer-events-none" style={{ height: `${(maxR - minR) * 80}px`, top: `${(minR - activeR) * 80 + 40}px`, left: '38px', zIndex: -1 }} />
                          )}
                          {group.map((item) => {
                            const Comp = item.cell.type;
                            return (
                              <div key={item.cell.id} className="absolute w-20 h-20 flex items-center justify-center" style={{ top: `${(item.r - activeR) * 80}px`, left: 0 }}>
                                <Comp {...item.cell.props} id={item.cell.id} isOverlay={true} onSidebar={false} />
                              </div>
                            );
                          })}
                        </div>
                      );
                    }
                  }

                  const Comp = movingGate.type;
                  return (
                    <div className="w-20 h-20 flex items-center justify-center">
                      <Comp {...movingGate.props} id={movingGate.id} isOverlay={true} onSidebar={false} />
                    </div>
                  );
                }

                return null;
              })()
            : null}
        </DragOverlay>
      </DndContext>

      {editingGate && (
        <GateEditor
          gate={editingGate.gate}
          grid={grid}
          onSave={handleGateSave}
          onClose={() => setEditingGate(null)}
        />
      )}
    </div>
  );
}
