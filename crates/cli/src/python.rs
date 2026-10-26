use getset::Getters;
use qctidy::Circuit;
use qctidy_facade::{ParsePythonRequest, parse_python};
use qctidy_ports::{
    GateParameter, ParseIssue, ParsedCircuit, ParsedGate, PythonAnalysis, Scope, ScopeChild,
    ScopeType, SourceLocation,
};
use qctidy_python_ast::PythonAstAdapter;
use serde::Serialize;

use crate::error::CliError;

/// A checkable circuit extracted from a Python file.
#[derive(Getters)]
pub(crate) struct CircuitInfo {
    /// The input name followed by the circuit's variable name.
    #[get = "pub(crate)"]
    name: String,
    /// The circuit's variable name in the source.
    #[get = "pub(crate)"]
    circuit_name: String,
    #[get = "pub(crate)"]
    circuit: Circuit,
    #[get = "pub(crate)"]
    source_map: Vec<SourceLocation>,
}

/// Parse Python source into the circuits and scopes it defines.
pub(crate) fn analyze(source: &str) -> PythonAnalysis {
    parse_python(&ParsePythonRequest::new(source.into()), &PythonAstAdapter)
        .analysis()
        .clone()
}

/// Parse Python source and flatten it into one outcome per selected circuit.
///
/// The selector (`--circuit`) picks a single circuit by variable name, or a
/// disambiguated `name:line` when the name is redefined. Without a selector,
/// every circuit is returned.
pub(crate) fn extract_circuits(
    source: &str,
    source_name: &str,
    selector: Option<&str>,
) -> Result<Vec<Result<CircuitInfo, CliError>>, CliError> {
    circuits(analyze(source), source_name, selector)
}

/// Flatten an analysis into one outcome per selected circuit.
///
/// See [`extract_circuits`] for the selector syntax.
pub(crate) fn circuits(
    analysis: PythonAnalysis,
    source_name: &str,
    selector: Option<&str>,
) -> Result<Vec<Result<CircuitInfo, CliError>>, CliError> {
    let circuits = match selector {
        Some(selector) => select(flatten(analysis), selector, source_name)?,
        None => flatten(analysis),
    };

    Ok(circuits
        .into_iter()
        .map(|circuit| {
            let circuit_name = circuit.name.clone();
            let name = format!("{source_name}:{circuit_name}");

            if let Some(build) = circuit.build {
                Ok(CircuitInfo {
                    name,
                    circuit_name,
                    circuit: build.circuit,
                    source_map: build.source_map,
                })
            } else {
                let message = circuit
                    .issues
                    .iter()
                    .map(|issue| issue.message.as_str())
                    .collect::<Vec<_>>()
                    .join("; ");

                Err(CliError::UncheckableCircuit {
                    filename: source_name.to_owned(),
                    circuit: circuit_name,
                    message,
                })
            }
        })
        .collect())
}

fn flatten(analysis: PythonAnalysis) -> Vec<ParsedCircuit> {
    let mut circuits = Vec::new();

    for scope in analysis.scopes {
        collect(scope, &mut circuits);
    }

    for circuit in analysis.module_circuits {
        circuits.push(circuit);
    }

    circuits
}

fn collect(scope: Scope, circuits: &mut Vec<ParsedCircuit>) {
    for child in scope.children {
        match child {
            ScopeChild::Circuit(circuit) => circuits.push(circuit),
            ScopeChild::Scope(inner) => collect(inner, circuits),
        }
    }
}

fn select(
    circuits: Vec<ParsedCircuit>,
    selector: &str,
    source_name: &str,
) -> Result<Vec<ParsedCircuit>, CliError> {
    let (name, line) = parse_selector(selector);
    let matches: Vec<ParsedCircuit> = circuits
        .into_iter()
        .filter(|circuit| circuit.name == name && line.is_none_or(|line| circuit.line + 1 == line))
        .collect();

    match matches.len() {
        0 => Err(CliError::UnknownCircuit {
            source_name: source_name.to_owned(),
            selector: selector.to_owned(),
        }),
        1 => Ok(matches),
        _ => Err(CliError::AmbiguousCircuit {
            source_name: source_name.to_owned(),
            selector: selector.to_owned(),
        }),
    }
}

