/** Gate keys for two-qubit gates drawn as a control/target or qubit pair. */
export const TWO_QUBIT_GATES = ["CX", "CY", "CZ", "CH", "CP", "SWAP"] as const;

/** Gate keys for three-qubit gates. */
export const THREE_QUBIT_GATES = ["CCX", "CCZ", "CSWAP"] as const;

/** Two-qubit gates that the JSON format stores as `qubit1`/`qubit2`. */
export const QUBIT_PAIR_GATES = ["SWAP", "CZ", "CP"] as const;

export const TWO_QUBIT_GATE_NAMES = TWO_QUBIT_GATES.map((gate) =>
  gate.toLowerCase(),
);
export const THREE_QUBIT_GATE_NAMES = THREE_QUBIT_GATES.map((gate) =>
  gate.toLowerCase(),
);
export const QUBIT_PAIR_GATE_NAMES = QUBIT_PAIR_GATES.map((gate) =>
  gate.toLowerCase(),
);
