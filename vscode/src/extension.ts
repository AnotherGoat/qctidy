import * as vscode from 'vscode';
import { detectQiskitVersion, qiskitOutputChannel, watchInterpreter } from './qiskit';
import { QCTidyTreeDataProvider } from './treeDataProvider';

export async function activate(context: vscode.ExtensionContext) {
    console.log('QCTidy VS Code extension is now active!');

    try {
        // 1. Registrar el Sidebar (Tree View)
        const treeDataProvider = new QCTidyTreeDataProvider();
        const treeView = vscode.window.createTreeView('qctidy-ast-view', { treeDataProvider });
        context.subscriptions.push(treeView);

        // Detectar la versión de Qiskit (mejor esfuerzo) y mostrarla en la sidebar
        context.subscriptions.push(qiskitOutputChannel());

        let qiskitResource: string | undefined;

        const refreshQiskitVersion = (): void => {
            const resource = vscode.window.activeTextEditor?.document.uri
                ?? vscode.workspace.workspaceFolders?.[0]?.uri;
            qiskitResource = resource?.fsPath;
            void detectQiskitVersion(resource).then(version => treeDataProvider.setQiskitVersion(version));
        };
        refreshQiskitVersion();

        // Volver a detectar cuando cambie el intérprete configurado de Python
        context.subscriptions.push(...watchInterpreter(refreshQiskitVersion));

        // 2. Refrescar el sidebar cuando el usuario cambia de pestaña o edita el texto
        vscode.window.onDidChangeActiveTextEditor(() => {
            treeDataProvider.refresh();

            // Si al activar no había editor para resolver el intérprete, reintentar ahora
            if (qiskitResource === undefined) {
                refreshQiskitVersion();
            }
        });
        vscode.workspace.onDidChangeTextDocument(e => {
            if (vscode.window.activeTextEditor && e.document === vscode.window.activeTextEditor.document) {
                treeDataProvider.refresh();
            }
        });

        // 3. Comandos de navegación
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
                treeDataProvider.reload();
                refreshQiskitVersion();
            }),
            vscode.commands.registerCommand('qctidy.jumpToLine', jumpToLine),
            // Al hacer click, además se expande el nodo si puede
            vscode.commands.registerCommand('qctidy.openNode', async (key: string, line: number, column: number, endLine?: number, endColumn?: number) => {
                const node = treeDataProvider.findByKey(key);

                if (node && node.collapsibleState !== vscode.TreeItemCollapsibleState.None) {
                    try {
                        await treeView.reveal(node, { expand: true });
                    } catch {
                        // El nodo puede estar obsoleto tras un refresco; saltar a la línea sigue siendo útil.
                    }
                }

                jumpToLine(line, column, endLine, endColumn);
            })
        );
    } catch (error) {
        vscode.window.showErrorMessage('Failed to initialize QCTidy: ' + String(error));
    }
}

export function deactivate() {}
