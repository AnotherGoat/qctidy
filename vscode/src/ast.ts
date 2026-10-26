
/**
 * The Python analysis emitted in the `analyses` array of
 * `qctidy check --output-format json`.
 *
 * These types mirror that payload so the tree view can be built without parsing
 * the Python source in the extension.
 */

export type ScopeType = 'function' | 'class';

export interface AstScope {
    type: ScopeType;
    name: string;
    line: number;
    column: number;
    children: AstChild[];
}

export interface AstCircuit {
    type: 'circuit';
    name: string;
    qubit_count: string;
    bit_count: string;
    line: number;
    column: number;
    issues: AstIssue[];
    gates: AstGate[];
    source_map: SourceLocation[] | null;
}

export type AstChild = AstScope | AstCircuit;

export interface AstAnalysis {
    filename: string;
    scopes: AstScope[];
    module_circuits: AstCircuit[];
}

export interface AstGate {
    display_name: string;
    description: string;
    line: number;
    column: number;
    end_line: number;
    end_column: number;
    parameters: AstParameter[];
}

export interface AstParameter {
    name: string;
    value: string;
    line: number;
    column: number;
}

export interface AstIssue {
    message: string;
    line: number;
    column: number;
}

export interface SourceLocation {
    index: number;
    line: number;
    column: number;
    end_line: number;
    end_column: number;
}

