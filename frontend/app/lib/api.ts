import type { GateState } from "~/routes/home";

export interface QSimplifyRequest {
  circuit: {
    qubit_count: number;
    operations: Record<string, any>[];
  };
  iterations: number;
}

export interface QSimplifyResponse {
  circuit: {
    version: number;
    qubit_count: number;
    operations: Record<string, any>[];
  };
}

export function buildJsonFromGrid(grid: GateState[][]): QSimplifyRequest {
  const qubit_count = grid.length;
  const operations: Record<string, any>[] = [];

  // Find the maximum number of columns across all rows
  const maxCols = Math.max(...grid.map((row) => row.length), 0);

  // Iterate column by column to maintain chronological order
  for (let c = 0; c < maxCols; c++) {
    for (let r = 0; r < qubit_count; r++) {
      const cell = grid[r][c];
      if (cell) {
        const gateName = (cell.type.displayName || "id").toLowerCase();

        let op: Record<string, any> = {};

        if (["cx", "cy", "cz", "cp", "ch"].includes(gateName)) {
          // Placeholder: Multi-qubit gates need control and target
          op = {
            gate: gateName,
            control: r,
            target: (r + 1) % qubit_count, // Dummy target for now
          };
        } else if (["ccx", "cswap"].includes(gateName)) {
          op = {
            gate: gateName,
            control1: r,
            control2: (r + 1) % qubit_count,
            target: (r + 2) % qubit_count,
          };
        } else if (gateName === "measure" || gateName === "m") {
          op = {
            gate: "m",
            qubit: r,
            bit:
              cell.props?.classicalBit !== undefined
                ? cell.props.classicalBit
                : r,
          };
        } else {
          // Single qubit gate
          op = {
            gate: gateName,
            qubit: r,
          };

          if (cell.props?.angle !== undefined) {
            // Convert angle from degrees to radians for the backend
            op.theta = cell.props.angle * (Math.PI / 180);
          }
        }

        operations.push(op);
      }
    }
  }

  return {
    circuit: {
      qubit_count,
      operations,
    },
    iterations: 10,
  };
}

export async function simplifyCircuit(
  request: QSimplifyRequest,
): Promise<QSimplifyResponse> {
  const response = await fetch("/api/simplify", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    throw new Error(`API Error: ${response.status} ${response.statusText}`);
  }

  return await response.json();
}

import { GATE_REGISTRY } from "~/components/gate";

export function buildGridFromJson(response: QSimplifyResponse): GateState[][] {
  const { qubit_count, operations } = response.circuit;
  const grid: GateState[][] = Array.from({ length: qubit_count }, () => []);

  // Simplistic layout: just push gates to their respective rows.
  // In a real scenario we'd need to pad with IdGate to align multi-qubit gates correctly,
  // but for the visualizer, this works as a baseline.
  for (const op of operations) {
    const gateKey = Object.keys(GATE_REGISTRY).find(
      (k) => k.toLowerCase() === op.gate.toLowerCase(),
    );

    const type = gateKey ? GATE_REGISTRY[gateKey] : GATE_REGISTRY["I"];

    // Determine the primary row for the gate
    const primaryRow = op.qubit ?? op.control ?? op.control1 ?? 0;

    grid[primaryRow].push({
      id: crypto.randomUUID(),
      type,
      props: {
        angle: op.theta !== undefined ? op.theta * (180 / Math.PI) : undefined,
        classicalBit: op.bit,
      },
    });
  }

  return grid;
}
