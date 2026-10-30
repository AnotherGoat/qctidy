//! Extraction of Qiskit circuits from a Python abstract syntax tree.

mod expressions;
mod gates;

use std::collections::HashMap;

use qctidy::Circuit;
use qctidy_ports::{
    CircuitBuild, ParseIssue, ParsedCircuit, PythonAnalysis, Scope, ScopeChild, ScopeType,
    SourceLocation,
};
use tree_sitter::{Node, Parser, TreeCursor};

use gates::{Argument, parse_gate_call, parse_measure_all};

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
    let name = scope_node.child_by_field_name("name").map_or_else(
        || {
            if scope_type == ScopeType::Class {
                "Anonymous class".to_owned()
            } else {
                "Anonymous function".to_owned()
            }
        },
        |name| node_text(name, source),
    );

    let mut children = Vec::new();

    if scope_type == ScopeType::Class
        && let Some(body) = scope_node.child_by_field_name("body")
    {
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

    find_assignments(scope_node, source, &mut circuits, &mut variable_map);
    find_gate_calls(scope_node, source, &mut circuits, &variable_map);
    finalize_circuits(&mut circuits);

    circuits
}

fn find_assignments(
    scope_node: Node,
    source: &str,
    circuits: &mut Vec<ParsedCircuit>,
    variable_map: &mut HashMap<String, Vec<usize>>,
) {
    let mut constants: HashMap<String, String> = HashMap::new();

    walk_named(scope_node, |node| {
        if node.kind() == "augmented_assignment" {
            if let Some(left) = node.child_by_field_name("left")
                && left.kind() == "identifier"
            {
                constants.remove(&node_text(left, source));
            }

            return;
        }

        if node.kind() != "assignment" {
            return;
        }

        let left = node.child_by_field_name("left");
        let right = node.child_by_field_name("right");

        let (Some(left), Some(right)) = (left, right) else {
            return;
        };

        if left.kind() == "identifier" {
            let name = node_text(left, source);

            if !is_conditional(node, scope_node)
                && let Some(value) = constant_text(right, source, &constants)
            {
                constants.insert(name, value);
            } else {
                constants.remove(&name);
            }
        }

        if right.kind() == "call"
            && let Some(function) = right.child_by_field_name("function")
        {
            let function_text = node_text(function, source);

            if function_text == "QuantumCircuit" || function_text.ends_with(".QuantumCircuit") {
                let variable_name = node_text(left, source);
                let arguments = right.child_by_field_name("arguments");
                let (qubit_count, bit_count) = circuit_counts(arguments, source, &constants);
                let start = node.start_position();

                let circuit = ParsedCircuit {
                    name: variable_name.clone(),
                    qubit_count,
                    bit_count,
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
    });
}

fn find_gate_calls(
    scope_node: Node,
    source: &str,
    circuits: &mut [ParsedCircuit],
    variable_map: &HashMap<String, Vec<usize>>,
) {
    walk_named(scope_node, |node| {
        if node.kind() != "call" {
            return;
        }

        let Some(function) = node.child_by_field_name("function") else {
            return;
        };

        if function.kind() != "attribute" {
            return;
        }

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

                let gates = if gate_method == "measure_all" && call_arguments.is_empty() {
                    let qubit_count = expressions::evaluate_integer(&circuits[target].qubit_count);
                    let bit_count = expressions::evaluate_integer(&circuits[target].bit_count);
                    let gates = parse_measure_all(
                        qubit_count,
                        bit_count.unwrap_or(0),
                        start.row,
                        start.column,
                        end.row,
                        end.column,
                    );

                    if let (Some(qubit_count), Some(bit_count)) = (qubit_count, bit_count) {
                        circuits[target].bit_count = (bit_count + qubit_count).to_string();
                    }

                    gates
                } else {
                    parse_gate_call(
                        &gate_method,
                        &call_arguments,
                        start.row,
                        start.column,
                        end.row,
                        end.column,
                    )
                    .into_iter()
                    .collect()
                };

                circuits[target].gates.extend(gates);
            }
        }
    });
}

/// Visit `scope_node` and its named descendants in pre-order with a single cursor.
///
/// Nested scopes are visited but not descended into, matching the previous
/// recursive walk.
fn walk_named(scope_node: Node, mut visit: impl FnMut(Node)) {
    let mut cursor = scope_node.walk();

    loop {
        let node = cursor.node();

        if node == scope_node || !is_scope(node) {
            visit(node);

            if goto_first_named_child(&mut cursor) {
                continue;
            }
        }

        loop {
            if cursor.node() == scope_node {
                return;
            }

            if !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    return;
                }

                continue;
            }

            if cursor.node().is_named() {
                break;
            }
        }
    }
}

