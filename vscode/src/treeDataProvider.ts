import * as vscode from 'vscode';
import { getParser } from './parser';

export type AstNodeType =
    | 'class'
    | 'function'
    | 'circuit'
    | 'metadata'
    | 'gate'
    | 'parameter';

export class AstNodeItem extends vscode.TreeItem {
    public children: AstNodeItem[] = [];

    constructor(
        public readonly label: string,
        public readonly nodeType: AstNodeType,
        public readonly line: number,
        public readonly column: number,
        collapsibleState: vscode.TreeItemCollapsibleState = vscode.TreeItemCollapsibleState.None,
        description?: string
    ) {
        super(label, collapsibleState);
        this.description = description;

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
        }

        if (line >= 0 && column >= 0) {
            this.command = {
                command: 'qctidy.jumpToLine',
                title: 'Saltar a línea',
                arguments: [this.line, this.column]
            };
        }
    }
}

interface GateParameter {
    name: string;
    value: string;
}

interface ParsedGate {
    gateName: string;
    displayName: string;
    description: string;
    line: number;
    column: number;
    parameters: GateParameter[];
}

interface ParsedCircuit {
    variableName: string;
    qubits: string;
    clbits: string;
    line: number;
    column: number;
    gates: ParsedGate[];
}

export class QCTidyTreeDataProvider implements vscode.TreeDataProvider<AstNodeItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<AstNodeItem | undefined | void> = new vscode.EventEmitter<AstNodeItem | undefined | void>();
    readonly onDidChangeTreeData: vscode.Event<AstNodeItem | undefined | void> = this._onDidChangeTreeData.event;
    private rootItems: AstNodeItem[] = [];

    refresh(): void {
        this.rootItems = [];
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: AstNodeItem): vscode.TreeItem {
        return element;
    }

    getChildren(element?: AstNodeItem): Thenable<AstNodeItem[]> {
        if (element) {
            return Promise.resolve(element.children);
        }

        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'python') {
            return Promise.resolve([]);
        }

        const text = editor.document.getText();

        try {
            const parser = getParser();
            const tree = parser.parse(text);
            this.rootItems = this.buildHierarchy(tree.rootNode);
            return Promise.resolve(this.rootItems);
        } catch (error) {
            console.error('Error al analizar el AST:', error);
            return Promise.resolve([]);
        }
    }

    public buildHierarchy(rootNode: any): AstNodeItem[] {
        const rootItems: AstNodeItem[] = [];

        // Traverse root level to find functions, classes, and module-level circuits
        const processScope = (scopeNode: any, scopeType: 'function' | 'class'): AstNodeItem => {
            const nameNode = scopeNode.childForFieldName('name');
            const scopeName = nameNode ? nameNode.text : (scopeType === 'class' ? 'Clase Anónima' : 'Función Anónima');

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
                scopeType === 'class' ? 'Clase' : 'Método'
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
                        const rawArguments: string[] = [];

                        if (argumentsNode) {
                            for (let i = 0; i < argumentsNode.namedChildCount; i++) {
                                rawArguments.push(argumentsNode.namedChild(i).text);
                            }
                        }

                        const parsedGate = this.parseGateCall(
                            gateMethod,
                            rawArguments,
                            node.startPosition.row,
                            node.startPosition.column
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
                        const rawArguments: string[] = [];

                        if (argumentsNode) {
                            for (let i = 0; i < argumentsNode.namedChildCount; i++) {
                                rawArguments.push(argumentsNode.namedChild(i).text);
                            }
                        }

                        const parsedGate = this.parseGateCall(
                            gateMethod,
                            rawArguments,
                            node.startPosition.row,
                            node.startPosition.column
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
        rawArguments: string[],
        line: number,
        column: number
    ): ParsedGate | null {
        let gateName = gateMethod;
        let displayName = gateMethod;
        let description = '';
        const parameters: GateParameter[] = [];

        // Requirement 8: Detección especial para la compuerta sqrt(Y) (YGate().power(1 / 2))
        if (gateMethod === 'append') {
            const firstArgument = rawArguments[0] || '';
            if (firstArgument.includes('YGate') && firstArgument.includes('power')) {
                gateName = 'sqrt(Y)';
                displayName = 'sqrt(Y)';

                let powerValue = '1/2';
                const powerMatch = firstArgument.match(/power\(([^)]+)\)/);
                if (powerMatch) {
                    powerValue = powerMatch[1].trim();
                }

                parameters.push({ name: 'power', value: powerValue });

                if (rawArguments.length > 1) {
                    parameters.push({ name: 'qubits', value: rawArguments[1] });
                    description = `power: ${powerValue}, q: ${rawArguments[1]}`;
                } else {
                    description = `power: ${powerValue}`;
                }
            } else {
                displayName = 'append';
                if (rawArguments.length > 0) parameters.push({ name: 'gate', value: rawArguments[0] });
                if (rawArguments.length > 1) parameters.push({ name: 'qubits', value: rawArguments[1] });
                if (rawArguments.length > 2) parameters.push({ name: 'clbits', value: rawArguments[2] });
                description = rawArguments.join(', ');
            }
        } else {
            switch (gateMethod.toLowerCase()) {
                case 'id':
                case 'h':
                case 'x':
                case 'y':
                case 'z':
                case 's':
                case 'sdg':
                case 'sx':
                case 't':
                case 'tdg':
                    if (rawArguments[0] !== undefined) {
                        parameters.push({ name: 'qubit', value: rawArguments[0] });
                        description = `q: ${rawArguments[0]}`;
                    }
                    break;

                case 'p':
                    if (rawArguments[0] !== undefined) parameters.push({ name: 'theta', value: rawArguments[0] });
                    if (rawArguments[1] !== undefined) parameters.push({ name: 'qubit', value: rawArguments[1] });
                    description = `θ: ${rawArguments[0]}, q: ${rawArguments[1]}`;
                    break;

                case 'rx':
                case 'ry':
                case 'rz':
                    if (rawArguments[0] !== undefined) parameters.push({ name: 'theta', value: rawArguments[0] });
                    if (rawArguments[1] !== undefined) parameters.push({ name: 'qubit', value: rawArguments[1] });
                    description = `θ: ${rawArguments[0]}, q: ${rawArguments[1]}`;
                    break;

                case 'u':
                    if (rawArguments[0] !== undefined) parameters.push({ name: 'theta', value: rawArguments[0] });
                    if (rawArguments[1] !== undefined) parameters.push({ name: 'phi', value: rawArguments[1] });
                    if (rawArguments[2] !== undefined) parameters.push({ name: 'lam', value: rawArguments[2] });
                    if (rawArguments[3] !== undefined) parameters.push({ name: 'qubit', value: rawArguments[3] });
                    description = `θ: ${rawArguments[0]}, φ: ${rawArguments[1]}, λ: ${rawArguments[2]}, q: ${rawArguments[3]}`;
                    break;

                case 'swap':
                    if (rawArguments[0] !== undefined) parameters.push({ name: 'qubit 1', value: rawArguments[0] });
                    if (rawArguments[1] !== undefined) parameters.push({ name: 'qubit 2', value: rawArguments[1] });
                    description = `q1: ${rawArguments[0]}, q2: ${rawArguments[1]}`;
                    break;

                case 'ch':
                case 'cx':
                case 'cy':
                case 'cz':
                    if (rawArguments[0] !== undefined) parameters.push({ name: 'control', value: rawArguments[0] });
                    if (rawArguments[1] !== undefined) parameters.push({ name: 'target', value: rawArguments[1] });
                    description = `ctrl: ${rawArguments[0]}, tgt: ${rawArguments[1]}`;
                    break;

                case 'cp':
                    if (rawArguments[0] !== undefined) parameters.push({ name: 'theta', value: rawArguments[0] });
                    if (rawArguments[1] !== undefined) parameters.push({ name: 'control', value: rawArguments[1] });
                    if (rawArguments[2] !== undefined) parameters.push({ name: 'target', value: rawArguments[2] });
                    description = `θ: ${rawArguments[0]}, ctrl: ${rawArguments[1]}, tgt: ${rawArguments[2]}`;
                    break;

                case 'cswap':
                    if (rawArguments[0] !== undefined) parameters.push({ name: 'control', value: rawArguments[0] });
                    if (rawArguments[1] !== undefined) parameters.push({ name: 'target 1', value: rawArguments[1] });
                    if (rawArguments[2] !== undefined) parameters.push({ name: 'target 2', value: rawArguments[2] });
                    description = `ctrl: ${rawArguments[0]}, tgt: ${rawArguments[1]}, ${rawArguments[2]}`;
                    break;

                case 'ccx':
                case 'ccz':
                    if (rawArguments[0] !== undefined) parameters.push({ name: 'control 1', value: rawArguments[0] });
                    if (rawArguments[1] !== undefined) parameters.push({ name: 'control 2', value: rawArguments[1] });
                    if (rawArguments[2] !== undefined) parameters.push({ name: 'target', value: rawArguments[2] });
                    description = `ctrl: ${rawArguments[0]}, ${rawArguments[1]}, tgt: ${rawArguments[2]}`;
                    break;

                case 'measure':
                    if (rawArguments[0] !== undefined) parameters.push({ name: 'qubit', value: rawArguments[0] });
                    if (rawArguments[1] !== undefined) parameters.push({ name: 'clbit', value: rawArguments[1] });
                    description = `q: ${rawArguments[0]} → c: ${rawArguments[1]}`;
                    break;

                default:
                    // Ignore non-gate utility methods
                    if (['draw', 'copy', 'qasm', 'cls', 'to_gate'].includes(gateMethod)) {
                        return null;
                    }
                    rawArguments.forEach((argumentValue, argumentIndex) => {
                        parameters.push({ name: `param_${argumentIndex + 1}`, value: argumentValue });
                    });
                    description = rawArguments.join(', ');
                    break;
            }
        }

        return {
            gateName,
            displayName,
            description,
            line,
            column,
            parameters
        };
    }

    private createCircuitItem(circuit: ParsedCircuit): AstNodeItem {
        // Requirement 3: Mostrar nombre de la variable que contiene el circuito
        // Requirement 4: Detectar número de cúbits y bits clásicos y mostrarlos
        const circuitItem = new AstNodeItem(
            circuit.variableName,
            'circuit',
            circuit.line,
            circuit.column,
            vscode.TreeItemCollapsibleState.Collapsed,
            `(${circuit.qubits} qubits, ${circuit.clbits} bits clásicos)`
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
            `Bits clásicos: ${circuit.clbits}`,
            'metadata',
            circuit.line,
            circuit.column,
            vscode.TreeItemCollapsibleState.None
        );

        circuitItem.children.push(qubitsMetadata);
        circuitItem.children.push(clbitsMetadata);

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
                    gate.line,
                    gate.column,
                    vscode.TreeItemCollapsibleState.None
                );
                gateItem.children.push(parameterItem);
            }

            circuitItem.children.push(gateItem);
        }

        return circuitItem;
    }
}
