import * as React from "react";
import type { GateState } from "~/routes/home";
import { DroppableRow } from "./droppable_row";
import { Button } from "./ui/button";

export type Grid = GateState[][];

export interface CircuitProps {
  grid: Grid;
  preview?: { row: number; column: number } | null;
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
      <div className="overflow-x-auto overflow-y-clip overscroll-x-contain p-4 pb-12">
        <div className="min-h-max">
          <div className="grid grid-cols-[auto_1fr] gap-x-2 auto-rows-max">
            <div className="flex items-center justify-center">
              {!readOnly && (
                <Button
                  onClick={onAddRowTop}
                  className="px-2 py-1 bg-green-500/80 hover:bg-green-500 text-white rounded"
                >
                  +
                </Button>
              )}
            </div>
            <div /> {}
            {grid.map((row, rowIndex) => {
              const trimmed = trimRow(row);

              return (
                <React.Fragment key={rowIndex}>
                  <div className="flex items-center justify-center">
                    {!readOnly && (
                      <Button
                        onClick={() => onRemoveRow(rowIndex)}
                        disabled={grid.length <= 1}
                        className={`px-2 py-1 rounded ${
                          grid.length <= 1
                            ? "bg-gray-400/50"
                            : "bg-red-500/80 hover:bg-red-500 text-white"
                        }`}
                      >
                        -
                      </Button>
                    )}
                  </div>

                  {readOnly ? (
                    <div className="w-full">
                      <div
                        className="flex min-h-[80px] items-center relative w-max min-w-full"
                        data-row={rowIndex}
                        style={{
                          backgroundImage:
                            "repeating-linear-gradient(to right, rgba(255,255,255,0.7) 0 7px, transparent 7px 14px)",
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
                                  className="w-20 h-20 flex items-center justify-center flex-shrink-0"
                                >
                                  {(() => {
                                    const GateComponent = cell.type;
                                    return (
                                      <GateComponent
                                        {...cell.props}
                                        id={cell.id}
                                        onSidebar={false}
                                        isOverlay={false}
                                      />
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
                            "repeating-linear-gradient(to right, rgba(255,255,255,0.7) 0 7px, transparent 7px 14px)",
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
                              preview.row === rowIndex &&
                              preview.column === i;

                            elements.push(
                              <React.Fragment key={i}>
                                {isPreviewHere && (
                                  <div className="relative w-0 h-20 flex-shrink-0 flex items-center justify-center z-20">
                                    <div className="absolute w-1.5 h-16 bg-blue-500 rounded-full shadow-[0_0_12px_rgba(59,130,246,1)] animate-pulse" />
                                  </div>
                                )}

                                {i < trimmed.length && (
                                  <>
                                    <div
                                      data-cell
                                      className="w-20 h-20 flex items-center justify-center flex-shrink-0 z-10"
                                    >
                                      {(() => {
                                        const cell = trimmed[i];
                                        if (!cell) return null;
                                        const GateComponent = cell.type;
                                        return (
                                          <GateComponent
                                            {...cell.props}
                                            id={cell.id}
                                            onSidebar={false}
                                            onClick={() =>
                                              onGateClick?.(rowIndex, i, cell)
                                            }
                                          />
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
            <div className="flex items-center justify-center min-h-[80px]">
              {!readOnly && (
                <button
                  onClick={onAddRowBottom}
                  className="px-2 py-1 bg-green-500/80 hover:bg-green-500 text-white rounded"
                >
                  +
                </button>
              )}
            </div>
            <div />
          </div>
        </div>
      </div>
    </div>
  );
};