fn goto_first_named_child(cursor: &mut TreeCursor) -> bool {
    if !cursor.goto_first_child() {
        return false;
    }

    loop {
        if cursor.node().is_named() {
            return true;
        }

        if !cursor.goto_next_sibling() {
            return false;
        }
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
fn circuit_counts(
    arguments: Option<Node>,
    source: &str,
    constants: &HashMap<String, String>,
) -> (String, String) {
    let mut qubit_count = "0".to_owned();
    let mut bit_count = "0".to_owned();

    let Some(arguments) = arguments else {
        return (qubit_count, bit_count);
    };

    let mut positional = Vec::new();

    for child in named_children(arguments) {
        if child.kind() == "keyword_argument" {
            let key_name = child
                .child_by_field_name("name")
                .map(|name| node_text(name, source));
            let value = child
                .child_by_field_name("value")
                .map(|value| resolve_constant(&node_text(value, source), constants));

            match key_name.as_deref() {
                Some("qubits" | "num_qubits") => {
                    qubit_count = value.unwrap_or_else(|| "0".to_owned());
                }
                Some("clbits" | "num_clbits") => {
                    bit_count = value.unwrap_or_else(|| "0".to_owned());
                }
                _ => {}
            }
        } else {
            positional.push(resolve_constant(&node_text(child, source), constants));
        }
    }

    if let Some(positional_qubit_count) = positional.first() {
        qubit_count = positional_qubit_count.clone();
    }

    if let Some(positional_bit_count) = positional.get(1) {
        bit_count = positional_bit_count.clone();
    }

    (qubit_count, bit_count)
}

/// Whether `node` sits inside control flow between it and `scope_node`.
///
/// Assignments in a branch may not run, so their values are never propagated.
fn is_conditional(node: Node, scope_node: Node) -> bool {
    let mut current = node.parent();

    while let Some(parent) = current {
        if parent == scope_node {
            return false;
        }

        if matches!(
            parent.kind(),
            "if_statement"
                | "elif_clause"
                | "else_clause"
                | "for_statement"
                | "while_statement"
                | "try_statement"
                | "except_clause"
                | "finally_clause"
                | "with_statement"
                | "match_statement"
                | "case_clause"
        ) {
            return true;
        }

        current = parent.parent();
    }

    false
}

/// The value of a trivial constant assignment: an integer literal or an alias of one.
fn constant_text(node: Node, source: &str, constants: &HashMap<String, String>) -> Option<String> {
    match node.kind() {
        "integer" => {
            let text = node_text(node, source);

            text.bytes()
                .all(|byte| byte.is_ascii_digit())
                .then_some(text)
        }
        "identifier" => constants.get(&node_text(node, source)).cloned(),
        _ => None,
    }
}

/// Replace a bare identifier with its constant value, leaving anything else as is.
fn resolve_constant(text: &str, constants: &HashMap<String, String>) -> String {
    constants
        .get(text.trim())
        .cloned()
        .unwrap_or_else(|| text.to_owned())
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
fn compute_issues(circuit: &ParsedCircuit) -> Vec<ParseIssue> {
    let mut issues = Vec::new();

    if expressions::evaluate_integer(&circuit.qubit_count).is_none() {
        issues.push(ParseIssue {
            message: format!("qubit count is not a number: {}", circuit.qubit_count),
            line: circuit.line,
            column: circuit.column,
        });
    }

    if expressions::evaluate_integer(&circuit.bit_count).is_none() {
        issues.push(ParseIssue {
            message: format!("classical bit count is not a number: {}", circuit.bit_count),
            line: circuit.line,
            column: circuit.column,
        });
    }

    for gate in &circuit.gates {
        if gate.operation.is_none() {
            issues.push(ParseIssue {
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
    let qubit_count = expressions::evaluate_integer(&circuit.qubit_count)?;
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
    node.utf8_text(source.as_bytes()).unwrap_or("").to_owned()
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

#[cfg(test)]
mod expressions_tests;
