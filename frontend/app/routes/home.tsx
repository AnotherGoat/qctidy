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
    row: number;
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
      const newGrid = buildGridFromJson(response);
      setSimplifiedGrid(newGrid);
    } catch (e) {
      console.error(e);
      alert("Error al simplificar el circuito.");
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

    setPreview({ row: rowIndex, column: insertIndex });
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
              newGrid[row].splice(idx, 1);
            }
          }
          return newGrid;
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

      if (rowElement) {
        const cells = Array.from(rowElement.querySelectorAll("[data-cell]"));

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
      }

      const source = active.data.current?.source;

      if (source === "sidebar") {
        const gateKey = active.data.current?.gateKey;
        const gateComponent = GATE_REGISTRY[gateKey];

        if (!gateComponent) return newGrid;

        const newGate: GateState = {
          id: crypto.randomUUID(),
          type: gateComponent,
          props: {
            angle: 0,
            classicalBit: 0,
          },
        };

        newGrid[rowIndex].splice(insertIndex, 0, newGate);
        return newGrid;
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

      newGrid[fromRow].splice(fromColumn, 1);

      if (fromRow === rowIndex && fromColumn < insertIndex) {
        insertIndex--;
      }

      newGrid[rowIndex].splice(insertIndex, 0, movingGate);
      return newGrid;
    });
  };

  const addRowTop = () => {
    setGrid((prev) => [[], ...prev]);
  };

  const addRowBottom = () => {
    setGrid((prev) => [...prev, []]);
  };

  const deleteRow = (rowIndex: number) => {
    setGrid((previous) => previous.filter((_, index) => index !== rowIndex));
  };

  const clearCircuit = () => {
    setGrid([[], []]);
    setSimplifiedGrid(null);
  };

  const handleGateClick = (row: number, column: number, gate: GateState) => {
    setEditingGate({ row, column, gate });
  };

  const handleGateSave = (newProps: any) => {
    if (!editingGate) return;
    setGrid((prevGrid) => {
      const newGrid = prevGrid.map((r) => [...r]);
      const { row, column } = editingGate;
      newGrid[row][column] = {
        ...newGrid[row][column],
        props: newProps,
      };
      return newGrid;
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
                  return GateComponent ? (
                    <GateComponent id={key} isOverlay={true} />
                  ) : null;
                }

                const id = String(activeDrag.id);

                for (const row of grid) {
                  for (const cell of row) {
                    if (cell?.id === id) {
                      const GateComponent = cell.type;
                      return (
                        <GateComponent
                          {...cell.props}
                          id={cell.id}
                          isOverlay={true}
                        />
                      );
                    }
                  }
                }

                return null;
              })()
            : null}
        </DragOverlay>
      </DndContext>

      {editingGate && (
        <GateEditor
          gate={editingGate.gate}
          onSave={handleGateSave}
          onClose={() => setEditingGate(null)}
        />
      )}
    </div>
  );
}
