import { spawn, type ChildProcess } from 'child_process';
import { randomBytes } from 'crypto';
import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';
import * as vscode from 'vscode';

import { cliOutputChannel, executeQctidy } from './cli';
import type { AstNodeItem } from './treeDataProvider';

const DISPLAY_TIMEOUT_MS = 10000;
const PRESENT_TIMEOUT_MS = 20000;
const GRAPHVIZ_TIMEOUT_MS = 5000;
const TEXT_VIEW = 'text';
const GRAPH_VIEW = 'graph';

const GRAPHVIZ_MISSING_MESSAGE =
    "QCTidy: Graphviz ('dot') is not installed or not found in PATH. Install it from https://graphviz.org/download/ to use graph visualization.";

type PreviewKind = 'text' | 'graph';

interface CircuitPreview {
    kind: PreviewKind;
    title: string;
    circuitName: string;
    circuitLine: number;
    body: string;
}

interface CircuitTarget {
    source: string;
    inputPath: string;
    selector: string;
    name: string;
    line: number;
}

let graphvizAvailable = false;
let graphvizDetection: Promise<boolean> | null = null;

const panels = new Map<string, vscode.WebviewPanel>();
const panelPreviews = new Map<string, CircuitPreview>();

function probeGraphviz(): Promise<boolean> {
    return new Promise((resolve) => {
        let settled = false;
        let stderr = '';
        let timer: NodeJS.Timeout | undefined;
        let child: ChildProcess | undefined;

        const finish = (available: boolean): void => {
            if (settled) {
                return;
            }
            settled = true;
            if (timer) {
                clearTimeout(timer);
            }
            resolve(available);
        };

        try {
            child = spawn('dot', ['-V'], { shell: false, windowsHide: true });
        } catch {
            finish(false);
            return;
        }

        timer = setTimeout(() => {
            child?.kill();
            finish(false);
        }, GRAPHVIZ_TIMEOUT_MS);

        child.stderr?.setEncoding('utf8');
        child.stderr?.on('data', (chunk: string) => {
            stderr += chunk;
        });

        child.on('error', () => finish(false));
        child.on('close', (code: number | null) => finish(code === 0 || /graphviz/i.test(stderr)));
    });
}

/**
 * Detects whether the Graphviz `dot` binary is available in `PATH`.
 *
 * The result is cached in `graphvizAvailable` and reported in the QCTidy output channel.
 */
export function detectGraphviz(): Promise<boolean> {
    if (graphvizDetection === null) {
        graphvizDetection = probeGraphviz().then((available) => {
            graphvizAvailable = available;
            cliOutputChannel().appendLine(
                available
                    ? 'Graphviz detected: dot is available in PATH'
                    : 'Graphviz not detected: dot was not found in PATH'
            );
            return available;
        });
    }

    return graphvizDetection;
}

/** The last Graphviz detection result. */
export function isGraphvizAvailable(): boolean {
    return graphvizAvailable;
}

/** Closes every circuit preview panel and clears the deduplication state. */
export function disposeDisplayPanels(): void {
    for (const panel of panels.values()) {
        panel.dispose();
    }
    panels.clear();
    panelPreviews.clear();
}

/** Shows the selected circuit as plain text in a WebView panel. */
export async function showCircuitAsText(item?: AstNodeItem): Promise<void> {
    const target = resolveCircuitTarget(item);
    if (!target) {
        return;
    }

    try {
        const text = await executeQctidy(
            [
                'display',
                '--format', 'circuit',
                '--color', 'never',
                '--input-name', target.inputPath,
                '--circuit', target.selector,
            ],
            { stdin: target.source, timeoutMs: DISPLAY_TIMEOUT_MS }
        );

        showPreview(TEXT_VIEW, {
            kind: 'text',
            title: `QCTidy: ${target.name} (text)`,
            circuitName: target.name,
            circuitLine: target.line,
            body: text,
        });
    } catch (error) {
        reportError('display circuit as text', error);
    }
}

/** Shows the selected circuit as an inline SVG graph in a WebView panel. */
export async function showCircuitAsGraph(item?: AstNodeItem): Promise<void> {
    const target = resolveCircuitTarget(item);
    if (!target) {
        return;
    }

    const available = graphvizAvailable || (await detectGraphviz());
    if (!available) {
        void vscode.window.showErrorMessage(GRAPHVIZ_MISSING_MESSAGE);
        return;
    }

    try {
        const svg = await executeQctidy(
            [
                'present',
                '--format', 'svg',
                '--color', 'never',
                '--input-name', target.inputPath,
                '--circuit', target.selector,
            ],
            { stdin: target.source, timeoutMs: PRESENT_TIMEOUT_MS }
        );

        showPreview(GRAPH_VIEW, {
            kind: 'graph',
            title: `QCTidy: ${target.name} (graph)`,
            circuitName: target.name,
            circuitLine: target.line,
            body: extractInlineSvg(svg),
        });
    } catch (error) {
        reportError('render circuit as graph', error);
    }
}

function resolveCircuitTarget(item?: AstNodeItem): CircuitTarget | undefined {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'python' || !item || item.nodeType !== 'circuit') {
        return undefined;
    }

    const name = item.circuitName ?? item.label;
    const line = item.line + 1;

    return {
        source: editor.document.getText(),
        inputPath: editor.document.uri.fsPath,
        selector: `${name}:${line}`,
        name,
        line,
    };
}

