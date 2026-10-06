import { execFile } from 'child_process';
import * as fs from 'fs';
import * as path from 'path';
import * as vscode from 'vscode';

const PYTHON_EXTENSION_ID = 'ms-python.python';
const FALLBACK_INTERPRETERS = ['python3', 'python', 'py'];
const VENV_NAMES = ['.venv', 'venv', 'env'];
const VERSION_SCRIPT = 'import qiskit; print(qiskit.__version__)';
const TIMEOUT_MS = 5000;

let outputChannel: vscode.OutputChannel | undefined;

/** Output channel that explains how the Qiskit version was detected. */
export function qiskitOutputChannel(): vscode.OutputChannel {
    outputChannel ??= vscode.window.createOutputChannel('QCTidy');
    return outputChannel;
}

/** Interpreter configured in the Python extension, if any. */
async function interpreterFromPythonExtension(resource?: vscode.Uri): Promise<string | null> {
    const extension = vscode.extensions.getExtension(PYTHON_EXTENSION_ID);
    if (!extension) {
        return null;
    }

    try {
        const api = await extension.activate() as any;

        const command = api?.settings?.getExecutionDetails?.(resource)?.execCommand?.[0];
        if (typeof command === 'string' && command.length > 0) {
            return command;
        }

        const activePath = api?.environments?.getActiveEnvironmentPath?.(resource)?.path;
        if (typeof activePath === 'string' && activePath.length > 0) {
            return activePath;
        }
    } catch (error) {
        qiskitOutputChannel().appendLine(`Python extension error: ${String(error)}`);
    }

    return null;
}

/** Interpreter paths of virtual environments near the resource and the workspace. */
function venvInterpreterCandidates(resource?: vscode.Uri): string[] {
    const roots = new Set<string>();

    if (resource) {
        roots.add(path.dirname(resource.fsPath));

        const folder = vscode.workspace.getWorkspaceFolder(resource);
        if (folder) {
            roots.add(folder.uri.fsPath);
        }
    }

    for (const folder of vscode.workspace.workspaceFolders ?? []) {
        roots.add(folder.uri.fsPath);
    }

    const binaries = process.platform === 'win32'
        ? [path.join('Scripts', 'python.exe')]
        : [path.join('bin', 'python'), path.join('bin', 'python3')];

    const candidates: string[] = [];
    for (const root of roots) {
        for (const name of VENV_NAMES) {
            for (const binary of binaries) {
                candidates.push(path.join(root, name, binary));
            }
        }
    }

    return candidates;
}

function runVersion(interpreter: string): Promise<string | null> {
    return new Promise((resolve) => {
        execFile(
            interpreter,
            ['-c', VERSION_SCRIPT],
            { timeout: TIMEOUT_MS, windowsHide: true },
            (error, stdout, stderr) => {
                if (error) {
                    const detail = stderr.trim() || error.message;
                    qiskitOutputChannel().appendLine(`  ${interpreter}: ${detail.split('\n')[0]}`);
                    resolve(null);
                    return;
                }

                const version = stdout.trim();
                resolve(version.length > 0 ? version : null);
            }
        );
    });
}

/**
 * Best-effort detection of the Qiskit version available to the workspace.
 *
 * Uses the interpreter configured in the Python extension when available, then
 * a virtual environment near the file, and finally the usual PATH executables.
 */
export async function detectQiskitVersion(resource?: vscode.Uri): Promise<string | null> {
    const log = qiskitOutputChannel();
    log.appendLine(`Detecting Qiskit for ${resource?.fsPath ?? '(no resource)'}`);

    const configured = await interpreterFromPythonExtension(resource);
    if (configured) {
        log.appendLine(`Python extension interpreter: ${configured}`);
        const version = await runVersion(configured);
        if (version) {
            log.appendLine(`Qiskit ${version}`);
            return version;
        }

        log.appendLine('Qiskit not found in the configured interpreter; looking for a virtual environment.');
    } else {
        log.appendLine('No interpreter from the Python extension; looking for a virtual environment.');
    }

    for (const candidate of venvInterpreterCandidates(resource)) {
        if (configured && path.resolve(candidate) === path.resolve(configured)) {
            continue;
        }

        if (!fs.existsSync(candidate)) {
            continue;
        }

        const version = await runVersion(candidate);
        if (version) {
            log.appendLine(`Qiskit ${version} (${candidate})`);
            return version;
        }
    }

    log.appendLine('Looking for Python in the PATH.');

    for (const candidate of FALLBACK_INTERPRETERS) {
        const version = await runVersion(candidate);
        if (version) {
            log.appendLine(`Qiskit ${version} (${candidate})`);
            return version;
        }
    }

    log.appendLine('Qiskit not found.');
    return null;
}

/** Notify when the configured Python interpreter may have changed. */
export function watchInterpreter(listener: () => void): vscode.Disposable[] {
    const disposables: vscode.Disposable[] = [
        vscode.workspace.onDidChangeConfiguration((event) => {
            if (event.affectsConfiguration('python')) {
                listener();
            }
        })
    ];

    const extension = vscode.extensions.getExtension(PYTHON_EXTENSION_ID);
    if (extension) {
        void Promise.resolve(extension.activate())
            .then((api) => {
                const event = (api as any)?.environments?.onDidChangeActiveEnvironmentPath;
                if (typeof event === 'function') {
                    disposables.push(event(() => listener()));
                }
            })
            .catch(() => {
                // The Python extension could not be activated; the fallbacks still apply.
            });
    }

    return disposables;
}
