import { spawn } from 'child_process';

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

export interface AstOptions {
    /** Path to the `qctidy` executable. Defaults to `qctidy` from `PATH`. */
    executable?: string;
}

/**
 * Run `qctidy ast` on a Python source file and return its analysis.
 *
 * Rejects when the process can't be spawned or exits with a non-zero code.
 */
export function runAst(source: string, options: AstOptions = {}): Promise<AstAnalysis> {
    const executable = options.executable ?? 'qctidy';
    const args = ['ast', '--color', 'never'];

    return new Promise((resolve, reject) => {
        const child = spawn(executable, args);

        let stdout = '';
        let stderr = '';
        let spawnError: Error | null = null;

        child.stdout.setEncoding('utf8');
        child.stderr.setEncoding('utf8');

        child.stdout.on('data', (chunk: string) => {
            stdout += chunk;
        });
        child.stderr.on('data', (chunk: string) => {
            stderr += chunk;
        });
        child.on('error', (error: Error) => {
            spawnError = error;
        });
        child.on('close', (code: number | null) => {
            if (spawnError) {
                reject(spawnError);
                return;
            }

            if (code !== 0) {
                reject(new Error(stderr.trim() || `qctidy exited with code ${code}`));
                return;
            }

            try {
                resolve(JSON.parse(stdout) as AstAnalysis);
            } catch (error) {
                reject(new Error(`Failed to parse qctidy output: ${String(error)}`));
            }
        });

        child.stdin.end(source);
    });
}
