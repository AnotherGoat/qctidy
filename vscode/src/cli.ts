import { spawn } from 'child_process';
import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';
import * as vscode from 'vscode';

import { qiskitOutputChannel } from './qiskit';

const TIMEOUT_MS = 5000;

/** Output channel dedicated to QCTidy CLI and operations. */
export function cliOutputChannel(): vscode.OutputChannel {
    return qiskitOutputChannel();
}

export interface ResolvedExecutable {
    path: string;
    exists: boolean;
    isExplicit: boolean;
}

/**
 * Resolves the configured `qctidy.path`.
 *
 * If the path contains separators (`/` or `\`), it is treated as an explicit path,
 * expanding `~` to the home directory and resolving relative paths against the workspace.
 * On Windows, `.exe` is checked for existence without forcing the user to hardcode it.
 *
 * If there are no separators, it is treated as a command to be resolved via `PATH`.
 */
export function resolveQctidyExecutable(): ResolvedExecutable {
    const config = vscode.workspace.getConfiguration('qctidy');
    const rawPath = config.get<string>('path')?.trim() || 'qctidy';

    const hasSeparators = rawPath.includes('/') || rawPath.includes('\\');
    if (!hasSeparators) {
        return { path: rawPath, exists: true, isExplicit: false };
    }

    let resolved = rawPath;
    if (resolved === '~') {
        resolved = os.homedir();
    } else if (resolved.startsWith('~/') || resolved.startsWith('~\\')) {
        resolved = path.join(os.homedir(), resolved.slice(2));
    }

    if (!path.isAbsolute(resolved)) {
        const root = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
        resolved = root ? path.resolve(root, resolved) : path.resolve(resolved);
    }

    let exists = fs.existsSync(resolved);
    if (!exists && process.platform === 'win32') {
        const ext = path.extname(resolved).toLowerCase();
        if (!ext && fs.existsSync(`${resolved}.exe`)) {
            exists = true;
        }
    }

    return { path: resolved, exists, isExplicit: true };
}

export interface QctidyStatus {
    available: boolean;
    version?: string;
    executable: string;
    error?: string;
}

let cachedStatus: QctidyStatus | null = null;
let statusPromise: Promise<QctidyStatus> | null = null;

/** Invalidate the cached status of QCTidy. */
export function invalidateQctidyCache(): void {
    cachedStatus = null;
    statusPromise = null;
}

function probeVersion(executable: string): Promise<{ success: boolean; version?: string; error?: string }> {
    return new Promise((resolve) => {
        const child = spawn(executable, ['--version'], {
            shell: false,
            windowsHide: true,
        });

        let stdout = '';
        let stderr = '';
        let spawnError: Error | null = null;
        let timedOut = false;

        const timer = setTimeout(() => {
            timedOut = true;
            child.kill();
        }, TIMEOUT_MS);

        child.stdout?.setEncoding('utf8');
        child.stderr?.setEncoding('utf8');

        child.stdout?.on('data', (data: string) => {
            stdout += data;
        });

        child.stderr?.on('data', (data: string) => {
            stderr += data;
        });

        child.on('error', (err: Error) => {
            spawnError = err;
        });

        child.on('close', (code: number | null) => {
            clearTimeout(timer);
            if (timedOut) {
                resolve({ success: false, error: `Command timed out after ${TIMEOUT_MS}ms` });
                return;
            }
            if (spawnError) {
                resolve({ success: false, error: spawnError.message });
                return;
            }
            if (code !== 0) {
                resolve({ success: false, error: stderr.trim() || `Exit code ${code}` });
                return;
            }
            resolve({ success: true, version: stdout.trim() });
        });
    });
}

/**
 * Detects the configured QCTidy executable and tests availability with `qctidy --version`.
 *
 * Employs a timeout and caching mechanism similar to `qiskit.ts`.
 */
