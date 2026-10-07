use std::io;
use std::process::ExitCode;

use anstream::{AutoStream, ColorChoice};
use qctidy_facade::{ParsePythonRequest, parse_python};
use qctidy_ports::{
    CircuitIssue, GateParameter, ParsedCircuit, ParsedGate, PythonAnalysis, Scope, ScopeChild,
    ScopeType, SourceLocation,
};
use qctidy_python::PythonAdapter;
use serde::Serialize;

use crate::arguments::AstArguments;
use crate::error::CliError;
use crate::input::Input;
use crate::output;

/// Run the `ast` command.
pub(crate) fn run(arguments: &AstArguments, color: ColorChoice) -> ExitCode {
    let mut stderr = AutoStream::new(io::stderr(), color);
    let input = Input::resolve_one(arguments.input.as_deref());

    let json = match analyze(&input) {
        Ok(json) => json,
        Err(error) => {
            output::human_error(&mut stderr, &error);
            return ExitCode::from(2);
        }
    };

    let mut bytes = json.into_bytes();
    bytes.push(b'\n');

    if let Err(error) = output::write_bytes(arguments.output.as_deref(), &bytes) {
        output::human_error(&mut stderr, &error);
        return ExitCode::from(2);
    }

    ExitCode::SUCCESS
}

fn analyze(input: &Input) -> Result<String, CliError> {
    let bytes = input.read().map_err(|error| CliError::Read {
        source_name: input.name().to_owned(),
        error,
    })?;

    let source = String::from_utf8_lossy(&bytes).into_owned();
    let request = ParsePythonRequest::new(source.into());
    let response = parse_python(&request, &PythonAdapter);
    let analysis = to_json(response.analysis());

    serde_json::to_string_pretty(&analysis).map_err(|error| CliError::SerializeJson { error })
}

#[derive(Serialize)]
struct JsonAnalysis {
    scopes: Vec<JsonNode>,
    module_circuits: Vec<JsonCircuit>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum JsonNode {
    Function(JsonScope),
    Class(JsonScope),
    Circuit(JsonCircuit),
}

#[derive(Serialize)]
struct JsonScope {
    name: String,
    line: usize,
    column: usize,
    children: Vec<JsonNode>,
}

#[derive(Serialize)]
struct JsonCircuit {
    name: String,
    qubits: String,
    clbits: String,
    line: usize,
    column: usize,
    issues: Vec<JsonIssue>,
    gates: Vec<JsonGate>,
    source_map: Option<Vec<JsonSourceLocation>>,
}

#[derive(Serialize)]
struct JsonGate {
    display_name: String,
    description: String,
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
    parameters: Vec<JsonParameter>,
}

#[derive(Serialize)]
struct JsonParameter {
    name: String,
    value: String,
    line: usize,
    column: usize,
}

#[derive(Serialize)]
struct JsonIssue {
    message: String,
    line: usize,
    column: usize,
}

#[derive(Serialize)]
struct JsonSourceLocation {
    index: usize,
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
}

#[must_use]
fn to_json(analysis: &PythonAnalysis) -> JsonAnalysis {
    JsonAnalysis {
        scopes: analysis.scopes.iter().map(scope_node).collect(),
        module_circuits: analysis.module_circuits.iter().map(circuit_node).collect(),
    }
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
        qubits: circuit.qubits.clone(),
        clbits: circuit.clbits.clone(),
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
fn issue(issue: &CircuitIssue) -> JsonIssue {
    JsonIssue {
        message: issue.message.clone(),
        line: issue.line,
        column: issue.column,
    }
}

#[must_use]
fn source_location(location: &SourceLocation) -> JsonSourceLocation {
    JsonSourceLocation {
        index: location.index,
        line: location.line,
        column: location.column,
        end_line: location.end_line,
        end_column: location.end_column,
    }
}
