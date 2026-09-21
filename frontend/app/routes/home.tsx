import { Circuit } from "~/components/circuit";
import type { Route } from "./+types/home";
import { GateSidebar } from "~/components/gate_sidebar";
import { useMemo, useRef, useState } from "react";
import { DndContext, DragOverlay, pointerWithin } from "@dnd-kit/core";
import {
  BarChart3,
  CircleDollarSign,
  Download,
  Sparkles,
  Trash2,
  Upload,
} from "lucide-react";
import React from "react";
import type { GateData, GateState, Grid } from "~/types/circuit";
import { buildGridFromJson, buildJsonFromGrid } from "~/utils/api";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "~/components/ui/table";
import { GateEditor } from "~/components/gate-editor";
import { DragOverlayContent } from "~/components/drag_overlay";
import { alignGrid, removeRow, saveGateProps } from "~/utils/grid";
import { useCircuitDrag } from "~/composables/use_circuit_drag";
import {
  useAnalyzeCircuit,
  useEstimateCircuit,
  useSimplifyCircuit,
} from "~/composables/use_circuit_api";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "QCTidy" },
    { name: "description", content: "Build and simplify quantum circuits" },
  ];
}

export default function Home() {
  const [grid, setGrid] = useState<Grid>([[], []]);
  const [editingGate, setEditingGate] = useState<{
    row: number;
    column: number;
    gate: GateState;
  } | null>(null);

  const importInputRef = useRef<HTMLInputElement>(null);

  const simplify = useSimplifyCircuit();
  const estimate = useEstimateCircuit();
  const analyze = useAnalyzeCircuit();

  const simplifiedGrid = useMemo(
    () => (simplify.data ? alignGrid(buildGridFromJson(simplify.data)) : null),
    [simplify.data],
  );
  const requestError = simplify.error ?? estimate.error ?? analyze.error;

  const handleSimplify = () => {
    estimate.clearError();
    analyze.clearError();
    simplify.run(buildJsonFromGrid(grid));
  };
  const handleEstimateCosts = () => {
    simplify.clearError();
    analyze.clearError();
    estimate.run(buildJsonFromGrid(grid).circuit);
  };
  const handleAnalyzeMetrics = () => {
    simplify.clearError();
    estimate.clearError();
    analyze.run(buildJsonFromGrid(grid).circuit);
  };

  const {
    activeDrag,
    preview,
    deepestRow,
    showExtraRow,
    sensors,
    handleDragStart,
    handleDragMove,
    handleDragCancel,
    handleDragEnd,
  } = useCircuitDrag(grid, setGrid);

  const addRowTop = () => {
    setGrid((prev) => [[], ...prev]);
  };

  const addRowBottom = () => {
    setGrid((prev) => [...prev, []]);
  };

  const deleteRow = (rowIndex: number) => {
    setGrid((previous) => removeRow(previous, rowIndex));
  };

  const clearCircuit = () => {
    setGrid([[], []]);
    simplify.reset();
    estimate.reset();
    analyze.reset();
  };

  const handleExportJson = () => {
    const blob = new Blob(
      [JSON.stringify(buildJsonFromGrid(grid).circuit, null, 2)],
      {
        type: "application/json",
      },
    );
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = "qctidy-circuit.json";
    link.click();
    URL.revokeObjectURL(url);
  };

  const handleImportJson = async (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const file = event.target.files?.[0];
    event.target.value = "";
    if (!file) return;

    try {
      const parsed = JSON.parse(await file.text());
      if (!parsed?.operations || typeof parsed.qubit_count !== "number") {
        throw new Error("Invalid circuit JSON");
      }

      setGrid(alignGrid(buildGridFromJson({ circuit: parsed })));
      simplify.reset();
      estimate.reset();
      analyze.reset();
    } catch (error) {
      console.error(error);
      alert("Error importing the JSON circuit.");
    }
  };

  const handleGateClick = (row: number, column: number, gate: GateState) => {
    setEditingGate({ row, column, gate });
  };

  const handleGateSave = (
    newProps: GateData,
    moveInstructions?: Record<string, number>,
  ) => {
    if (!editingGate) return;
    setGrid((previousGrid) =>
      saveGateProps(previousGrid, editingGate, newProps, moveInstructions),
    );
    setEditingGate(null);
  };

  const metricEntries = analyze.data
    ? Object.entries(analyze.data.metrics)
    : [];
  const metricColumns = [
    metricEntries.slice(0, Math.ceil(metricEntries.length / 2)),
    metricEntries.slice(Math.ceil(metricEntries.length / 2)),
  ];

  return (
    <div className="flex h-screen min-w-0 flex-col overflow-hidden bg-transparent">
      <header className="flex h-14 flex-shrink-0 items-center justify-between border-b border-border bg-background/70 px-4 backdrop-blur-xl">
        <div className="flex items-center gap-2.5">
          <span className="grid size-8 place-items-center rounded-lg bg-gradient-to-br from-violet-500 to-cyan-500 text-sm font-bold text-white shadow-lg">
            Q
          </span>
          <span className="text-base font-semibold tracking-tight">QCTidy</span>
          <span className="chip hidden sm:inline-flex">
            Quantum circuit editor
          </span>
        </div>

        <div className="flex items-center gap-2">
          <Button variant="outline" size="sm" onClick={handleExportJson}>
            <Download />
            <span className="hidden md:inline">Export</span>
          </Button>
          <input
            ref={importInputRef}
            type="file"
            accept="application/json,.json"
            className="hidden"
            onChange={handleImportJson}
          />
          <Button
            variant="outline"
            size="sm"
            onClick={() => importInputRef.current?.click()}
          >
            <Upload />
            <span className="hidden md:inline">Import</span>
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={clearCircuit}
            className="text-muted-foreground hover:text-red-300"
          >
            <Trash2 />
            <span className="hidden md:inline">Clear</span>
          </Button>

          <div className="mx-1 h-5 w-px bg-border" />

          <Button
            size="sm"
            onClick={handleSimplify}
            disabled={simplify.isLoading}
          >
            <Sparkles />
            {simplify.isLoading ? "Simplifying…" : "Simplify"}
          </Button>
          <Button
            variant="secondary"
            size="sm"
            onClick={handleEstimateCosts}
            disabled={estimate.isLoading}
          >
            <CircleDollarSign />
            {estimate.isLoading ? "Estimating…" : "Estimate"}
          </Button>
          <Button
            variant="secondary"
            size="sm"
            onClick={handleAnalyzeMetrics}
            disabled={analyze.isLoading}
          >
            <BarChart3 />
            {analyze.isLoading ? "Analyzing…" : "Metrics"}
          </Button>
        </div>
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

          <main className="glass-panel flex min-h-0 min-w-0 flex-1 flex-col gap-4 overflow-y-auto overflow-x-hidden overscroll-contain p-6">
            <div className="mb-2 flex items-center justify-between gap-3">
              <h2 className="text-lg font-semibold tracking-tight">
                Original Circuit
              </h2>
              <span className="chip">
                {grid.length} qubit{grid.length === 1 ? "" : "s"}
              </span>
            </div>

            {requestError && (
              <div className="rounded-lg border border-destructive/40 bg-destructive/15 px-3 py-2 text-sm text-red-200">
                {requestError}
              </div>
            )}

            <Circuit
              grid={grid}
              preview={activeDrag ? preview : null}
              deepestRow={deepestRow}
              showExtraRow={showExtraRow}
              onAddRowTop={addRowTop}
              onAddRowBottom={addRowBottom}
              onRemoveRow={deleteRow}
              onGateClick={handleGateClick}
            />

            {simplifiedGrid && (
              <Card className="animate-in fade-in slide-in-from-bottom-4 duration-500">
                <CardHeader>
                  <CardTitle className="text-violet-300">
                    Simplified Circuit
                  </CardTitle>
                </CardHeader>
                <CardContent className="pointer-events-none opacity-90">
                  <Circuit
                    grid={simplifiedGrid}
                    preview={null}
                    onAddRowTop={() => {}}
                    onAddRowBottom={() => {}}
                    onRemoveRow={() => {}}
                    readOnly={true}
                  />
                </CardContent>
              </Card>
            )}

            {estimate.data && (
              <Card className="animate-in fade-in slide-in-from-bottom-4 duration-500">
                <CardHeader>
                  <CardTitle className="text-cyan-300">
                    Cost Estimates
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>Provider</TableHead>
                        <TableHead>Status</TableHead>
                        <TableHead>Plan</TableHead>
                        <TableHead>Price</TableHead>
                        <TableHead className="text-right">Cost USD</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {estimate.data.estimates.flatMap((providerEstimate) =>
                        providerEstimate.estimates.length > 0
                          ? providerEstimate.estimates.map((cost) => (
                              <TableRow
                                key={`${providerEstimate.provider}-${cost.plan_name}-${cost.price_label}`}
                              >
                                <TableCell>
                                  {providerEstimate.provider}
                                </TableCell>
                                <TableCell>{providerEstimate.status}</TableCell>
                                <TableCell>{cost.plan_name}</TableCell>
                                <TableCell>{cost.price_label}</TableCell>
                                <TableCell className="text-right font-mono tabular-nums">
                                  {cost.cost_usd == null
                                    ? "-"
                                    : `$${cost.cost_usd.toFixed(4)}`}
                                </TableCell>
                              </TableRow>
                            ))
                          : [
                              <TableRow key={providerEstimate.provider}>
                                <TableCell>
                                  {providerEstimate.provider}
                                </TableCell>
                                <TableCell>{providerEstimate.status}</TableCell>
                                <TableCell colSpan={3}>
                                  No estimates returned
                                </TableCell>
                              </TableRow>,
                            ],
                      )}
                    </TableBody>
                  </Table>
                </CardContent>
              </Card>
            )}

            {analyze.data && (
              <Card className="animate-in fade-in slide-in-from-bottom-4 duration-500">
                <CardHeader>
                  <CardTitle className="text-blue-300">Metrics</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="grid gap-4 lg:grid-cols-2">
                    {metricColumns.map((metrics, columnIndex) => (
                      <Table key={columnIndex}>
                        <TableHeader>
                          <TableRow>
                            <TableHead>Metric</TableHead>
                            <TableHead className="text-right">Value</TableHead>
                          </TableRow>
                        </TableHeader>
                        <TableBody>
                          {metrics.map(([metric, value]) => (
                            <TableRow key={metric}>
                              <TableCell>
                                {metric.replaceAll("_", " ")}
                              </TableCell>
                              <TableCell className="text-right font-mono tabular-nums">
                                {Number.isInteger(value)
                                  ? value
                                  : value.toFixed(4)}
                              </TableCell>
                            </TableRow>
                          ))}
                        </TableBody>
                      </Table>
                    ))}
                  </div>
                </CardContent>
              </Card>
            )}
          </main>
        </div>

        <DragOverlay dropAnimation={null}>
          <DragOverlayContent active={activeDrag} grid={grid} />
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
