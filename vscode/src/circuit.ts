/**
 * Circuit JSON schema accepted by the QCTidy converter (format version 1).
 *
 * This mirrors `crates/converter/src/formats/json.rs` so that a parsed circuit
 * can be sent to the `qctidy` CLI without any extra conversion step.
 */
export interface Circuit {
    version: number;
    qubit_count: number;
    operations: CircuitOperation[];
}

export type CircuitOperation =
    | { gate: 'id' | 'h' | 'x' | 'y' | 'z' | 's' | 'sdg' | 'sx' | 'sy' | 't' | 'tdg'; qubit: number }
    | { gate: 'p' | 'rx' | 'ry'; theta: number; qubit: number }
    | { gate: 'rz'; phi: number; qubit: number }
    | { gate: 'u'; theta: number; phi: number; lambda: number; qubit: number }
    | { gate: 'measure'; qubit: number; bit: number }
    | { gate: 'swap' | 'cz'; qubit1: number; qubit2: number }
    | { gate: 'cp'; theta: number; qubit1: number; qubit2: number }
    | { gate: 'ch' | 'cx' | 'cy'; control: number; target: number }
    | { gate: 'cswap'; control: number; target1: number; target2: number }
    | { gate: 'ccx'; control1: number; control2: number; target: number }
    | { gate: 'ccz'; qubit1: number; qubit2: number; qubit3: number };

/** Maps an operation index to the source range of the gate call that produced it. */
export interface OperationLocation {
    index: number;
    line: number;
    column: number;
    endLine: number;
    endColumn: number;
}

/** A checkable circuit and its operation index to source range mapping. */
export interface CircuitBuild {
    circuit: Circuit;
    sourceMap: OperationLocation[];
}

/** A problem that prevents a circuit from being checked. */
export interface CircuitIssue {
    message: string;
    line: number;
    column: number;
}

export interface GateParameter {
    name: string;
    value: string;
    /** Source position of the argument that holds this parameter. */
    line: number;
    column: number;
}

export interface ParsedGate {
    gateName: string;
    displayName: string;
    description: string;
    line: number;
    column: number;
    endLine: number;
    endColumn: number;
    parameters: GateParameter[];
    /** Canonical operation, or `null` when the gate can't be checked. */
    operation: CircuitOperation | null;
    /** Why the gate can't be checked, when `operation` is `null`. */
    reason?: string;
}

export interface ParsedCircuit {
    variableName: string;
    qubits: string;
    clbits: string;
    line: number;
    column: number;
    gates: ParsedGate[];
}

const PI_PATTERN = /\b(?:numpy|np|math)\.pi\b/g;
const BARE_PI_PATTERN = /\bpi\b/g;
const SAFE_EXPRESSION_PATTERN = /^[\s\d+\-*/().eEPI]+$/;
const INTEGER_PATTERN = /^\d+$/;

/**
 * Evaluate a numeric Python expression, resolving `pi` constants.
 *
 * Returns `null` for anything that can't be resolved statically, such as
 * variables, function calls or attribute accesses.
 */
export function evaluateNumber(text: string): number | null {
    const normalized = text
        .replace(PI_PATTERN, 'PI')
        .replace(BARE_PI_PATTERN, 'PI');

    if (!SAFE_EXPRESSION_PATTERN.test(normalized)) {
        return null;
    }

    try {
        // Note: the pattern above whitelists every character of the expression,
        // so only arithmetic and pi constants can reach the evaluator.
        const expression = normalized.replace(/PI/g, `(${Math.PI})`);
        const value = Function(`"use strict";return (${expression});`)();
        return typeof value === 'number' && Number.isFinite(value) ? value : null;
    } catch {
        return null;
    }
}

/** Evaluate a non-negative integer, such as a qubit index. */
export function evaluateInteger(text: string): number | null {
    const trimmed = text.trim();
    return INTEGER_PATTERN.test(trimmed) ? Number.parseInt(trimmed, 10) : null;
}

/** Every problem that prevents the circuit from being checked. */
export function circuitIssues(parsed: ParsedCircuit): CircuitIssue[] {
    const issues: CircuitIssue[] = [];

    if (evaluateInteger(parsed.qubits) === null) {
        issues.push({
            message: `qubit count is not a number: ${parsed.qubits}`,
            line: parsed.line,
            column: parsed.column
        });
    }

    if (evaluateInteger(parsed.clbits) === null) {
        issues.push({
            message: `classical bit count is not a number: ${parsed.clbits}`,
            line: parsed.line,
            column: parsed.column
        });
    }

    for (const gate of parsed.gates) {
        if (gate.operation === null) {
            issues.push({
                message: gate.reason ?? `unsupported gate: ${gate.gateName}`,
                line: gate.line,
                column: gate.column
            });
        }
    }

    return issues;
}

/** Whether the circuit can be serialized to the expected schema. */
export function isCheckable(parsed: ParsedCircuit): boolean {
    return circuitIssues(parsed).length === 0;
}

/**
 * Build the circuit JSON and its operation index to source range mapping.
 *
 * Returns `null` when the circuit can't be checked, because a partial circuit
 * could hide gates and make the CLI detect patterns that aren't there.
 */
export function toCircuit(parsed: ParsedCircuit): CircuitBuild | null {
    const qubitCount = evaluateInteger(parsed.qubits);

    if (qubitCount === null || !isCheckable(parsed)) {
        return null;
    }

    const operations: CircuitOperation[] = [];
    const sourceMap: OperationLocation[] = [];

    for (const gate of parsed.gates) {
        if (!gate.operation) {
            return null;
        }

        sourceMap.push({
            index: operations.length,
            line: gate.line,
            column: gate.column,
            endLine: gate.endLine,
            endColumn: gate.endColumn
        });
        operations.push(gate.operation);
    }

    return {
        circuit: { version: 1, qubit_count: qubitCount, operations },
        sourceMap
    };
}
