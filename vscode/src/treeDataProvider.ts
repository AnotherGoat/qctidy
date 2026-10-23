import * as vscode from 'vscode';
import { AstAnalysis, AstCircuit, AstScope, runAst } from './ast';
import { QctidyStatus } from './cli';

export type AstNodeType =
    | 'class'
    | 'function'
    | 'circuit'
    | 'metadata'
    | 'gate'
    | 'parameter'
    | 'error';

export class AstNodeItem extends vscode.TreeItem {
    public children: AstNodeItem[] = [];
    public readonly key: string;

    constructor(
        public label: string,
        public readonly nodeType: AstNodeType,
        public readonly line: number,
        public readonly column: number,
        collapsibleState: vscode.TreeItemCollapsibleState = vscode.TreeItemCollapsibleState.None,
        description?: string,
        icon?: string,
        public readonly endLine?: number,
        public readonly endColumn?: number
    ) {
        super(label, collapsibleState);
        this.key = `${nodeType}:${line}:${column}:${label}`;
        this.description = description;
        // Los TreeItem no soportan ajuste de línea: el tooltip muestra el texto completo.
        this.tooltip = description ? `${label}\n\n${description}` : label;

        if (icon) {
            this.iconPath = new vscode.ThemeIcon(icon);
        } else {
            switch (nodeType) {
                case 'class':
                    this.iconPath = new vscode.ThemeIcon('symbol-class');
                    break;
                case 'function':
                    this.iconPath = new vscode.ThemeIcon('symbol-method');
                    break;
                case 'circuit':
                    this.iconPath = new vscode.ThemeIcon('circuit-board');
                    break;
                case 'metadata':
                    this.iconPath = new vscode.ThemeIcon('info');
                    break;
                case 'gate':
                    this.iconPath = new vscode.ThemeIcon('symbol-operator');
                    break;
                case 'parameter':
                    this.iconPath = new vscode.ThemeIcon('symbol-variable');
                    break;
                case 'error':
                    this.iconPath = new vscode.ThemeIcon('error');
                    break;
            }
        }

        if (line >= 0 && column >= 0) {
            this.command = {
                command: 'qctidy.openNode',
                title: 'Open node',
                // Note: only serializable arguments, so the command can look the node up again.
                arguments: endLine !== undefined && endColumn !== undefined
                    ? [this.key, line, column, endLine, endColumn]
                    : [this.key, line, column]
            };
        }
    }
}