function showPreview(viewType: string, preview: CircuitPreview): void {
    const key = `${viewType}:${preview.circuitName}:${preview.circuitLine}`;
    const existingPanel = panels.get(key);

    if (existingPanel) {
        panelPreviews.set(key, preview);
        existingPanel.title = preview.title;
        existingPanel.webview.html = buildHtml(preview);
        existingPanel.reveal();
        return;
    }

    const panel = vscode.window.createWebviewPanel(
        'qctidy.circuitPreview',
        preview.title,
        vscode.ViewColumn.Beside,
        { enableScripts: true }
    );

    panels.set(key, panel);
    panelPreviews.set(key, preview);
    panel.webview.html = buildHtml(preview);

    panel.webview.onDidReceiveMessage((message: unknown) => {
        if (!isSaveMessage(message)) {
            return;
        }

        const currentPreview = panelPreviews.get(key);
        if (currentPreview) {
            void savePreview(currentPreview);
        }
    });

    panel.onDidDispose(() => {
        panels.delete(key);
        panelPreviews.delete(key);
    });
}

function isSaveMessage(message: unknown): message is { command: 'save' } {
    return typeof message === 'object'
        && message !== null
        && (message as { command?: unknown }).command === 'save';
}

async function savePreview(preview: CircuitPreview): Promise<void> {
    const extension = preview.kind === 'graph' ? 'svg' : 'txt';
    const filters: Record<string, string[]> = preview.kind === 'graph'
        ? { 'SVG image': ['svg'] }
        : { 'Text file': ['txt'] };
    const fileName = `${sanitizeFileName(preview.circuitName)}-${preview.circuitLine}.${extension}`;

    const uri = await vscode.window.showSaveDialog({
        defaultUri: vscode.Uri.file(path.join(defaultSaveDirectory(), fileName)),
        filters,
        saveLabel: 'Save',
    });

    if (!uri) {
        return;
    }

    const targetPath = path.extname(uri.fsPath) ? uri.fsPath : `${uri.fsPath}.${extension}`;

    try {
        await writeFile(targetPath, preview.body);
        void vscode.window.showInformationMessage(`QCTidy: saved ${path.basename(targetPath)}`);
    } catch (error) {
        reportError('save circuit preview', error);
    }
}

function writeFile(filePath: string, contents: string): Promise<void> {
    return new Promise((resolve, reject) => {
        fs.writeFile(filePath, contents, 'utf8', (error) => {
            if (error) {
                reject(error);
            } else {
                resolve();
            }
        });
    });
}

function defaultSaveDirectory(): string {
    const activeDocument = vscode.window.activeTextEditor?.document;
    if (activeDocument && activeDocument.uri.scheme === 'file') {
        return path.dirname(activeDocument.uri.fsPath);
    }

    return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? os.homedir();
}

function sanitizeFileName(name: string): string {
    const sanitized = name.replace(/[^\w.-]+/g, '_').replace(/^_+|_+$/g, '');
    return sanitized || 'circuit';
}

function extractInlineSvg(svg: string): string {
    const match = /<svg[\s\S]*<\/svg>/i.exec(svg);
    return match ? match[0] : svg;
}

function reportError(action: string, error: unknown): void {
    const message = error instanceof Error ? error.message : String(error);
    cliOutputChannel().appendLine(`Failed to ${action}: ${message}`);
    void vscode.window.showErrorMessage(`QCTidy: failed to ${action}: ${message}`);
}

function escapeHtml(value: string): string {
    return value
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');
}

function buildHtml(preview: CircuitPreview): string {
    const nonce = randomBytes(16).toString('hex');
    const body = preview.kind === 'graph'
        ? `<div class="graph">${preview.body}</div>`
        : `<pre class="text">${escapeHtml(preview.body)}</pre>`;

    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'nonce-${nonce}';">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${escapeHtml(preview.title)}</title>
    <style>
        * {
            box-sizing: border-box;
        }

        html,
        body {
            height: 100%;
        }

        body {
            margin: 0;
            display: flex;
            flex-direction: column;
            font-family: var(--vscode-font-family);
            color: var(--vscode-editor-foreground);
            background: var(--vscode-editor-background);
        }

        body.text-view {
            background: #1e1e1e;
        }

        body.graph-view {
            background: #f3f3f3;
        }

        .toolbar {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 12px;
            padding: 10px 16px;
            border-bottom: 1px solid var(--vscode-panel-border);
            background: var(--vscode-sideBar-background);
        }

        .title {
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            font-weight: 600;
        }

        button {
            padding: 4px 14px;
            border: 1px solid var(--vscode-button-border, transparent);
            border-radius: 2px;
            background: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            font-family: inherit;
            cursor: pointer;
        }

        button:hover {
            background: var(--vscode-button-hoverBackground);
        }

        main {
            flex: 1;
            overflow: auto;
            padding: 16px;
        }

        pre.text {
            margin: 0;
            padding: 16px;
            overflow: auto;
            border-radius: 6px;
            background: #1e1e1e;
            color: #d4d4d4;
            font-family: Consolas, 'Courier New', monospace;
            font-size: 13px;
            line-height: 1.45;
            white-space: pre;
        }

        .graph {
            display: inline-block;
            padding: 16px;
            border-radius: 6px;
            background: #ffffff;
        }

        .graph svg {
            max-width: 100%;
            height: auto;
        }
    </style>
</head>
<body class="${preview.kind === 'graph' ? 'graph-view' : 'text-view'}">
    <header class="toolbar">
        <span class="title">${escapeHtml(preview.title)}</span>
        <button id="save" type="button">Save</button>
    </header>
    <main>${body}</main>
    <script nonce="${nonce}">
        const vscode = acquireVsCodeApi();
        document.getElementById('save').addEventListener('click', () => {
            vscode.postMessage({ command: 'save' });
        });
    </script>
</body>
</html>`;
}
