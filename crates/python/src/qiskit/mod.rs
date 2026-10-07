//! Extraction of Qiskit circuits from a Python abstract syntax tree.

mod expressions;
mod gates;

use std::collections::HashMap;

use qctidy::Circuit;
use qctidy_ports::{
    CircuitBuild, CircuitIssue, ParsedCircuit, PythonAnalysis, Scope, ScopeChild, ScopeType,
    SourceLocation,
};
use tree_sitter::{Node, Parser};

use gates::{Argument, parse_gate_call};

/// Parse Python source and return the circuits and scopes it defines.
///
/// Mirrors the tree-walking logic of the VS Code extension.
#[must_use]
pub(crate) fn analyze(source: &str) -> PythonAnalysis {
    let mut parser = Parser::new();

    if parser
        .set_language(&tree_sitter_python::LANGUAGE.into())
        .is_err()
    {
        return PythonAnalysis {
            scopes: Vec::new(),
            module_circuits: Vec::new(),
        };
    }

    let Some(tree) = parser.parse(source, None) else {
        return PythonAnalysis {
            scopes: Vec::new(),
            module_circuits: Vec::new(),
        };
    };

    build_hierarchy(tree.root_node(), source)
}

#[must_use]
fn build_hierarchy(root: Node, source: &str) -> PythonAnalysis {
    let mut scopes = Vec::new();

    for child in named_children(root) {
        match child.kind() {
            "function_definition" => scopes.push(process_scope(child, ScopeType::Function, source)),
            "class_definition" => scopes.push(process_scope(child, ScopeType::Class, source)),
            _ => {}
        }
    }

    PythonAnalysis {
        scopes,
        module_circuits: extract_circuits(root, source),
    }
}

#[must_use]
fn process_scope(scope_node: Node, scope_type: ScopeType, source: &str) -> Scope {
    let name = scope_node
        .child_by_field_name("name")
        .map(|name| node_text(name, source))
        .unwrap_or_else(|| {
            if scope_type == ScopeType::Class {
                "Anonymous class".to_string()
            } else {
                "Anonymous function".to_string()
            }
        });

    let mut children = Vec::new();

    if scope_type == ScopeType::Class {
        if let Some(body) = scope_node.child_by_field_name("body") {
            for child in named_children(body) {
                if child.kind() == "function_definition" {
                    children.push(ScopeChild::Scope(process_scope(
                        child,
                        ScopeType::Function,
                        source,
                    )));
                }
            }
        }
    }

    for circuit in extract_circuits(scope_node, source) {
        children.push(ScopeChild::Circuit(circuit));
    }

    let start = scope_node.start_position();

    Scope {
        scope_type,
        name,
        line: start.row,
        column: start.column,
        children,
    }
}

#[must_use]
fn extract_circuits(scope_node: Node, source: &str) -> Vec<ParsedCircuit> {
    let mut circuits = Vec::new();
    let mut variable_map = HashMap::new();

    find_assignments(
        scope_node,
        scope_node,
        source,
        &mut circuits,
        &mut variable_map,
    );
    find_gate_calls(scope_node, scope_node, source, &mut circuits, &variable_map);
    finalize_circuits(&mut circuits);

    circuits
}

fn find_assignments(
    node: Node,
    scope_node: Node,
    source: &str,
    circuits: &mut Vec<ParsedCircuit>,
    variable_map: &mut HashMap<String, Vec<usize>>,
) {
    if node != scope_node && is_scope(node) {
        return;
    }

    if node.kind() == "assignment" {
        let left = node.child_by_field_name("left");
        let right = node.child_by_field_name("right");

        if let (Some(left), Some(right)) = (left, right) {
            if right.kind() == "call" {
                if let Some(function) = right.child_by_field_name("function") {
                    let function_text = node_text(function, source);

                    if function_text == "QuantumCircuit"
                        || function_text.ends_with(".QuantumCircuit")
                    {
                        let variable_name = node_text(left, source);
                        let arguments = right.child_by_field_name("arguments");
                        let (qubits, clbits) = circuit_counts(arguments, source);
                        let start = node.start_position();

                        let circuit = ParsedCircuit {
                            name: variable_name.clone(),
                            qubits,
                            clbits,
                            line: start.row,
                            column: start.column,
                            gates: Vec::new(),
                            issues: Vec::new(),
                            build: None,
                        };

                        let index = circuits.len();
                        circuits.push(circuit);
                        variable_map.entry(variable_name).or_default().push(index);
                    }
                }
            }
        }
    }

    for child in named_children(node) {
        find_assignments(child, scope_node, source, circuits, variable_map);
    }
}

