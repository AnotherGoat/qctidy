import * as vscode from 'vscode';
import {
    CircuitOperation,
    GateParameter,
    ParsedCircuit,
    ParsedGate,
    evaluateInteger,
    evaluateNumber,
    circuitIssues
} from './circuit';
import { getParser } from './parser';

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
        icon?: string
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
                arguments: [this.key, line, column]
            };
        }
    }
}

/** A call argument and the source range it occupies. */
interface ArgumentNode {
    text: string;
    line: number;
    column: number;
    endLine: number;
    endColumn: number;
}

export class QCTidyTreeDataProvider implements vscode.TreeDataProvider<AstNodeItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<AstNodeItem | undefined | void> = new vscode.EventEmitter<AstNodeItem | undefined | void>();
    readonly onDidChangeTreeData: vscode.Event<AstNodeItem | undefined | void> = this._onDidChangeTreeData.event;
    private rootItems: AstNodeItem[] = [];
    private qiskitVersion: string | null = null;
    private qiskitChecked = false;
    private foundCircuits = 0;
    private circuitNameCounts = new Map<string, number>();

    refresh(): void {
        this.rootItems = [];
        this._onDidChangeTreeData.fire();
    }

    /** Store the detected Qiskit version and refresh the sidebar. */
    setQiskitVersion(version: string | null): void {
        this.qiskitVersion = version;
        this.qiskitChecked = true;
        this.refresh();
    }

    /** Reset the detected Qiskit version and re-parse the sidebar. */
    reload(): void {
        this.qiskitVersion = null;
        this.qiskitChecked = false;
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

    getChildren(element?: AstNodeItem): Thenable<AstNodeItem[]> {
        if (element) {
            return Promise.resolve(element.children);
        }

        const items: AstNodeItem[] = [this.createQiskitItem()];
        const editor = vscode.window.activeTextEditor;

        if (!editor || editor.document.languageId !== 'python') {
            items.push(this.createMessageItem('Open a Python (.py) file to analyze circuits'));
            this.rootItems = items;
            return Promise.resolve(items);
        }

        const text = editor.document.getText();
        this.foundCircuits = 0;
        this.circuitNameCounts.clear();

        try {
            const parser = getParser();
            const tree = parser.parse(text);
            const astItems = this.buildHierarchy(tree.rootNode);
            this.disambiguateCircuitNames(astItems);

            if (this.foundCircuits === 0) {
                items.push(this.createMessageItem('No circuits found in this file'));
            }

            items.push(...astItems);
        } catch (error) {
            console.error('Failed to parse the AST:', error);
            items.push(this.createMessageItem('Failed to parse the Python file'));
        }

        this.rootItems = items;
        return Promise.resolve(items);
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

    public buildHierarchy(rootNode: any): AstNodeItem[] {
        const rootItems: AstNodeItem[] = [];

        // Traverse root level to find functions, classes, and module-level circuits
        const processScope = (scopeNode: any, scopeType: 'function' | 'class'): AstNodeItem => {
            const nameNode = scopeNode.childForFieldName('name');
            const scopeName = nameNode ? nameNode.text : (scopeType === 'class' ? 'Anonymous class' : 'Anonymous function');

            const circuits = this.extractCircuitsInScope(scopeNode);

            // Check if there are nested functions or methods inside a class
            const nestedItems: AstNodeItem[] = [];
            if (scopeType === 'class') {
                const bodyNode = scopeNode.childForFieldName('body');
                if (bodyNode) {
                    for (let i = 0; i < bodyNode.childCount; i++) {
                        const child = bodyNode.child(i);
                        if (child.type === 'function_definition') {
                            nestedItems.push(processScope(child, 'function'));
                        }
                    }
                }
            }

            const circuitItems = circuits.map(circuit => this.createCircuitItem(circuit));
            const allChildren = [...nestedItems, ...circuitItems];

            const collapsibleState = allChildren.length > 0
                ? vscode.TreeItemCollapsibleState.Expanded
                : vscode.TreeItemCollapsibleState.None;

            const scopeItem = new AstNodeItem(
                scopeName,
                scopeType,
                scopeNode.startPosition.row,
                scopeNode.startPosition.column,
                collapsibleState,
                scopeType === 'class' ? 'Class' : 'Method'
            );

            scopeItem.children = allChildren;
            return scopeItem;
        };

        // 1. Process top-level functions and classes
        for (let i = 0; i < rootNode.childCount; i++) {
            const node = rootNode.child(i);
            if (node.type === 'function_definition') {
                rootItems.push(processScope(node, 'function'));
            } else if (node.type === 'class_definition') {
                rootItems.push(processScope(node, 'class'));
            }
        }

        // 2. Process top-level / module-level circuits (if any)
        const moduleCircuits = this.extractModuleLevelCircuits(rootNode);
        for (const circuit of moduleCircuits) {
            rootItems.push(this.createCircuitItem(circuit));
        }

        return rootItems;
    }

    private extractCircuitsInScope(scopeNode: any): ParsedCircuit[] {
        const circuits: ParsedCircuit[] = [];
        const circuitVariableMap = new Map<string, ParsedCircuit[]>();

        // Find all QuantumCircuit assignments inside this scope (ignoring nested functions/classes)
        const findAssignments = (node: any) => {
            if (node !== scopeNode && (node.type === 'function_definition' || node.type === 'class_definition')) {
                return;
            }

            if (node.type === 'assignment') {
                const left = node.childForFieldName('left');
                const right = node.childForFieldName('right');
                if (left && right && right.type === 'call') {
                    const func = right.childForFieldName('function');
                    if (func && (func.text === 'QuantumCircuit' || func.text.endsWith('.QuantumCircuit'))) {
                        const variableName = left.text;
                        const argumentsNode = right.childForFieldName('arguments');

                        let qubits = '0';
                        let clbits = '0';

                        if (argumentsNode) {
                            const positionalArguments: string[] = [];
                            for (let i = 0; i < argumentsNode.namedChildCount; i++) {
                                const child = argumentsNode.namedChild(i);
                                if (child.type === 'keyword_argument') {
                                    const keyName = child.childForFieldName('name')?.text;
                                    const valueNode = child.childForFieldName('value');
                                    if (keyName === 'qubits' || keyName === 'num_qubits') {
                                        qubits = valueNode ? valueNode.text : '0';
                                    } else if (keyName === 'clbits' || keyName === 'num_clbits') {
                                        clbits = valueNode ? valueNode.text : '0';
                                    }
                                } else {
                                    positionalArguments.push(child.text);
                                }
                            }

                            if (positionalArguments.length >= 1) {
                                qubits = positionalArguments[0];
                            }
                            if (positionalArguments.length >= 2) {
                                clbits = positionalArguments[1];
                            }
                        }

                        const parsedCircuit: ParsedCircuit = {
                            variableName,
                            qubits,
                            clbits,
                            line: node.startPosition.row,
                            column: node.startPosition.column,
                            gates: []
                        };

                        circuits.push(parsedCircuit);
                        const list = circuitVariableMap.get(variableName) || [];
                        list.push(parsedCircuit);
                        circuitVariableMap.set(variableName, list);
                    }
                }
            }

            for (let i = 0; i < node.childCount; i++) {
                findAssignments(node.child(i));
            }
        };

        findAssignments(scopeNode);

        // Find all gate calls on detected circuit variables
        const findGateCalls = (node: any) => {
            if (node !== scopeNode && (node.type === 'function_definition' || node.type === 'class_definition')) {
                return;
            }

            if (node.type === 'call') {
                const func = node.childForFieldName('function');
                if (func && func.type === 'attribute') {
                    const objectNode = func.childForFieldName('object');
                    const attributeNode = func.childForFieldName('attribute');

                    if (objectNode && attributeNode && circuitVariableMap.has(objectNode.text)) {
                        const candidateCircuits = circuitVariableMap.get(objectNode.text)!;
                        // Select the circuit definition that precedes this call
                        let targetCircuit = candidateCircuits[0];
                        for (const candidate of candidateCircuits) {
                            if (candidate.line <= node.startPosition.row) {
                                targetCircuit = candidate;
                            }
                        }

                        const gateMethod = attributeNode.text;
                        const argumentsNode = node.childForFieldName('arguments');
                        const callArguments: ArgumentNode[] = [];

                        if (argumentsNode) {
                            for (let i = 0; i < argumentsNode.namedChildCount; i++) {
                                const argumentNode = argumentsNode.namedChild(i);
                                callArguments.push({
                                    text: argumentNode.text,
                                    line: argumentNode.startPosition.row,
                                    column: argumentNode.startPosition.column,
                                    endLine: argumentNode.endPosition.row,
                                    endColumn: argumentNode.endPosition.column
                                });
                            }
                        }

                        const parsedGate = this.parseGateCall(
                            gateMethod,
                            callArguments,
                            node.startPosition.row,
                            node.startPosition.column,
                            node.endPosition.row,
                            node.endPosition.column
                        );

                        if (parsedGate) {
                            targetCircuit.gates.push(parsedGate);
                        }
                    }
                }
            }

            for (let i = 0; i < node.childCount; i++) {
                findGateCalls(node.child(i));
            }
        };

        findGateCalls(scopeNode);

        return circuits;
    }

    private extractModuleLevelCircuits(rootNode: any): ParsedCircuit[] {
        // Collect circuits defined outside any class or function
        const circuits: ParsedCircuit[] = [];
        const circuitVariableMap = new Map<string, ParsedCircuit[]>();

        const findTopLevelAssignments = (node: any) => {
            if (node.type === 'function_definition' || node.type === 'class_definition') {
                return;
            }

            if (node.type === 'assignment') {
                const left = node.childForFieldName('left');
                const right = node.childForFieldName('right');
                if (left && right && right.type === 'call') {
                    const func = right.childForFieldName('function');
                    if (func && (func.text === 'QuantumCircuit' || func.text.endsWith('.QuantumCircuit'))) {
                        const variableName = left.text;
                        const argumentsNode = right.childForFieldName('arguments');

                        let qubits = '0';
                        let clbits = '0';

                        if (argumentsNode) {
                            const positionalArguments: string[] = [];
                            for (let i = 0; i < argumentsNode.namedChildCount; i++) {
                                const child = argumentsNode.namedChild(i);
                                if (child.type === 'keyword_argument') {
                                    const keyName = child.childForFieldName('name')?.text;
                                    const valueNode = child.childForFieldName('value');
                                    if (keyName === 'qubits' || keyName === 'num_qubits') {
                                        qubits = valueNode ? valueNode.text : '0';
                                    } else if (keyName === 'clbits' || keyName === 'num_clbits') {
                                        clbits = valueNode ? valueNode.text : '0';
                                    }
                                } else {
                                    positionalArguments.push(child.text);
                                }
                            }

                            if (positionalArguments.length >= 1) qubits = positionalArguments[0];
                            if (positionalArguments.length >= 2) clbits = positionalArguments[1];
                        }

                        const parsedCircuit: ParsedCircuit = {
                            variableName,
                            qubits,
                            clbits,
                            line: node.startPosition.row,
                            column: node.startPosition.column,
                            gates: []
                        };

                        circuits.push(parsedCircuit);
                        const list = circuitVariableMap.get(variableName) || [];
                        list.push(parsedCircuit);
                        circuitVariableMap.set(variableName, list);
                    }
                }
            }

            for (let i = 0; i < node.childCount; i++) {
                findTopLevelAssignments(node.child(i));
            }
        };

        findTopLevelAssignments(rootNode);

        const findTopLevelGates = (node: any) => {
            if (node.type === 'function_definition' || node.type === 'class_definition') {
                return;
            }

            if (node.type === 'call') {
                const func = node.childForFieldName('function');
                if (func && func.type === 'attribute') {
                    const objectNode = func.childForFieldName('object');
                    const attributeNode = func.childForFieldName('attribute');

                    if (objectNode && attributeNode && circuitVariableMap.has(objectNode.text)) {
                        const candidateCircuits = circuitVariableMap.get(objectNode.text)!;
                        let targetCircuit = candidateCircuits[0];
                        for (const candidate of candidateCircuits) {
                            if (candidate.line <= node.startPosition.row) {
                                targetCircuit = candidate;
                            }
                        }

                        const gateMethod = attributeNode.text;
                        const argumentsNode = node.childForFieldName('arguments');
                        const callArguments: ArgumentNode[] = [];

                        if (argumentsNode) {
                            for (let i = 0; i < argumentsNode.namedChildCount; i++) {
                                const argumentNode = argumentsNode.namedChild(i);
                                callArguments.push({
                                    text: argumentNode.text,
                                    line: argumentNode.startPosition.row,
                                    column: argumentNode.startPosition.column,
                                    endLine: argumentNode.endPosition.row,
                                    endColumn: argumentNode.endPosition.column
                                });
                            }
                        }

                        const parsedGate = this.parseGateCall(
                            gateMethod,
                            callArguments,
                            node.startPosition.row,
                            node.startPosition.column,
                            node.endPosition.row,
                            node.endPosition.column
                        );

                        if (parsedGate) {
                            targetCircuit.gates.push(parsedGate);
                        }
                    }
                }
            }

            for (let i = 0; i < node.childCount; i++) {
                findTopLevelGates(node.child(i));
            }
        };

        findTopLevelGates(rootNode);

        return circuits;
    }

    private parseGateCall(
        gateMethod: string,
        callArguments: ArgumentNode[],
        line: number,
        column: number,
        endLine: number,
        endColumn: number
    ): ParsedGate | null {
        let gateName = gateMethod;
        let displayName = gateMethod;
        let description = '';
        let operation: CircuitOperation | null = null;
        let reason: string | undefined;
        const parameters: GateParameter[] = [];
        const rawArguments = callArguments.map(argument => argument.text);

        const integer = (index: number): number | null => {
            const text = rawArguments[index];
            return text === undefined ? null : evaluateInteger(text);
        };
        const number = (index: number): number | null => {
            const text = rawArguments[index];
            return text === undefined ? null : evaluateNumber(text);
        };
        const addParameter = (name: string, index: number, value?: string): void => {
            const argument = callArguments[index];
            parameters.push({
                name,
                value: value ?? rawArguments[index] ?? '',
                line: argument?.line ?? line,
                column: argument?.column ?? column
            });
        };

        // Special case for sqrt(Y): circuit.append(YGate().power(1 / 2), [0])
        if (gateMethod === 'append') {
            const firstArgument = rawArguments[0] || '';
            const qubit = rawArguments[1] === undefined ? null : extractFirstInteger(rawArguments[1]);

            if (firstArgument.includes('YGate') && firstArgument.includes('power') && qubit !== null) {
                gateName = 'sqrt(Y)';
                displayName = 'sqrt(Y)';
                operation = { gate: 'sy', qubit };

                let powerValue = '1/2';
                const powerMatch = firstArgument.match(/power\(([^)]+)\)/);
                if (powerMatch) {
                    powerValue = powerMatch[1].trim();
                }

                addParameter('power', 0, powerValue);
                addParameter('qubits', 1);
                description = `power: ${powerValue}, q: ${rawArguments[1]}`;
            } else {
                displayName = 'append';
                if (rawArguments.length > 0) addParameter('gate', 0);
                if (rawArguments.length > 1) addParameter('qubits', 1);
                if (rawArguments.length > 2) addParameter('clbits', 2);
                description = rawArguments.join(', ');
                reason = 'unsupported append call';
            }
        } else {
            const method = gateMethod.toLowerCase();

            switch (method) {
                case 'id':
                case 'h':
                case 'x':
                case 'y':
                case 'z':
                case 's':
                case 'sdg':
                case 'sx':
                case 't':
                case 'tdg': {
                    if (rawArguments[0] !== undefined) {
                        addParameter('qubit', 0);
                        description = `q: ${rawArguments[0]}`;
                    }
                    const qubit = integer(0);
                    if (qubit !== null) {
                        operation = { gate: method, qubit };
                    }
                    break;
                }

                case 'p':
                case 'rx':
                case 'ry': {
                    if (rawArguments[0] !== undefined) addParameter('theta', 0);
                    if (rawArguments[1] !== undefined) addParameter('qubit', 1);
                    description = `θ: ${rawArguments[0]}, q: ${rawArguments[1]}`;
                    const theta = number(0);
                    const qubit = integer(1);
                    if (theta !== null && qubit !== null) {
                        operation = { gate: method, theta, qubit };
                    }
                    break;
                }

                case 'rz': {
                    if (rawArguments[0] !== undefined) addParameter('theta', 0);
                    if (rawArguments[1] !== undefined) addParameter('qubit', 1);
                    description = `θ: ${rawArguments[0]}, q: ${rawArguments[1]}`;
                    const phi = number(0);
                    const qubit = integer(1);
                    if (phi !== null && qubit !== null) {
                        operation = { gate: 'rz', phi, qubit };
                    }
                    break;
                }

                case 'u': {
                    if (rawArguments[0] !== undefined) addParameter('theta', 0);
                    if (rawArguments[1] !== undefined) addParameter('phi', 1);
                    if (rawArguments[2] !== undefined) addParameter('lam', 2);
                    if (rawArguments[3] !== undefined) addParameter('qubit', 3);
                    description = `θ: ${rawArguments[0]}, φ: ${rawArguments[1]}, λ: ${rawArguments[2]}, q: ${rawArguments[3]}`;
                    const theta = number(0);
                    const phi = number(1);
                    const lambda = number(2);
                    const qubit = integer(3);
                    if (theta !== null && phi !== null && lambda !== null && qubit !== null) {
                        operation = { gate: 'u', theta, phi, lambda, qubit };
                    }
                    break;
                }

                case 'swap':
                case 'cz': {
                    if (rawArguments[0] !== undefined) addParameter('qubit 1', 0);
                    if (rawArguments[1] !== undefined) addParameter('qubit 2', 1);
                    description = `q1: ${rawArguments[0]}, q2: ${rawArguments[1]}`;
                    const qubit1 = integer(0);
                    const qubit2 = integer(1);
                    if (qubit1 !== null && qubit2 !== null) {
                        operation = { gate: method, qubit1, qubit2 };
                    }
                    break;
                }

                case 'ch':
                case 'cx':
                case 'cy': {
                    if (rawArguments[0] !== undefined) addParameter('control', 0);
                    if (rawArguments[1] !== undefined) addParameter('target', 1);
                    description = `ctrl: ${rawArguments[0]}, tgt: ${rawArguments[1]}`;
                    const control = integer(0);
                    const target = integer(1);
                    if (control !== null && target !== null) {
                        operation = { gate: method, control, target };
                    }
                    break;
                }

                case 'cp': {
                    if (rawArguments[0] !== undefined) addParameter('theta', 0);
                    if (rawArguments[1] !== undefined) addParameter('control', 1);
                    if (rawArguments[2] !== undefined) addParameter('target', 2);
                    description = `θ: ${rawArguments[0]}, ctrl: ${rawArguments[1]}, tgt: ${rawArguments[2]}`;
                    const theta = number(0);
                    const qubit1 = integer(1);
                    const qubit2 = integer(2);
                    if (theta !== null && qubit1 !== null && qubit2 !== null) {
                        operation = { gate: 'cp', theta, qubit1, qubit2 };
                    }
                    break;
                }

                case 'cswap': {
                    if (rawArguments[0] !== undefined) addParameter('control', 0);
                    if (rawArguments[1] !== undefined) addParameter('target 1', 1);
                    if (rawArguments[2] !== undefined) addParameter('target 2', 2);
                    description = `ctrl: ${rawArguments[0]}, tgt: ${rawArguments[1]}, ${rawArguments[2]}`;
                    const control = integer(0);
                    const target1 = integer(1);
                    const target2 = integer(2);
                    if (control !== null && target1 !== null && target2 !== null) {
                        operation = { gate: 'cswap', control, target1, target2 };
                    }
                    break;
                }

                case 'ccx': {
                    if (rawArguments[0] !== undefined) addParameter('control 1', 0);
                    if (rawArguments[1] !== undefined) addParameter('control 2', 1);
                    if (rawArguments[2] !== undefined) addParameter('target', 2);
                    description = `ctrl: ${rawArguments[0]}, ${rawArguments[1]}, tgt: ${rawArguments[2]}`;
                    const control1 = integer(0);
                    const control2 = integer(1);
                    const target = integer(2);
                    if (control1 !== null && control2 !== null && target !== null) {
                        operation = { gate: 'ccx', control1, control2, target };
                    }
                    break;
                }

                case 'ccz': {
                    if (rawArguments[0] !== undefined) addParameter('control 1', 0);
                    if (rawArguments[1] !== undefined) addParameter('control 2', 1);
                    if (rawArguments[2] !== undefined) addParameter('target', 2);
                    description = `ctrl: ${rawArguments[0]}, ${rawArguments[1]}, tgt: ${rawArguments[2]}`;
                    const qubit1 = integer(0);
                    const qubit2 = integer(1);
                    const qubit3 = integer(2);
                    if (qubit1 !== null && qubit2 !== null && qubit3 !== null) {
                        operation = { gate: 'ccz', qubit1, qubit2, qubit3 };
                    }
                    break;
                }

                case 'measure': {
                    if (rawArguments[0] !== undefined) addParameter('qubit', 0);
                    if (rawArguments[1] !== undefined) addParameter('clbit', 1);
                    description = `q: ${rawArguments[0]} → c: ${rawArguments[1]}`;
                    const qubit = integer(0);
                    const bit = integer(1);
                    if (qubit !== null && bit !== null) {
                        operation = { gate: 'measure', qubit, bit };
                    }
                    break;
                }

                default:
                    // Ignore non-gate utility methods
                    if (['draw', 'copy', 'qasm', 'cls', 'to_gate'].includes(gateMethod)) {
                        return null;
                    }
                    rawArguments.forEach((argumentValue, argumentIndex) => {
                        addParameter(`param_${argumentIndex + 1}`, argumentIndex, argumentValue);
                    });
                    description = rawArguments.join(', ');
                    reason = `unsupported gate: ${gateMethod}`;
                    break;
            }
        }

        if (operation === null && reason === undefined) {
            reason = `unsupported arguments: ${rawArguments.join(', ')}`;
        }

        return {
            gateName,
            displayName,
            description,
            line,
            column,
            endLine,
            endColumn,
            parameters,
            operation,
            reason
        };
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

    private createCircuitItem(circuit: ParsedCircuit): AstNodeItem {
        this.foundCircuits += 1;
        this.circuitNameCounts.set(circuit.variableName, (this.circuitNameCounts.get(circuit.variableName) ?? 0) + 1);

        // Requirement 3: Mostrar nombre de la variable que contiene el circuito
        // Requirement 4: Detectar número de cúbits y classical bits y mostrarlos
        const issues = circuitIssues(circuit);
        const hasIssues = issues.length > 0;

        const circuitItem = new AstNodeItem(
            circuit.variableName,
            'circuit',
            circuit.line,
            circuit.column,
            vscode.TreeItemCollapsibleState.Collapsed,
            hasIssues
                ? `(${circuit.qubits} qubits, ${circuit.clbits} classical bits) — cannot be checked`
                : `(${circuit.qubits} qubits, ${circuit.clbits} classical bits)`,
            hasIssues ? 'error' : undefined
        );

        // Metadata items under circuit (Requirement 4 & 6)
        const qubitsMetadata = new AstNodeItem(
            `Qubits: ${circuit.qubits}`,
            'metadata',
            circuit.line,
            circuit.column,
            vscode.TreeItemCollapsibleState.None
        );
        const clbitsMetadata = new AstNodeItem(
            `Classical bits: ${circuit.clbits}`,
            'metadata',
            circuit.line,
            circuit.column,
            vscode.TreeItemCollapsibleState.None
        );

        circuitItem.children.push(qubitsMetadata);
        circuitItem.children.push(clbitsMetadata);

        // A circuit with unsupported gates or parameters is not listed as gates,
        // because a partial circuit could make the CLI detect patterns that don't exist.
        if (hasIssues) {
            for (const issue of issues) {
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

        // Gates under circuit (Requirement 5 & 6)
        for (const gate of circuit.gates) {
            // Requirement 9: jumpToLine when clicking gate
            const gateItem = new AstNodeItem(
                gate.displayName,
                'gate',
                gate.line,
                gate.column,
                gate.parameters.length > 0
                    ? vscode.TreeItemCollapsibleState.Collapsed
                    : vscode.TreeItemCollapsibleState.None,
                gate.description
            );

            // Requirement 7: Parámetros dentro de cada compuerta
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
}

/** Extract the first integer of a value such as `[0]` or `[1, 2]`. */
function extractFirstInteger(text: string): number | null {
    const match = text.match(/\d+/);
    return match ? Number.parseInt(match[0], 10) : null;
}
