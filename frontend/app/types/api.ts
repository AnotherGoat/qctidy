/** A single gate operation in the circuit JSON format. */
export interface CircuitOperation {
  gate: string;
  qubit?: number;
  qubit1?: number;
  qubit2?: number;
  qubit3?: number;
  control?: number;
  control1?: number;
  control2?: number;
  target?: number;
  target1?: number;
  target2?: number;
  theta?: number;
  phi?: number;
  lambda?: number;
  bit?: number;
}

export interface QCTidyRequest {
  circuit: {
    qubit_count: number;
    operations: CircuitOperation[];
  };
  iterations: number;
}

export interface QCTidyResponse {
  circuit: {
    version?: number;
    qubit_count: number;
    operations: CircuitOperation[];
  };
}

export interface EstimateCircuitResponse {
  estimates: {
    provider: string;
    status: string;
    estimates: {
      provider: string;
      plan_name: string;
      price_label: string;
      cost_usd: number | null;
    }[];
  }[];
}

export interface AnalyzeCircuitResponse {
  metrics: Record<string, number>;
}
