import * as React from "react";
import { Minus, Plus } from "lucide-react";
import type { GateCell, GateState, Grid } from "~/types/circuit";
import { DroppableRow } from "./droppable_row";
import { Button } from "./ui/button";

export interface CircuitProps {
  grid: Grid;
  preview?: { rows: number[]; column: number } | null;
  deepestRow?: number;
  showExtraRow?: boolean;
  onAddRowTop: () => void;
  onAddRowBottom: () => void;
  onRemoveRow: (rowIndex: number) => void;
  onGateClick?: (rowIndex: number, colIndex: number, gate: GateState) => void;
  readOnly?: boolean;
}

function trimRow(row: GateCell[]) {
  let lastNonNull = -1;

  for (let i = row.length - 1; i >= 0; i--) {
    if (row[i] !== null) {
      lastNonNull = i;
      break;
    }
  }

  return lastNonNull === -1 ? [] : row.slice(0, lastNonNull + 1);
}

export const Circuit: React.FC<CircuitProps> = ({
  grid,
  preview,
  deepestRow = -1,
  showExtraRow = false,
  onAddRowTop,
  onAddRowBottom,
  onRemoveRow,
  onGateClick,
  readOnly = false,
}) => {
  const previewRows = preview?.rows ?? [];

  // Render the rows the gate preview spans (including rows it will create), so
  // multi-qubit gates can extend the circuit while dragging. For single-qubit
  // gates, keep the deepest row reached and add one more only when the pointer
  // goes past it.
  const rowCount = readOnly
    ? grid.length
    : Math.max(
        grid.length,
        deepestRow + 1,
        ...previewRows.map((row) => row + 1),
      ) + (showExtraRow ? 1 : 0);

  return (
    <div className="glass-panel flex w-full flex-shrink-0 flex-col overflow-hidden rounded-xl">
      <div className="overflow-x-auto overflow-y-clip overscroll-x-contain p-4">
        <div className="min-h-max">
          <div className="grid grid-cols-[auto_auto_1fr] gap-x-2 auto-rows-max">
            {!readOnly && (
              <>
                <div className="flex min-h-[80px] items-center justify-center">
                  <Button
                    onClick={onAddRowTop}
                    variant="outline"
                    size="icon"
                    aria-label="Add qubit above"
                    className="h-8 w-8 rounded-full border-emerald-400/30 text-emerald-300 hover:bg-emerald-500/20 hover:text-emerald-200"
                  >
                    <Plus className="size-4" />
                  </Button>
                </div>
                <div />
                <div /> {}
              </>
            )}
            {Array.from({ length: rowCount }, (_, rowIndex) => {
              const row = grid[rowIndex] ?? [];
              const trimmed = trimRow(row);
              const isVirtual = rowIndex >= grid.length;

              return (
                <React.Fragment key={rowIndex}>
                  <div className="flex items-center justify-center">
                    {!readOnly && !isVirtual && (
                      <Button
                        onClick={() => onRemoveRow(rowIndex)}
                        disabled={grid.length <= 1}
                        variant="outline"
                        size="icon"
                        aria-label={`Remove qubit ${rowIndex}`}
                        className="h-8 w-8 rounded-full border-rose-400/30 text-rose-300 hover:bg-rose-500/20 hover:text-rose-200"
                      >
                        <Minus className="size-4" />
                      </Button>
                    )}
                  </div>
                  <div className="flex min-h-[80px] items-center justify-end pr-2">
                    <span className="chip font-mono">q{rowIndex}</span>
                  </div>

                  {readOnly ? (
                    <div className="w-full">
                      <div
                        className="flex min-h-[80px] items-center relative w-max min-w-full"
                        style={{
                          backgroundImage:
                            "repeating-linear-gradient(to right, rgba(255,255,255,0.2) 0 7px, transparent 7px 14px)",
                          backgroundPosition: "0 50%",
                          backgroundRepeat: "repeat-x",
                          backgroundSize: "auto 2px",
                        }}
                      >
                        <div className="w-2 h-20 flex-shrink-0" />
                        {(() => {
                          const elements = [];
                          for (let i = 0; i < trimmed.length; i++) {
                            const cell = trimmed[i];
                            elements.push(
                              <React.Fragment key={i}>
                                <div className="w-20 h-20 flex items-center justify-center flex-shrink-0 relative">
                                  {(() => {
                                    if (!cell) return null;

                                    let minRow = rowIndex;
                                    let maxRow = rowIndex;
                                    if (cell.props.multiQubitId) {
                                      for (let r = 0; r < grid.length; r++) {
                                        const p = grid[r][i];
                                        if (
                                          p?.props.multiQubitId ===
                                          cell.props.multiQubitId
                                        ) {
                                          if (r < minRow) minRow = r;
                                          if (r > maxRow) maxRow = r;
                                        }
                                      }
                                    }

                                    const GateComponent = cell.type;
                                    return (
                                      <>
                                        {minRow === rowIndex &&
                                          maxRow > minRow && (
                                            <div
                                              className="pointer-events-none absolute w-1 rounded-full bg-violet-400 shadow-[0_0_10px_rgba(167,139,250,0.85)]"
                                              style={{
                                                height: `${(maxRow - minRow) * 80}px`,
                                                top: "50%",
                                                left: "50%",
                                                transform: "translateX(-50%)",
                                                zIndex: -1,
                                              }}
                                            />
                                          )}
                                        <GateComponent
                                          {...cell.props}
                                          id={cell.id}
                                          onSidebar={false}
                                          isOverlay={false}
                                        />
                                      </>
                                    );
                                  })()}
                                </div>
                                <div className="w-4 h-20 flex-shrink-0" />
                              </React.Fragment>,
                            );
                          }
                          return elements;
                        })()}
                      </div>
                    </div>
                  ) : (
                    <DroppableRow id={`row-${rowIndex}`}>
                      <div
                        className="flex min-h-[80px] items-center relative w-max min-w-full"
                        style={{
                          backgroundImage:
                            "repeating-linear-gradient(to right, rgba(255,255,255,0.2) 0 7px, transparent 7px 14px)",
                          backgroundPosition: "0 50%",
                          backgroundRepeat: "repeat-x",
                          backgroundSize: "auto 2px",
                        }}
                      >
                        <div className="w-2 h-20 flex-shrink-0" />

                        {(() => {
                          const elements = [];

                          for (let i = 0; i <= trimmed.length; i++) {
                            const isPreviewHere =
                              preview &&
                              preview.rows.includes(rowIndex) &&
                              preview.column === i;

                            elements.push(
                              <React.Fragment key={i}>
                                {isPreviewHere && (
                                  <div
                                    className="relative w-0 h-20 flex-shrink-0 flex items-center justify-center z-20"
                                    style={{
                                      transform:
                                        i > 0 ? "translateX(-8px)" : undefined,
                                    }}
                                  >
                                    <div className="absolute h-16 w-1.5 animate-pulse rounded-full bg-violet-400 shadow-[0_0_12px_rgba(167,139,250,0.9)]" />
                                  </div>
                                )}

                                {i < trimmed.length && (
                                  <>
                                    <div className="relative z-10 flex h-20 w-20 flex-shrink-0 items-center justify-center">
                                      {(() => {
                                        const cell = trimmed[i];
                                        if (!cell) return null;
                                        const GateComponent = cell.type;

                                        let minRow = rowIndex;
                                        let maxRow = rowIndex;
                                        if (cell.props.multiQubitId) {
                                          for (
                                            let r = 0;
                                            r < grid.length;
                                            r++
                                          ) {
                                            const p = grid[r][i];
                                            if (
                                              p?.props.multiQubitId ===
                                              cell.props.multiQubitId
                                            ) {
                                              if (r < minRow) minRow = r;
                                              if (r > maxRow) maxRow = r;
                                            }
                                          }
                                        }

                                        return (
                                          <>
                                            {minRow === rowIndex &&
                                              maxRow > minRow && (
                                                <div
                                                  className="pointer-events-none absolute w-1 rounded-full bg-violet-400 shadow-[0_0_10px_rgba(167,139,250,0.85)]"
                                                  style={{
                                                    height: `${(maxRow - minRow) * 80}px`,
                                                    top: "50%",
                                                    left: "50%",
                                                    transform:
                                                      "translateX(-50%)",
                                                    zIndex: -1,
                                                  }}
                                                />
                                              )}
                                            <GateComponent
                                              {...cell.props}
                                              id={cell.id}
                                              onSidebar={false}
                                              onClick={() =>
                                                onGateClick?.(rowIndex, i, cell)
                                              }
                                            />
                                          </>
                                        );
                                      })()}
                                    </div>
                                    <div className="w-4 h-20 flex-shrink-0" />
                                  </>
                                )}
                              </React.Fragment>,
                            );
                          }

                          return elements;
                        })()}
                      </div>
                    </DroppableRow>
                  )}
                </React.Fragment>
              );
            })}
            {!readOnly && (
              <>
                <div className="flex min-h-[80px] items-center justify-center">
                  <Button
                    onClick={onAddRowBottom}
                    variant="outline"
                    size="icon"
                    aria-label="Add qubit below"
                    className="h-8 w-8 rounded-full border-emerald-400/30 text-emerald-300 hover:bg-emerald-500/20 hover:text-emerald-200"
                  >
                    <Plus className="size-4" />
                  </Button>
                </div>
                <div />
                <div />
              </>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
