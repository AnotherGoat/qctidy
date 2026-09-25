import * as vscode from 'vscode';
import { initParser } from './parser';
import { QCTidyTreeDataProvider } from './treeDataProvider';

export async function activate(context: vscode.ExtensionContext) {
    console.log('QCTidy VS Code extension is now active!');

    try {
        // 1. Inicializar Tree-sitter
        await initParser(context.extensionUri);

        // 2. Registrar el Sidebar (Tree View)
        const treeDataProvider = new QCTidyTreeDataProvider();
        vscode.window.registerTreeDataProvider('qctidy-ast-view', treeDataProvider);

        // 3. Refrescar el sidebar cuando el usuario cambia de pestaña o edita el texto
        vscode.window.onDidChangeActiveTextEditor(() => treeDataProvider.refresh());
        vscode.workspace.onDidChangeTextDocument(e => {
            if (vscode.window.activeTextEditor && e.document === vscode.window.activeTextEditor.document) {
                treeDataProvider.refresh();
            }
        });

        // 4. Comando de navegación
        const disposable = vscode.commands.registerCommand('qctidy.jumpToLine', (line: number, column: number) => {
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                const position = new vscode.Position(line, column);
                editor.selection = new vscode.Selection(position, position);
                editor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
            }
        });

        context.subscriptions.push(disposable);
    } catch (error) {
        vscode.window.showErrorMessage('Error inicializando QCTidy: ' + String(error));
    }
}

export function deactivate() {}
