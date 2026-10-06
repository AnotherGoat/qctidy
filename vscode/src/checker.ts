import { spawn } from 'child_process';
import { CircuitBuild } from './circuit';

/** A diagnostic reported by the `qctidy check` JSON report. */
export interface CheckDiagnostic {
    filename: string;
    rule: string;
    group: string;
    message: string;
    /** Qubit (row) and time step (column) pairs affected by the detection. */
    positions: { row: number; column: number }[];
    /** Indices of the operations affected by the detection, usable with the source map. */
    operations: number[];
}

export interface CheckOptions {
    /** Path to the `qctidy` executable. Defaults to `qctidy` from `PATH`. */
    executable?: string;
    /** Name reported for the standard input source, instead of `<stdin>`. */
    inputName?: string;
}

/**
 * Run `qctidy check` on a built circuit and return its diagnostics.
 *
 * Resolves for exit code 0 (clean) and 1 (detections found), and rejects with
 * the CLI error message for exit code 2 or when the process can't be spawned.
 */
export function runCheck(build: CircuitBuild, options: CheckOptions = {}): Promise<CheckDiagnostic[]> {
    const executable = options.executable ?? 'qctidy';
    const args = [
        'check',
        '--input', '-',
        '--input-format', 'json',
        '--output-format', 'json',
        '--color', 'never'
    ];

    if (options.inputName) {
        args.push('--input-name', options.inputName);
    }

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

            // Exit code 1 means detections were found, which is not a failure.
            if (code !== 0 && code !== 1) {
                reject(new Error(stderr.trim() || `qctidy exited with code ${code}`));
                return;
            }

            try {
                resolve(JSON.parse(stdout) as CheckDiagnostic[]);
            } catch (error) {
                reject(new Error(`Failed to parse qctidy output: ${String(error)}`));
            }
        });

        child.stdin.end(JSON.stringify(build.circuit));
    });
}