/// Split a selector into a variable name and an optional 1-based line.
fn parse_selector(selector: &str) -> (&str, Option<usize>) {
    match selector.rsplit_once(':') {
        Some((name, line))
            if !line.is_empty() && line.bytes().all(|byte| byte.is_ascii_digit()) =>
        {
            (name, line.parse().ok())
        }
        _ => (selector, None),
    }
}

/// The Python analysis of one file, as reported in the JSON output.
#[derive(Debug, Serialize)]
pub(crate) struct JsonAnalysis {
    filename: String,
    scopes: Vec<JsonNode>,
    module_circuits: Vec<JsonCircuit>,
}

impl JsonAnalysis {
    #[must_use]
    pub(crate) fn new(filename: &str, analysis: &PythonAnalysis) -> Self {
        Self {
            filename: filename.to_owned(),
            scopes: analysis.scopes.iter().map(scope_node).collect(),
            module_circuits: analysis.module_circuits.iter().map(circuit_node).collect(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum JsonNode {
    Function(JsonScope),
    Class(JsonScope),
    Circuit(JsonCircuit),
}

#[derive(Debug, Serialize)]
struct JsonScope {
    name: String,
    line: usize,
    column: usize,
    children: Vec<JsonNode>,
}

#[derive(Debug, Serialize)]
struct JsonCircuit {
    name: String,
    qubit_count: String,
    bit_count: String,
    line: usize,
    column: usize,
    issues: Vec<JsonIssue>,
    gates: Vec<JsonGate>,
    source_map: Option<Vec<JsonSourceLocation>>,
}

#[derive(Debug, Serialize)]
struct JsonGate {
    display_name: String,
    description: String,
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
    parameters: Vec<JsonParameter>,
}

#[derive(Debug, Serialize)]
struct JsonParameter {
    name: String,
    value: String,
    line: usize,
    column: usize,
}

#[derive(Debug, Serialize)]
struct JsonIssue {
    message: String,
    line: usize,
    column: usize,
}

#[derive(Debug, Serialize)]
struct JsonSourceLocation {
    index: usize,
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
}

#[must_use]
fn scope_node(scope: &Scope) -> JsonNode {
    let fields = JsonScope {
        name: scope.name.clone(),
        line: scope.line,
        column: scope.column,
        children: scope.children.iter().map(child).collect(),
    };

    match scope.scope_type {
        ScopeType::Function => JsonNode::Function(fields),
        ScopeType::Class => JsonNode::Class(fields),
    }
}

#[must_use]
fn child(child: &ScopeChild) -> JsonNode {
    match child {
        ScopeChild::Scope(scope) => scope_node(scope),
        ScopeChild::Circuit(circuit) => JsonNode::Circuit(circuit_node(circuit)),
    }
}

#[must_use]
fn circuit_node(circuit: &ParsedCircuit) -> JsonCircuit {
    JsonCircuit {
        name: circuit.name.clone(),
        qubit_count: circuit.qubit_count.clone(),
        bit_count: circuit.bit_count.clone(),
        line: circuit.line,
        column: circuit.column,
        issues: circuit.issues.iter().map(issue).collect(),
        gates: circuit.gates.iter().map(gate).collect(),
        source_map: circuit
            .build
            .as_ref()
            .map(|build| build.source_map.iter().map(source_location).collect()),
    }
}

#[must_use]
fn gate(gate: &ParsedGate) -> JsonGate {
    JsonGate {
        display_name: gate.display_name.clone(),
        description: gate.description.clone(),
        line: gate.line,
        column: gate.column,
        end_line: gate.end_line,
        end_column: gate.end_column,
        parameters: gate.parameters.iter().map(parameter).collect(),
    }
}

#[must_use]
fn parameter(parameter: &GateParameter) -> JsonParameter {
    JsonParameter {
        name: parameter.name.clone(),
        value: parameter.value.clone(),
        line: parameter.line,
        column: parameter.column,
    }
}

#[must_use]
fn issue(issue: &ParseIssue) -> JsonIssue {
    JsonIssue {
        message: issue.message.clone(),
        line: issue.line,
        column: issue.column,
    }
}

#[must_use]
const fn source_location(location: &SourceLocation) -> JsonSourceLocation {
    JsonSourceLocation {
        index: location.index,
        line: location.line,
        column: location.column,
        end_line: location.end_line,
        end_column: location.end_column,
    }
}
