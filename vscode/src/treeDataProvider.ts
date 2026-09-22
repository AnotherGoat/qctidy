import * as vscode from 'vscode';
import { getParser } from './parser';

export class AstNodeItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly type: string,
        public readonly line: number,
        public readonly column: number,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState
    ) {
        super(label, collapsibleState);
        
        // Asignamos iconos distintos según si es Clase, Método o Circuito
        if (type === 'class') {
            this.iconPath = new vscode.ThemeIcon('symbol-class');
            this.description = 'Clase';
        } else if (type === 'function') {
            this.iconPath = new vscode.ThemeIcon('symbol-method');
            this.description = 'Método';
        } else if (type === 'circuit') {
            this.iconPath = new vscode.ThemeIcon('circuit-board');
            this.description = 'Circuito Qiskit';
        }

        // Más adelante conectaremos esto con el click
        this.command = {
            command: 'qctidy.jumpToLine',
            title: 'Saltar a línea',
            arguments: [this.line, this.column]
        };
    }
}

export class QCTidyTreeDataProvider implements vscode.TreeDataProvider<AstNodeItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<AstNodeItem | undefined | void> = new vscode.EventEmitter<AstNodeItem | undefined | void>();
    readonly onDidChangeTreeData: vscode.Event<AstNodeItem | undefined | void> = this._onDidChangeTreeData.event;

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: AstNodeItem): vscode.TreeItem {
        return element;
    }

    getChildren(element?: AstNodeItem): Thenable<AstNodeItem[]> {
        if (element) {
            // Por ahora mantenemos una lista plana (sin sub-hijos) para hacerlo sencillo
            return Promise.resolve([]);
        }

        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'python') {
            return Promise.resolve([]);
        }

        const text = editor.document.getText();
        
        try {
            const parser = getParser();
            const tree = parser.parse(text);
            const items: AstNodeItem[] = [];

            // Función recursiva para buscar clases y métodos
            const traverse = (node: any) => {
                if (node.type === 'class_definition') {
                    const nameNode = node.childForFieldName('name');
                    if (nameNode) {
                        items.push(new AstNodeItem(
                            nameNode.text,
                            'class',
                            nameNode.startPosition.row,
                            nameNode.startPosition.column,
                            vscode.TreeItemCollapsibleState.None
                        ));
                    }
                } else if (node.type === 'function_definition') {
                    const nameNode = node.childForFieldName('name');
                    if (nameNode) {
                        items.push(new AstNodeItem(
                            nameNode.text,
                            'function',
                            nameNode.startPosition.row,
                            nameNode.startPosition.column,
                            vscode.TreeItemCollapsibleState.None
                        ));
                    }
                } else if (node.type === 'call') {
                    const funcNode = node.childForFieldName('function');
                    if (funcNode && funcNode.text === 'QuantumCircuit') {
                        // Si encontramos una creación de QuantumCircuit
                        items.push(new AstNodeItem(
                            'QuantumCircuit',
                            'circuit',
                            funcNode.startPosition.row,
                            funcNode.startPosition.column,
                            vscode.TreeItemCollapsibleState.None
                        ));
                    }
                }

                // Seguimos buscando en los hijos
                for (let i = 0; i < node.childCount; i++) {
                    traverse(node.child(i));
                }
            };

            traverse(tree.rootNode);
            return Promise.resolve(items);
            
        } catch (error) {
            console.error('Error al analizar el AST:', error);
            return Promise.resolve([]);
        }
    }
}