fn find_gate_calls(
    node: Node,
    scope_node: Node,
    source: &str,
    circuits: &mut [ParsedCircuit],
    variable_map: &HashMap<String, Vec<usize>>,
) {
    if node != scope_node && is_scope(node) {
        return;
    }

    if node.kind() == "call" {
        if let Some(function) = node.child_by_field_name("function") {
            if function.kind() == "attribute" {
                let object = function.child_by_field_name("object");
                let attribute = function.child_by_field_name("attribute");

                if let (Some(object), Some(attribute)) = (object, attribute) {
                    let object_text = node_text(object, source);

                    if let Some(candidates) = variable_map.get(&object_text) {
                        let call_line = node.start_position().row;
                        let mut target = candidates[0];

                        for &candidate in candidates {
                            if circuits[candidate].line <= call_line {
                                target = candidate;
                            }
                        }

                        let gate_method = node_text(attribute, source);
                        let arguments = node.child_by_field_name("arguments");
                        let call_arguments = collect_arguments(arguments, source);
                        let start = node.start_position();
                        let end = node.end_position();

                        if let Some(gate) = parse_gate_call(
                            &gate_method,
                            &call_arguments,
                            start.row,
                            start.column,
                            end.row,
                            end.column,
                        ) {
                            circuits[target].gates.push(gate);
                        }
                    }
                }
            }
        }
    }

    for child in named_children(node) {
        find_gate_calls(child, scope_node, source, circuits, variable_map);
    }
}

#[must_use]
fn collect_arguments(arguments: Option<Node>, source: &str) -> Vec<Argument> {
    let Some(arguments) = arguments else {
        return Vec::new();
    };

    named_children(arguments)
        .into_iter()
        .map(|argument| {
            let position = argument.start_position();

            Argument::new(node_text(argument, source), position.row, position.column)
        })
        .collect()
}

#[must_use]
fn circuit_counts(arguments: Option<Node>, source: &str) -> (String, String) {
    let mut qubits = "0".to_string();
    let mut clbits = "0".to_string();

    let Some(arguments) = arguments else {
        return (qubits, clbits);
    };

    let mut positional = Vec::new();

    for child in named_children(arguments) {
        if child.kind() == "keyword_argument" {
            let key_name = child
                .child_by_field_name("name")
                .map(|name| node_text(name, source));
            let value = child
                .child_by_field_name("value")
                .map(|value| node_text(value, source));

            match key_name.as_deref() {
                Some("qubits" | "num_qubits") => {
                    qubits = value.unwrap_or_else(|| "0".to_string());
                }
                Some("clbits" | "num_clbits") => {
                    clbits = value.unwrap_or_else(|| "0".to_string());
                }
                _ => {}
            }
        } else {
            positional.push(node_text(child, source));
        }
    }

    if let Some(positional_qubits) = positional.first() {
        qubits = positional_qubits.clone();
    }

    if let Some(positional_clbits) = positional.get(1) {
        clbits = positional_clbits.clone();
    }

    (qubits, clbits)
}

fn finalize_circuits(circuits: &mut [ParsedCircuit]) {
    for circuit in circuits {
        let issues = compute_issues(circuit);
        circuit.build = if issues.is_empty() {
            build_circuit(circuit)
        } else {
            None
        };
        circuit.issues = issues;
    }
}

#[must_use]
fn compute_issues(circuit: &ParsedCircuit) -> Vec<CircuitIssue> {
    let mut issues = Vec::new();

    if expressions::evaluate_integer(&circuit.qubits).is_none() {
        issues.push(CircuitIssue {
            message: format!("qubit count is not a number: {}", circuit.qubits),
            line: circuit.line,
            column: circuit.column,
        });
    }

    if expressions::evaluate_integer(&circuit.clbits).is_none() {
        issues.push(CircuitIssue {
            message: format!("classical bit count is not a number: {}", circuit.clbits),
            line: circuit.line,
            column: circuit.column,
        });
    }

    for gate in &circuit.gates {
        if gate.operation.is_none() {
            issues.push(CircuitIssue {
                message: gate
                    .reason
                    .clone()
                    .unwrap_or_else(|| format!("unsupported gate: {}", gate.gate_name)),
                line: gate.line,
                column: gate.column,
            });
        }
    }

    issues
}

fn build_circuit(circuit: &ParsedCircuit) -> Option<CircuitBuild> {
    let qubit_count = expressions::evaluate_integer(&circuit.qubits)?;
    let mut operations = Vec::new();
    let mut source_map = Vec::new();

    for gate in &circuit.gates {
        let operation = gate.operation?;

        source_map.push(SourceLocation {
            index: operations.len(),
            line: gate.line,
            column: gate.column,
            end_line: gate.end_line,
            end_column: gate.end_column,
        });
        operations.push(operation);
    }

    Some(CircuitBuild {
        circuit: Circuit::new(qubit_count, operations),
        source_map,
    })
}

#[must_use]
fn node_text(node: Node, source: &str) -> String {
    node.utf8_text(source.as_bytes()).unwrap_or("").to_string()
}

#[must_use]
fn is_scope(node: Node) -> bool {
    matches!(node.kind(), "function_definition" | "class_definition")
}

#[must_use]
fn named_children(node: Node) -> Vec<Node> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor).collect()
}

#[cfg(test)]
mod tests;
