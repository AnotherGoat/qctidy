import * as vscode from 'vscode';
import { AstAnalysis, AstCircuit, AstGate, AstScope, runAst } from './ast';
import { QctidyStatus, runCheck } from './cli';
import { CheckReport, CliDiagnostic, CliError } from './diagnostics';

export type AstNodeType =
    | 'class'
    | 'function'
    | 'circuit'
    | 'metadata'
    | 'gate'
    | 'parameter'
    | 'error'
    | 'warning'
    | 'diagnostic';

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
        icon?: string | vscode.ThemeIcon,
        public readonly endLine?: number,
        public readonly endColumn?: number
    ) {
        super(label, collapsibleState);
        this.key = `${nodeType}:${line}:${column}:${label}`;
        this.description = description;
        // TreeItems do not support line wrapping: the tooltip displays full details.
        this.tooltip = description ? `${label}\n\n${description}` : label;

        if (icon) {
            this.iconPath = typeof icon === 'string' ? new vscode.ThemeIcon(icon) : icon;
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
                    this.iconPath = new vscode.ThemeIcon('error', new vscode.ThemeColor('problemsErrorIcon.foreground'));
                    break;
                case 'warning':
                    this.iconPath = new vscode.ThemeIcon('warning', new vscode.ThemeColor('problemsWarningIcon.foreground'));
                    break;
                case 'diagnostic':
                    this.iconPath = new vscode.ThemeIcon('info');
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

    private astAnalysis: AstAnalysis | null = null;
    private checkReport: CheckReport | null = null;
    private lastSource: string | null = null;

    refresh(): void {
        this.rootItems = [];
        this._onDidChangeTreeData.fire();
    }

    /** Store the external analysis and diagnostic results and refresh the sidebar. */
    setData(analysis: AstAnalysis, report: CheckReport, source?: string): void {
        this.astAnalysis = analysis;
        this.checkReport = report;
        if (source !== undefined) {
            this.lastSource = source;
        }
        this.refresh();
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
        this.astAnalysis = null;
        this.checkReport = null;
        this.lastSource = null;
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
        const inputPath = editor.document.uri.fsPath;
        this.foundCircuits = 0;
        this.circuitNameCounts.clear();

        try {
            let analysis = this.astAnalysis;
            let report = this.checkReport;

            if (!analysis || this.lastSource !== source) {
                const [astRes, checkRes] = await Promise.all([
                    runAst(source),
                    runCheck(source, inputPath),
                ]);
                analysis = astRes;
                report = checkRes;
                this.astAnalysis = astRes;
                this.checkReport = checkRes;
                this.lastSource = source;
            }

            const astItems = this.buildHierarchy(analysis, report ?? { diagnostics: [], errors: [] });
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

    private buildHierarchy(analysis: AstAnalysis, report: CheckReport): AstNodeItem[] {
        // Group diagnostics by circuit name
        const diagnosticsByCircuit = new Map<string, CliDiagnostic[]>();
        for (const diag of report.diagnostics) {
            const circuitKey = diag.circuit ?? '';
            const list = diagnosticsByCircuit.get(circuitKey) ?? [];
            list.push(diag);
            diagnosticsByCircuit.set(circuitKey, list);
        }

        // Group CLI errors by circuit name
        const errorsByCircuit = new Map<string, CliError[]>();
        for (const err of report.errors) {
            const circuitKey = err.circuit ?? '';
            const list = errorsByCircuit.get(circuitKey) ?? [];
            list.push(err);
            errorsByCircuit.set(circuitKey, list);
        }

        const rootItems: AstNodeItem[] = [];

        for (const scope of analysis.scopes) {
            rootItems.push(this.createScopeItem(scope, diagnosticsByCircuit, errorsByCircuit));
        }

        for (const circuit of analysis.module_circuits) {
            rootItems.push(this.createCircuitItem(circuit, diagnosticsByCircuit, errorsByCircuit));
        }

        return rootItems;
    }

    private createScopeItem(
        scope: AstScope,
        diagnosticsByCircuit: Map<string, CliDiagnostic[]>,
        errorsByCircuit: Map<string, CliError[]>
    ): AstNodeItem {
        const children = scope.children.map((child) => {
            if (child.type === 'circuit') {
                return this.createCircuitItem(child, diagnosticsByCircuit, errorsByCircuit);
            }

            return this.createScopeItem(child, diagnosticsByCircuit, errorsByCircuit);
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

    private createCircuitItem(
        circuit: AstCircuit,
        diagnosticsByCircuit: Map<string, CliDiagnostic[]>,
        errorsByCircuit: Map<string, CliError[]>
    ): AstNodeItem {
        this.foundCircuits += 1;
        this.circuitNameCounts.set(circuit.name, (this.circuitNameCounts.get(circuit.name) ?? 0) + 1);

        const hasIssues = circuit.issues.length > 0;
        const circuitErrors = errorsByCircuit.get(circuit.name) ?? [];

        // All diagnostics matching this circuit
        const allCircuitDiags = (diagnosticsByCircuit.get(circuit.name) ?? []).filter((diag) => {
            // If multiple circuits share the same variable name, correlate by range
            if ((this.circuitNameCounts.get(circuit.name) ?? 0) > 1 && circuit.gates.length > 0) {
                const maxEndLine = Math.max(...circuit.gates.map((g) => g.end_line));
                return diag.source_locations.some(
                    (loc) => loc.line >= circuit.line && loc.end_line <= maxEndLine
                );
            }
            return true;
        });

        const hasDiagnosticErrors = allCircuitDiags.some(
            (d) => d.severity.toLowerCase() === 'error'
        );
        const hasCliErrors = circuitErrors.length > 0;
        const isError = hasIssues || hasCliErrors || hasDiagnosticErrors;
        const hasWarnings = allCircuitDiags.some(
            (d) => d.severity.toLowerCase() !== 'error'
        );

        // Icon prioritizes worst state: Error > Warning > Pass (green check)
        let circuitIcon: vscode.ThemeIcon;
        let statusSummary = '';

        if (isError) {
            circuitIcon = new vscode.ThemeIcon('error', new vscode.ThemeColor('problemsErrorIcon.foreground'));
            statusSummary = hasIssues ? ' — cannot be checked' : ' — errors detected';
        } else if (hasWarnings) {
            circuitIcon = new vscode.ThemeIcon('warning', new vscode.ThemeColor('problemsWarningIcon.foreground'));
            statusSummary = ' — warnings detected';
        } else {
            // Green check indicating no issues detected
            circuitIcon = new vscode.ThemeIcon('pass', new vscode.ThemeColor('charts.green'));
        }

        const circuitItem = new AstNodeItem(
            circuit.name,
            'circuit',
            circuit.line,
            circuit.column,
            vscode.TreeItemCollapsibleState.Collapsed,
            `(${circuit.qubit_count} qubits, ${circuit.bit_count} classical bits)${statusSummary}`,
            circuitIcon
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

        // A circuit with parse issues is not listed as gates
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
                        new vscode.ThemeIcon('error', new vscode.ThemeColor('problemsErrorIcon.foreground'))
                    )
                );
            }
            return circuitItem;
        }

        // Add any global CLI errors for this circuit
        for (const err of circuitErrors) {
            circuitItem.children.push(
                new AstNodeItem(
                    err.message,
                    'error',
                    circuit.line,
                    circuit.column,
                    vscode.TreeItemCollapsibleState.None,
                    undefined,
                    new vscode.ThemeIcon('error', new vscode.ThemeColor('problemsErrorIcon.foreground'))
                )
            );
        }

        const assignedDiagnostics = new Set<CliDiagnostic>();

        // Build gate items and associate diagnostics
        for (let gateIndex = 0; gateIndex < circuit.gates.length; gateIndex++) {
            const gate = circuit.gates[gateIndex];

            // Match diagnostics by operation_indices or source_locations range overlap
            const gateDiags = allCircuitDiags.filter((diag) => {
                if (diag.operation_indices && diag.operation_indices.includes(gateIndex)) {
                    return true;
                }
                return diag.source_locations.some((loc) => {
                    if (loc.line === gate.line && loc.end_line === gate.end_line) {
                        return loc.column < gate.end_column && loc.end_column > gate.column;
                    }
                    return loc.line <= gate.end_line && loc.end_line >= gate.line;
                });
            });

            for (const diag of gateDiags) {
                assignedDiagnostics.add(diag);
            }

            const gateHasError = gateDiags.some((d) => d.severity.toLowerCase() === 'error');
            const gateHasWarning = gateDiags.length > 0 && !gateHasError;

            let gateIcon: vscode.ThemeIcon | undefined;
            let gateDescription = gate.description;

            if (gateHasError) {
                gateIcon = new vscode.ThemeIcon('error', new vscode.ThemeColor('problemsErrorIcon.foreground'));
                const diagSummary = gateDiags.map((d) => `[${d.code}] ${d.message}`).join('; ');
                gateDescription = gateDescription ? `${gateDescription} — ${diagSummary}` : diagSummary;
            } else if (gateHasWarning) {
                gateIcon = new vscode.ThemeIcon('warning', new vscode.ThemeColor('problemsWarningIcon.foreground'));
                const diagSummary = gateDiags.map((d) => `[${d.code}] ${d.message}`).join('; ');
                gateDescription = gateDescription ? `${gateDescription} — ${diagSummary}` : diagSummary;
            }

            const hasChildren = gate.parameters.length > 0 || gateDiags.length > 0;
            const gateItem = new AstNodeItem(
                gate.display_name,
                'gate',
                gate.line,
                gate.column,
                hasChildren
                    ? vscode.TreeItemCollapsibleState.Collapsed
                    : vscode.TreeItemCollapsibleState.None,
                gateDescription,
                gateIcon,
                gate.end_line,
                gate.end_column
            );

            // Gate parameters
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

            // Gate diagnostics as children for easy clicking and inspection
            for (const diag of gateDiags) {
                const isErr = diag.severity.toLowerCase() === 'error';
                const loc = diag.source_locations[0] ?? {
                    line: gate.line,
                    column: gate.column,
                    end_line: gate.end_line,
                    end_column: gate.end_column,
                };
                const diagItem = new AstNodeItem(
                    `[${diag.code}] ${diag.message}`,
                    isErr ? 'error' : 'warning',
                    loc.line,
                    loc.column,
                    vscode.TreeItemCollapsibleState.None,
                    diag.category ? `Category: ${diag.category}` : undefined,
                    isErr
                        ? new vscode.ThemeIcon('error', new vscode.ThemeColor('problemsErrorIcon.foreground'))
                        : new vscode.ThemeIcon('warning', new vscode.ThemeColor('problemsWarningIcon.foreground')),
                    loc.end_line,
                    loc.end_column
                );
                gateItem.children.push(diagItem);
            }

            circuitItem.children.push(gateItem);
        }

        // Diagnostics that were not assigned to any specific gate are shown under the circuit
        const unassignedDiags = allCircuitDiags.filter((diag) => !assignedDiagnostics.has(diag));
        for (const diag of unassignedDiags) {
            const isErr = diag.severity.toLowerCase() === 'error';
            const loc = diag.source_locations[0];
            const diagItem = new AstNodeItem(
                `[${diag.code}] ${diag.message}`,
                isErr ? 'error' : 'warning',
                loc ? loc.line : circuit.line,
                loc ? loc.column : circuit.column,
                vscode.TreeItemCollapsibleState.None,
                diag.category ? `Category: ${diag.category}` : undefined,
                isErr
                    ? new vscode.ThemeIcon('error', new vscode.ThemeColor('problemsErrorIcon.foreground'))
                    : new vscode.ThemeIcon('warning', new vscode.ThemeColor('problemsWarningIcon.foreground')),
                loc?.end_line,
                loc?.end_column
            );
            circuitItem.children.push(diagItem);
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