export class QCTidyTreeDataProvider implements vscode.TreeDataProvider<AstNodeItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<AstNodeItem | undefined | void> = new vscode.EventEmitter<AstNodeItem | undefined | void>();
    readonly onDidChangeTreeData: vscode.Event<AstNodeItem | undefined | void> = this._onDidChangeTreeData.event;
    private rootItems: AstNodeItem[] = [];
    private qiskitVersion: string | null = null;
    private qiskitChecked = false;
    private qctidyStatus: QctidyStatus | null = null;
    private qctidyChecked = false;
    private foundCircuits = 0;
    private circuitNameCounts = new Map<string, number>();

    refresh(): void {
        this.rootItems = [];
        this._onDidChangeTreeData.fire();
    }

    /** Store the detected QCTidy status and refresh the sidebar. */
    setQctidyStatus(status: QctidyStatus): void {
        this.qctidyStatus = status;
        this.qctidyChecked = true;
        this.refresh();
    }

    /** Store the detected Qiskit version and refresh the sidebar. */
    setQiskitVersion(version: string | null): void {
        this.qiskitVersion = version;
        this.qiskitChecked = true;
        this.refresh();
    }

    /** Reset the detected versions and re-parse the sidebar. */
    reload(): void {
        this.qiskitVersion = null;
        this.qiskitChecked = false;
        this.qctidyStatus = null;
        this.qctidyChecked = false;
        this.refresh();
    }

    /** Find a rendered node by its key, so commands can reveal it. */
    findByKey(key: string): AstNodeItem | undefined {
        const search = (items: AstNodeItem[]): AstNodeItem | undefined => {
            for (const item of items) {
                if (item.key === key) {
                    return item;
                }

                const found = search(item.children);
                if (found) {
                    return found;
                }
            }

            return undefined;
        };

        return search(this.rootItems);
    }

    getTreeItem(element: AstNodeItem): vscode.TreeItem {
        return element;
    }

    async getChildren(element?: AstNodeItem): Promise<AstNodeItem[]> {
        if (element) {
            return element.children;
        }

        const items: AstNodeItem[] = [
            this.createQctidyItem(),
            this.createQiskitItem()
        ];

        // If QCTidy CLI is not available, disable all circuit analysis features
        if (this.qctidyChecked && this.qctidyStatus && !this.qctidyStatus.available) {
            items.push(this.createMessageItem(
                this.qctidyStatus.error
                    ? `QCTidy error: ${this.qctidyStatus.error}`
                    : 'QCTidy executable not found. Check "qctidy.path" in settings.'
            ));
            this.rootItems = items;
            return items;
        }

        const editor = vscode.window.activeTextEditor;

        if (!editor || editor.document.languageId !== 'python') {
            items.push(this.createMessageItem('Open a Python (.py) file to analyze circuits'));
            this.rootItems = items;
            return items;
        }

        const source = editor.document.getText();
        this.foundCircuits = 0;
        this.circuitNameCounts.clear();

        try {
            const analysis = await runAst(source);
            const astItems = this.buildHierarchy(analysis);
            this.disambiguateCircuitNames(astItems);

            if (this.foundCircuits === 0) {
                items.push(this.createMessageItem('No circuits found in this file'));
            }

            items.push(...astItems);
        } catch (error) {
            console.error('Failed to analyze the Python file:', error);
            items.push(this.createMessageItem('Failed to analyze the Python file'));
        }

        this.rootItems = items;
        return items;
    }

    private createQctidyItem(): AstNodeItem {
        if (!this.qctidyChecked) {
            return new AstNodeItem('QCTidy: detecting...', 'metadata', -1, -1, vscode.TreeItemCollapsibleState.None, undefined, 'sync~spin');
        }

        if (!this.qctidyStatus || !this.qctidyStatus.available) {
            return new AstNodeItem(
                'QCTidy: not found',
                'metadata',
                -1,
                -1,
                vscode.TreeItemCollapsibleState.None,
                this.qctidyStatus?.error ?? 'Executable not found in PATH or configured path',
                'error'
            );
        }

        return new AstNodeItem(
            `QCTidy: ${this.qctidyStatus.version ?? 'available'}`,
            'metadata',
            -1,
            -1,
            vscode.TreeItemCollapsibleState.None,
            this.qctidyStatus.executable,
            'tools'
        );
    }

    private createQiskitItem(): AstNodeItem {
        if (!this.qiskitChecked) {
            return new AstNodeItem('Qiskit: detecting...', 'metadata', -1, -1, vscode.TreeItemCollapsibleState.None, undefined, 'sync~spin');
        }

        if (this.qiskitVersion === null) {
            return new AstNodeItem('Qiskit: not found', 'metadata', -1, -1, vscode.TreeItemCollapsibleState.None, undefined, 'warning');
        }

        return new AstNodeItem(`Qiskit: ${this.qiskitVersion}`, 'metadata', -1, -1, vscode.TreeItemCollapsibleState.None, undefined, 'beaker');
    }

    private createMessageItem(message: string): AstNodeItem {
        return new AstNodeItem(message, 'metadata', -1, -1, vscode.TreeItemCollapsibleState.None, undefined, 'info');
    }

    private buildHierarchy(analysis: AstAnalysis): AstNodeItem[] {
        const rootItems: AstNodeItem[] = [];

        for (const scope of analysis.scopes) {
            rootItems.push(this.createScopeItem(scope));
        }

        for (const circuit of analysis.module_circuits) {
            rootItems.push(this.createCircuitItem(circuit));
        }

        return rootItems;
    }

    private createScopeItem(scope: AstScope): AstNodeItem {
        const children = scope.children.map((child) => {
            if (child.type === 'circuit') {
                return this.createCircuitItem(child);
            }

            return this.createScopeItem(child);
        });

        const collapsibleState = children.length > 0
            ? vscode.TreeItemCollapsibleState.Expanded
            : vscode.TreeItemCollapsibleState.None;

        const scopeItem = new AstNodeItem(
            scope.name,
            scope.type,
            scope.line,
            scope.column,
            collapsibleState,
            scope.type === 'class' ? 'Class' : 'Method'
        );

        scopeItem.children = children;
        return scopeItem;
    }

    private createCircuitItem(circuit: AstCircuit): AstNodeItem {
        this.foundCircuits += 1;
        this.circuitNameCounts.set(circuit.name, (this.circuitNameCounts.get(circuit.name) ?? 0) + 1);

        const hasIssues = circuit.issues.length > 0;

        const circuitItem = new AstNodeItem(
            circuit.name,
            'circuit',
            circuit.line,
            circuit.column,
            vscode.TreeItemCollapsibleState.Collapsed,
            hasIssues
                ? `(${circuit.qubit_count} qubits, ${circuit.bit_count} classical bits) — cannot be checked`
                : `(${circuit.qubit_count} qubits, ${circuit.bit_count} classical bits)`,
            hasIssues ? 'error' : undefined
        );

        const qubitCountMetadata = new AstNodeItem(
            `Qubits: ${circuit.qubit_count}`,
            'metadata',
            circuit.line,
            circuit.column,
            vscode.TreeItemCollapsibleState.None
        );
        const bitCountMetadata = new AstNodeItem(
            `Classical bits: ${circuit.bit_count}`,
            'metadata',
            circuit.line,
            circuit.column,
            vscode.TreeItemCollapsibleState.None
        );

        circuitItem.children.push(qubitCountMetadata);
        circuitItem.children.push(bitCountMetadata);

        // A circuit with issues is not listed as gates, because a partial
        // circuit could make the CLI detect patterns that don't exist.
        if (hasIssues) {
            for (const issue of circuit.issues) {
                circuitItem.children.push(
                    new AstNodeItem(
                        issue.message,
                        'error',
                        issue.line,
                        issue.column,
                        vscode.TreeItemCollapsibleState.None,
                        undefined,
                        'error'
                    )
                );
            }

            return circuitItem;
        }

        for (const gate of circuit.gates) {
            const gateItem = new AstNodeItem(
                gate.display_name,
                'gate',
                gate.line,
                gate.column,
                gate.parameters.length > 0
                    ? vscode.TreeItemCollapsibleState.Collapsed
                    : vscode.TreeItemCollapsibleState.None,
                gate.description,
                undefined,
                gate.end_line,
                gate.end_column
            );

            for (const parameter of gate.parameters) {
                const parameterItem = new AstNodeItem(
                    `${parameter.name}: ${parameter.value}`,
                    'parameter',
                    parameter.line,
                    parameter.column,
                    vscode.TreeItemCollapsibleState.None
                );
                gateItem.children.push(parameterItem);
            }

            circuitItem.children.push(gateItem);
        }

        return circuitItem;
    }

    /** Append the definition line to circuits whose variable name is repeated. */
    private disambiguateCircuitNames(items: AstNodeItem[]): void {
        for (const item of items) {
            if (item.nodeType === 'circuit' && (this.circuitNameCounts.get(item.label) ?? 0) > 1) {
                item.label = `${item.label}:${item.line + 1}`;
            }

            this.disambiguateCircuitNames(item.children);
        }
    }
}
