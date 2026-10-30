import * as vscode from 'vscode';
import { cliOutputChannel, detectQctidyVersion, invalidateQctidyCache, runCheck } from './cli';
import { convertReportToVsCodeDiagnostics } from './diagnostics';
import { applyFixes, registerFixActions, rememberAnalyzedSource } from './fixes';
import { detectQiskitVersion, qiskitOutputChannel, watchInterpreter } from './qiskit';
import { AstNodeItem, QCTidyTreeDataProvider } from './treeDataProvider';

let diagnosticCollection: vscode.DiagnosticCollection | undefined;
let debounceTimer: NodeJS.Timeout | undefined;
const DEBOUNCE_DELAY_MS = 400;

export async function activate(context: vscode.ExtensionContext) {
    console.log('QCTidy VS Code extension is now active!');

    try {
        // 1. VS Code diagnostic collection to report editor diagnostics
        diagnosticCollection = vscode.languages.createDiagnosticCollection('qctidy');
        context.subscriptions.push(diagnosticCollection);
        registerFixActions(context);

        // 2. Register sidebar tree view
        const treeDataProvider = new QCTidyTreeDataProvider();
        const treeView = vscode.window.createTreeView('qctidy-ast-view', { treeDataProvider });
        context.subscriptions.push(treeView);

        // Unified output channel
        context.subscriptions.push(cliOutputChannel());

        // Central function to update editor diagnostics and sidebar tree data
        const updateDiagnosticsAndSidebar = async (document: vscode.TextDocument): Promise<void> => {
            if (document.languageId !== 'python') {
                return;
            }

            const status = await detectQctidyVersion();
            if (!status.available) {
                diagnosticCollection?.delete(document.uri);
                return;
            }

            const source = document.getText();
            const inputPath = document.uri.fsPath;

            try {
                const report = await runCheck(source, inputPath);
                rememberAnalyzedSource(document.uri, source);

                // Convert and publish diagnostics to the source editor
                const vsDiagnostics = convertReportToVsCodeDiagnostics(report);
                diagnosticCollection?.set(document.uri, vsDiagnostics);

                // Fused AST + diagnostics in the sidebar
                treeDataProvider.setData(report, source);
            } catch (error) {
                console.error('Failed to update diagnostics and AST:', error);
            }
        };

        // QCTidy CLI detection
        const checkQctidyStatus = async (showError = true): Promise<void> => {
            const status = await detectQctidyVersion();
            await vscode.commands.executeCommand('setContext', 'qctidy.available', status.available);
            treeDataProvider.setQctidyStatus(status);

            if (!status.available && showError) {
                vscode.window.showErrorMessage(
                    `QCTidy CLI not found (${status.executable}). Please verify that QCTidy is installed or configure "qctidy.path" in settings.`,
                    'Open Settings'
                ).then((action) => {
                    if (action === 'Open Settings') {
                        vscode.commands.executeCommand('workbench.action.openSettings', 'qctidy.path');
                    }
                });
            } else if (status.available && vscode.window.activeTextEditor?.document.languageId === 'python') {
                void updateDiagnosticsAndSidebar(vscode.window.activeTextEditor.document);
            }
        };

        void checkQctidyStatus(true);

        // Re-detect when the configured QCTidy path changes
        context.subscriptions.push(
            vscode.workspace.onDidChangeConfiguration((event) => {
                if (event.affectsConfiguration('qctidy.path')) {
                    invalidateQctidyCache();
                    void checkQctidyStatus(true);
                }
            })
        );

        // Detect Qiskit version (best effort) and display in the sidebar
        let qiskitResource: string | undefined;

        const refreshQiskitVersion = (): void => {
            const resource = vscode.window.activeTextEditor?.document.uri
                ?? vscode.workspace.workspaceFolders?.[0]?.uri;
            qiskitResource = resource?.fsPath;
            void detectQiskitVersion(resource).then(version => treeDataProvider.setQiskitVersion(version));
        };
        refreshQiskitVersion();

        // Re-detect when the configured Python interpreter changes
        context.subscriptions.push(...watchInterpreter(refreshQiskitVersion));

        // 3. Reactivity on active editor changes and text edits with debounce
        vscode.window.onDidChangeActiveTextEditor((editor) => {
            if (editor && editor.document.languageId === 'python') {
                void updateDiagnosticsAndSidebar(editor.document);
            } else {
                treeDataProvider.refresh();
            }

            // If no active editor was available on activation to resolve interpreter, retry now
            if (qiskitResource === undefined) {
                refreshQiskitVersion();
            }
        });

        // Debounce text document edits before re-running diagnostics
        context.subscriptions.push(
            vscode.workspace.onDidChangeTextDocument((event) => {
                const activeEditor = vscode.window.activeTextEditor;
                if (activeEditor && event.document === activeEditor.document && event.document.languageId === 'python') {
                    if (debounceTimer) {
                        clearTimeout(debounceTimer);
                    }
                    debounceTimer = setTimeout(() => {
                        void updateDiagnosticsAndSidebar(event.document);
                    }, DEBOUNCE_DELAY_MS);
                }
            })
        );

        // Clear diagnostics when a text document is closed
        context.subscriptions.push(
            vscode.workspace.onDidCloseTextDocument((document) => {
                diagnosticCollection?.delete(document.uri);
            })
        );

        // Initial analysis if an editor is already active
        if (vscode.window.activeTextEditor && vscode.window.activeTextEditor.document.languageId === 'python') {
            void updateDiagnosticsAndSidebar(vscode.window.activeTextEditor.document);
        }

        // 4. Navigation and reload commands
        const jumpToLine = (line: number, column: number, endLine?: number, endColumn?: number): void => {
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                const start = new vscode.Position(line, column);
                const end = endLine !== undefined && endColumn !== undefined
                    ? new vscode.Position(endLine, endColumn)
                    : start;

                editor.selection = new vscode.Selection(start, end);
                editor.revealRange(new vscode.Range(start, end), vscode.TextEditorRevealType.InCenter);
            }
        };

        context.subscriptions.push(
            vscode.commands.registerCommand('qctidy.refresh', () => {
                invalidateQctidyCache();
                void checkQctidyStatus(false);
                treeDataProvider.reload();
                refreshQiskitVersion();
                if (vscode.window.activeTextEditor && vscode.window.activeTextEditor.document.languageId === 'python') {
                    void updateDiagnosticsAndSidebar(vscode.window.activeTextEditor.document);
                }
            }),
            vscode.commands.registerCommand('qctidy.jumpToLine', jumpToLine),
            vscode.commands.registerCommand('qctidy.applyFix', async (item?: AstNodeItem) => {
                const editor = vscode.window.activeTextEditor;

                if (!editor || editor.document.languageId !== 'python' || !item?.fixes?.length) {
                    return;
                }

                await applyFixes(editor.document, item.fixes);
            }),
            vscode.commands.registerCommand('qctidy.fixAll', async () => {
                const editor = vscode.window.activeTextEditor;

                if (!editor || editor.document.languageId !== 'python') {
                    return;
                }

                await applyFixes(editor.document, treeDataProvider.allFixes());
            }),
            vscode.commands.registerCommand('qctidy.openNode', async (key: string, line: number, column: number, endLine?: number, endColumn?: number) => {
                const node = treeDataProvider.findByKey(key);

                if (node && node.collapsibleState !== vscode.TreeItemCollapsibleState.None) {
                    try {
                        await treeView.reveal(node, { expand: true });
                    } catch {
                        // The node may be outdated after a refresh; jumping to line is still helpful.
                    }
                }

                jumpToLine(line, column, endLine, endColumn);
            })
        );
    } catch (error) {
        vscode.window.showErrorMessage('Failed to initialize QCTidy: ' + String(error));
    }
}

export function deactivate() {
    if (debounceTimer) {
        clearTimeout(debounceTimer);
    }
    if (diagnosticCollection) {
        diagnosticCollection.clear();
        diagnosticCollection.dispose();
    }
    invalidateQctidyCache();
}
