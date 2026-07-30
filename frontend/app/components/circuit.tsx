import * as React from "react";
import type { GateState } from "~/routes/home";
import { DroppableRow } from "./droppable_row";
import { Button } from "./ui/button";

export type Grid = GateState[][];

export interface CircuitProps {
  grid: Grid;
  preview?: { rows: number[]; column: number } | null;
  onAddRowTop: () => void;
  onAddRowBottom: () => void;
  onRemoveRow: (rowIndex: number) => void;
  onGateClick?: (rowIndex: number, colIndex: number, gate: GateState) => void;
  readOnly?: boolean;
}

function trimRow(row: GateState[]) {
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
  onAddRowTop,
  onAddRowBottom,
  onRemoveRow,
  onGateClick,
  readOnly = false,
}) => {
  return (
    <div className="glass-panel flex w-full flex-shrink-0 flex-col overflow-hidden rounded-xl border border-white/20">
      <div className="overflow-x-auto overflow-y-clip overscroll-x-contain p-4">
        <div className="min-h-max">
          <div className="grid grid-cols-[auto_auto_1fr] gap-x-2 auto-rows-max">
            {!readOnly && (
              <>
                <div className="flex min-h-[80px] items-center justify-center">
                <Button
                  onClick={onAddRowTop}
                  className="h-8 w-8 rounded-full border-2 border-white/20 bg-green-300 p-0 text-xl font-black text-black shadow-lg hover:bg-green-500"
                >
                  +
                </Button>
                </div>
                <div />
                <div /> {}
              </>
            )}
            {grid.map((row, rowIndex) => {
              const trimmed = trimRow(row);

              return (
                <React.Fragment key={rowIndex}>
                  <div className="flex items-center justify-center">
                    {!readOnly && (
                      <Button
                        onClick={() => onRemoveRow(rowIndex)}
                        disabled={grid.length <= 1}
                        className={`h-8 w-8 rounded-full border-2 border-white/20 p-0 text-xl font-black text-black shadow-lg ${
                          grid.length <= 1
                            ? "bg-gray-300/50"
                            : "bg-red-300 hover:bg-red-500"
                        }`}
                      >
                        -
                      </Button>
                    )}
                  </div>
                  <div className="flex min-h-[80px] items-center justify-end pr-1 text-sm font-bold text-white/70">
                    q[{rowIndex}]
                  </div>

                  {readOnly ? (
                    <div className="w-full">
                      <div
                        className="flex min-h-[80px] items-center relative w-max min-w-full"
                        data-row={rowIndex}
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
                                  <div
                                    data-cell
                                    className="w-20 h-20 flex items-center justify-center flex-shrink-0 relative"
                                  >
                                    {(() => {
                                      if (!cell) return null;

                                      let minRow = rowIndex;
                                      let maxRow = rowIndex;
                                      if (cell.props.multiQubitId) {
                                        for (let r = 0; r < grid.length; r++) {
                                          const p = grid[r][i];
                                          if (p?.props.multiQubitId === cell.props.multiQubitId) {
                                            if (r < minRow) minRow = r;
                                            if (r > maxRow) maxRow = r;
                                          }
                                        }
                                      }

                                      const GateComponent = cell.type;
                                      return (
                                        <>
                                          {minRow === rowIndex && maxRow > minRow && (
                                            <div
                                              className="absolute w-1 bg-cyan-400 shadow-[0_0_8px_rgba(34,211,238,0.8)] pointer-events-none"
                                              style={{
                                                height: `${(maxRow - minRow) * 80}px`,
                                                top: '50%',
                                                left: '50%',
                                                transform: 'translateX(-50%)',
                                                zIndex: -1
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
                        data-row={rowIndex}
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
                                    style={{ transform: i > 0 ? "translateX(-8px)" : undefined }}
                                  >
                                    <div className="absolute w-1.5 h-16 bg-blue-500 rounded-full shadow-[0_0_12px_rgba(59,130,246,1)] animate-pulse" />
                                  </div>
                                )}

                                {i < trimmed.length && (
                                  <>
                                      <div
                                        data-cell
                                        className="w-20 h-20 flex items-center justify-center flex-shrink-0 z-10 relative"
                                      >
                                        {(() => {
                                          const cell = trimmed[i];
                                          if (!cell) return null;
                                          const GateComponent = cell.type;

                                          let minRow = rowIndex;
                                          let maxRow = rowIndex;
                                          if (cell.props.multiQubitId) {
                                            for (let r = 0; r < grid.length; r++) {
                                              const p = grid[r][i];
                                              if (p?.props.multiQubitId === cell.props.multiQubitId) {
                                                if (r < minRow) minRow = r;
                                                if (r > maxRow) maxRow = r;
                                              }
                                            }
                                          }

                                          return (
                                            <>
                                              {minRow === rowIndex && maxRow > minRow && (
                                                <div
                                                  className="absolute w-1 bg-cyan-400 shadow-[0_0_8px_rgba(34,211,238,0.8)] pointer-events-none"
                                                  style={{
                                                    height: `${(maxRow - minRow) * 80}px`,
                                                    top: '50%',
                                                    left: '50%',
                                                    transform: 'translateX(-50%)',
                                                    zIndex: -1
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
                <div className="flex items-center justify-center min-h-[80px]">
                <Button
                  onClick={onAddRowBottom}
                  className="h-8 w-8 rounded-full border-2 border-white/20 bg-green-300 p-0 text-xl font-black text-black shadow-lg hover:bg-green-500"
                >
                  +
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
