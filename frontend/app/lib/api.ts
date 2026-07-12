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
  
  // 1. Extract all gates with their row and column coordinates
  const allGates: Array<{ row: number, col: number, gate: GateState }> = [];
  for (let r = 0; r < grid.length; r++) {
    for (let c = 0; c < grid[r].length; c++) {
      if (grid[r][c]) {
        allGates.push({ row: r, col: c, gate: grid[r][c] });
      }
    }
  }

  // 2. Group by multiQubitId (if exists) or by gate id
  const groupedGates = new Map<string, Array<{ row: number, col: number, gate: GateState }>>();
  for (const item of allGates) {
    const key = item.gate.props?.multiQubitId || item.gate.id;
    if (!groupedGates.has(key)) {
      groupedGates.set(key, []);
    }
    groupedGates.get(key)!.push(item);
  }

  // 3. Convert to operations
  const operationsList: Array<{ op: Record<string, any>, sortCol: number }> = [];

  for (const [key, group] of groupedGates.entries()) {
    // The chronological order is dictated by the maximum column index of the group's nodes
    const sortCol = Math.max(...group.map(g => g.col));
    
    if (group.length === 1 && !group[0].gate.props?.multiQubitId) {
      // Single qubit gate
      const { row, gate } = group[0];
      const gateName = (gate.type.displayName || "id").toLowerCase();
      
      let op: Record<string, any> = {};
      
      if (gateName === "measure" || gateName === "m") {
        op = {
          gate: "m",
          qubit: row,
          bit: gate.props?.classicalBit !== undefined ? gate.props.classicalBit : row,
        };
      } else if (gateName === "u") {
        op = { gate: "u", qubit: row };
        op.theta = (gate.props?.angle !== undefined ? gate.props.angle : 0) * (Math.PI / 180);
        op.phi = (gate.props?.phi !== undefined ? gate.props.phi : 0) * (Math.PI / 180);
        op.lambda = (gate.props?.lambda !== undefined ? gate.props.lambda : 0) * (Math.PI / 180);
      } else {
        op = { gate: gateName, qubit: row };
        if (["rx", "ry", "rz", "p"].includes(gateName)) {
           op.theta = (gate.props?.angle !== undefined ? gate.props.angle : 0) * (Math.PI / 180);
        }
      }
      
      operationsList.push({ op, sortCol });
    } else if (group[0].gate.props?.multiQubitId) {
      const gateName = (group[0].gate.type.displayName || "cx").toLowerCase();
      let op: Record<string, any> = { gate: gateName };

      if (["ccx", "ccz", "cswap"].includes(gateName)) {
         if (group.length === 3) {
           if (gateName === "ccz") {
             const q1 = group.find(g => g.gate.props?.multiQubitRole === "qubit1");
             const q2 = group.find(g => g.gate.props?.multiQubitRole === "qubit2");
             const q3 = group.find(g => g.gate.props?.multiQubitRole === "qubit3");
             if (q1 && q2 && q3) { op.qubit1 = q1.row; op.qubit2 = q2.row; op.qubit3 = q3.row; }
           } else if (gateName === "ccx") {
             const c1 = group.find(g => g.gate.props?.multiQubitRole === "control1");
             const c2 = group.find(g => g.gate.props?.multiQubitRole === "control2");
             const t = group.find(g => g.gate.props?.multiQubitRole === "target");
             if (c1 && c2 && t) { op.control1 = c1.row; op.control2 = c2.row; op.target = t.row; }
           } else if (gateName === "cswap") {
             const c = group.find(g => g.gate.props?.multiQubitRole === "control");
             const t1 = group.find(g => g.gate.props?.multiQubitRole === "target1");
             const t2 = group.find(g => g.gate.props?.multiQubitRole === "target2");
             if (c && t1 && t2) { op.control = c.row; op.target1 = t1.row; op.target2 = t2.row; }
           }
           if (Object.keys(op).length > 1) {
             operationsList.push({ op, sortCol });
           }
         }
      } else {
        const controlNode = group.find(g => g.gate.props?.multiQubitRole === "control" || g.gate.props?.multiQubitRole === "qubit1");
        const targetNode = group.find(g => g.gate.props?.multiQubitRole === "target" || g.gate.props?.multiQubitRole === "qubit2");
        
        if (controlNode && targetNode) {
          if (["swap", "cz", "cp"].includes(gateName)) {
             op.qubit1 = controlNode.row;
             op.qubit2 = targetNode.row;
             
             if (gateName === "cp") {
               op.theta = (controlNode.gate.props?.angle !== undefined ? controlNode.gate.props.angle : 0) * (Math.PI / 180);
             }
          } else {
             op.control = controlNode.row;
             op.target = targetNode.row;
          }
          operationsList.push({ op, sortCol });
        }
      }
    }
  }

  // 4. Sort operations by their designated column
  operationsList.sort((a, b) => a.sortCol - b.sortCol);

  const operations = operationsList.map(item => item.op);

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

  for (const op of operations) {
    const gateKey = Object.keys(GATE_REGISTRY).find(
      (k) => k.toLowerCase() === op.gate.toLowerCase(),
    );

    const type = gateKey ? GATE_REGISTRY[gateKey] : GATE_REGISTRY["I"];

    if (["cx", "cy", "cz", "ch", "cp", "swap"].includes(op.gate.toLowerCase())) {
      const multiQubitId = crypto.randomUUID();
      
      const isQubitPairs = ["swap", "cz", "cp"].includes(op.gate.toLowerCase());
      const controlRow = isQubitPairs ? op.qubit1 : op.control;
      const targetRow = isQubitPairs ? op.qubit2 : op.target;
      
      if (controlRow !== undefined && targetRow !== undefined) {
        grid[controlRow].push({
          id: crypto.randomUUID(),
          type,
          props: { 
            multiQubitId, 
            multiQubitRole: isQubitPairs ? "qubit1" : "control",
            ...(op.gate.toLowerCase() === "cp" && op.theta !== undefined ? { angle: op.theta * (180 / Math.PI) } : {})
          },
        });
        
        grid[targetRow].push({
          id: crypto.randomUUID(),
          type,
          props: { 
            multiQubitId, 
            multiQubitRole: isQubitPairs ? "qubit2" : "target",
            ...(op.gate.toLowerCase() === "cp" && op.theta !== undefined ? { angle: op.theta * (180 / Math.PI) } : {})
          },
        });
      }
    } else if (["ccx", "ccz", "cswap"].includes(op.gate.toLowerCase())) {
      const multiQubitId = crypto.randomUUID();
      const gateName = op.gate.toLowerCase();
      
      let row0, row1, row2;
      let role0: any, role1: any, role2: any;
      
      if (gateName === "ccz") {
        row0 = op.qubit1; row1 = op.qubit2; row2 = op.qubit3;
        role0 = "qubit1"; role1 = "qubit2"; role2 = "qubit3";
      } else if (gateName === "ccx") {
        row0 = op.control1; row1 = op.control2; row2 = op.target;
        role0 = "control1"; role1 = "control2"; role2 = "target";
      } else if (gateName === "cswap") {
        row0 = op.control; row1 = op.target1; row2 = op.target2;
        role0 = "control"; role1 = "target1"; role2 = "target2";
      }
      
      if (row0 !== undefined && row1 !== undefined && row2 !== undefined) {
         grid[row0].push({ id: crypto.randomUUID(), type, props: { multiQubitId, multiQubitRole: role0 } });
         grid[row1].push({ id: crypto.randomUUID(), type, props: { multiQubitId, multiQubitRole: role1 } });
         grid[row2].push({ id: crypto.randomUUID(), type, props: { multiQubitId, multiQubitRole: role2 } });
      }
    } else {
      const primaryRow = op.qubit ?? op.control ?? op.control1 ?? 0;
      grid[primaryRow].push({
        id: crypto.randomUUID(),
        type,
        props: {
          angle: op.theta !== undefined ? op.theta * (180 / Math.PI) : undefined,
          phi: op.phi !== undefined ? op.phi * (180 / Math.PI) : undefined,
          lambda: op.lambda !== undefined ? op.lambda * (180 / Math.PI) : undefined,
          classicalBit: op.bit,
        },
      });
    }
  }

  return grid;
}
