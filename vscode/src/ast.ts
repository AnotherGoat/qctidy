
/**
 * The JSON emitted by `qctidy ast`.
 *
 * These types mirror the `qctidy ast` output so the tree view can be built
 * without parsing the Python source in the extension.
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
    qubits: string;
    clbits: string;
    line: number;
    column: number;
    issues: AstIssue[];
    gates: AstGate[];
    source_map: SourceLocation[] | null;
}

export type AstChild = AstScope | AstCircuit;

export interface AstAnalysis {
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

export { runAst } from './cli';