export async function detectQctidyVersion(forceRefresh = false): Promise<QctidyStatus> {
    if (!forceRefresh && cachedStatus !== null) {
        return cachedStatus;
    }
    if (statusPromise !== null) {
        return statusPromise;
    }

    statusPromise = (async () => {
        const log = cliOutputChannel();
        const resolved = resolveQctidyExecutable();
        log.appendLine(`Detecting QCTidy CLI: ${resolved.path} (explicit path: ${resolved.isExplicit})`);

        if (resolved.isExplicit && !resolved.exists) {
            const errorMsg = `Explicit path does not exist: ${resolved.path}`;
            log.appendLine(`  ${errorMsg}`);
            cachedStatus = {
                available: false,
                executable: resolved.path,
                error: errorMsg,
            };
            statusPromise = null;
            return cachedStatus;
        }

        const probe = await probeVersion(resolved.path);
        if (probe.success && probe.version) {
            log.appendLine(`  QCTidy version: ${probe.version}`);
            cachedStatus = {
                available: true,
                version: probe.version,
                executable: resolved.path,
            };
        } else {
            const errorMsg = probe.error ?? 'Unknown error';
            log.appendLine(`  QCTidy not available: ${errorMsg}`);
            cachedStatus = {
                available: false,
                executable: resolved.path,
                error: errorMsg,
            };
        }

        statusPromise = null;
        return cachedStatus;
    })();

    return statusPromise;
}

import type { CheckReport } from './diagnostics';

export interface ExecuteOptions {
    stdin?: string;
    timeoutMs?: number;
    validExitCodes?: number[];
}

/**
 * Executes QCTidy with an argument array without a shell (supporting paths with spaces),
 * and connects stdin when provided.
 */
export async function executeQctidy(args: string[], options: ExecuteOptions = {}): Promise<string> {
    const status = await detectQctidyVersion();
    if (!status.available) {
        throw new Error(`QCTidy CLI is not available: ${status.error || 'Executable not found'}`);
    }

    const executable = status.executable;
    const timeout = options.timeoutMs ?? 10000;
    const validExitCodes = options.validExitCodes ?? [0];

    return new Promise((resolve, reject) => {
        const child = spawn(executable, args, {
            shell: false,
            windowsHide: true,
        });

        let stdout = '';
        let stderr = '';
        let spawnError: Error | null = null;
        let timedOut = false;

        const timer = setTimeout(() => {
            timedOut = true;
            child.kill();
        }, timeout);

        child.stdout.setEncoding('utf8');
        child.stderr.setEncoding('utf8');

        child.stdout.on('data', (chunk: string) => {
            stdout += chunk;
        });

        child.stderr.on('data', (chunk: string) => {
            stderr += chunk;
        });

        child.on('error', (err: Error) => {
            spawnError = err;
        });

        child.on('close', (code: number | null) => {
            clearTimeout(timer);
            if (timedOut) {
                reject(new Error(`qctidy process timed out after ${timeout}ms`));
                return;
            }
            if (spawnError) {
                reject(spawnError);
                return;
            }
            if (code === null || !validExitCodes.includes(code)) {
                reject(new Error(stderr.trim() || `qctidy exited with code ${code}`));
                return;
            }
            resolve(stdout);
        });

        if (options.stdin !== undefined) {
            child.stdin.end(options.stdin, 'utf8');
        } else {
            child.stdin.end();
        }
    });
}

/**
 * Runs `qctidy check --output-format json --color never --input-name <inputPath>`
 * passing `source` via stdin, accepting exit codes 0, 1, and 2.
 */
export async function runCheck(source: string, inputPath: string): Promise<CheckReport> {
    const stdout = await executeQctidy(
        ['check', '--output-format', 'json', '--color', 'never', '--input-name', inputPath],
        {
            stdin: source,
            validExitCodes: [0, 1, 2],
        }
    );
    try {
        return JSON.parse(stdout) as CheckReport;
    } catch (error) {
        throw new Error(`Failed to parse qctidy check output: ${String(error)}`);
    }
}

