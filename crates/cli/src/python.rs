use qctidy::Circuit;
use qctidy_facade::{ParsePythonRequest, parse_python};
use qctidy_ports::{ParsedCircuit, PythonAnalysis, Scope, ScopeChild, SourceLocation};
use qctidy_python::PythonAdapter;

use crate::error::CliError;

/// A checkable circuit extracted from a Python file.
#[expect(clippy::field_scoped_visibility_modifiers)]
pub(crate) struct CircuitInfo {
    /// The input name followed by the circuit's variable name.
    pub(crate) name: String,
    pub(crate) circuit: Circuit,
    pub(crate) source_map: Vec<SourceLocation>,
}

/// Parse Python source and return one outcome per selected circuit.
///
/// The selector (`--circuit`) picks a single circuit by variable name, or a
/// disambiguated `name:line` when the name is redefined. Without a selector,
/// every circuit is returned.
pub(crate) fn extract_circuits(
    source: &str,
    source_name: &str,
    selector: Option<&str>,
) -> Result<Vec<Result<CircuitInfo, CliError>>, CliError> {
    let circuits = match selector {
        Some(selector) => select(flatten(parse(source)), selector, source_name)?,
        None => flatten(parse(source)),
    };

    Ok(circuits
        .into_iter()
        .map(|circuit| {
            let name = format!("{source_name}:{}", circuit.name);

            match circuit.build {
                Some(build) => Ok(CircuitInfo {
                    name,
                    circuit: build.circuit,
                    source_map: build.source_map,
                }),
                None => {
                    let message = circuit
                        .issues
                        .iter()
                        .map(|issue| issue.message.as_str())
                        .collect::<Vec<_>>()
                        .join("; ");

                    Err(CliError::UncheckableCircuit {
                        source_name: name,
                        message,
                    })
                }
            }
        })
        .collect())
}

fn parse(source: &str) -> PythonAnalysis {
    parse_python(&ParsePythonRequest::new(source.into()), &PythonAdapter)
        .analysis()
        .clone()
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
