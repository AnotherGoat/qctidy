use qctidy::GateOperation;
use qctidy_ports::{GateParameter, ParsedGate};

use crate::qiskit::expressions::{evaluate_integer, evaluate_number};

/// A call argument and the source position it occupies.
pub(crate) struct Argument {
    text: String,
    line: usize,
    column: usize,
}

impl Argument {
    #[must_use]
    pub(crate) fn new(text: String, line: usize, column: usize) -> Self {
        Self { text, line, column }
    }
}

/// Map a gate call on a circuit to a parsed gate, mirroring the VS Code parser.
///
/// Returns `None` for utility methods that are not gates, such as `draw`.
pub(crate) fn parse_gate_call(
    gate_method: &str,
    args: &[Argument],
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
) -> Option<ParsedGate> {
    let raw: Vec<&str> = args.iter().map(|argument| argument.text.as_str()).collect();
    let mut gate_name = gate_method.to_string();
    let mut display_name = gate_method.to_string();
    let mut description = String::new();
    let mut operation = None;
    let mut reason = None;
    let mut parameters = Vec::new();

    let integer = |index: usize| raw.get(index).copied().and_then(evaluate_integer);
    let number = |index: usize| raw.get(index).copied().and_then(evaluate_number);

    if gate_method == "append" {
        let first_argument = raw.first().copied().unwrap_or("");
        let qubit = raw.get(1).and_then(|text| extract_first_integer(text));
        let is_sqrt_y = first_argument.contains("YGate") && first_argument.contains("power");

        match (is_sqrt_y, qubit) {
            (true, Some(qubit)) => {
                gate_name = "sqrt(Y)".to_string();
                display_name = "sqrt(Y)".to_string();
                operation = Some(GateOperation::sy(qubit));

                let mut power_value = "1/2".to_string();
                if let Some(inner) = extract_power(first_argument) {
                    power_value = inner.trim().to_string();
                }

                add_parameter(
                    &mut parameters,
                    args,
                    "power",
                    0,
                    Some(&power_value),
                    line,
                    column,
                );
                add_parameter(&mut parameters, args, "qubits", 1, None, line, column);
                description = format!("power: {power_value}, q: {}", raw_arg(&raw, 1));
            }
            _ => {
                display_name = "append".to_string();
                append_parameters(&mut parameters, args, line, column);
                description = raw.join(", ");
                reason = Some("unsupported append call".to_string());
            }
        }
    } else {
        let method = gate_method.to_lowercase();

        match method.as_str() {
            "id" | "h" | "x" | "y" | "z" | "s" | "sdg" | "sx" | "t" | "tdg" => {
                if !raw.is_empty() {
                    add_parameter(&mut parameters, args, "qubit", 0, None, line, column);
                    description = format!("q: {}", raw[0]);
                }

                if let Some(qubit) = integer(0) {
                    operation = Some(single_qubit_operation(&method, qubit));
                }
            }
            "p" | "rx" | "ry" => {
                if !raw.is_empty() {
                    add_parameter(&mut parameters, args, "theta", 0, None, line, column);
                }
                if raw.get(1).is_some() {
                    add_parameter(&mut parameters, args, "qubit", 1, None, line, column);
                }

                description = format!("θ: {}, q: {}", raw_arg(&raw, 0), raw_arg(&raw, 1));

                if let (Some(theta), Some(qubit)) = (number(0), integer(1)) {
                    operation = Some(match method.as_str() {
                        "p" => GateOperation::P { theta, qubit },
                        "rx" => GateOperation::RX { theta, qubit },
                        _ => GateOperation::RY { theta, qubit },
                    });
                }
            }
            "rz" => {
                if !raw.is_empty() {
                    add_parameter(&mut parameters, args, "theta", 0, None, line, column);
                }
                if raw.get(1).is_some() {
                    add_parameter(&mut parameters, args, "qubit", 1, None, line, column);
                }

                description = format!("θ: {}, q: {}", raw_arg(&raw, 0), raw_arg(&raw, 1));

                if let (Some(phi), Some(qubit)) = (number(0), integer(1)) {
                    operation = Some(GateOperation::RZ { phi, qubit });
                }
            }
            "u" => {
                if !raw.is_empty() {
                    add_parameter(&mut parameters, args, "theta", 0, None, line, column);
                }
                if raw.get(1).is_some() {
                    add_parameter(&mut parameters, args, "phi", 1, None, line, column);
                }
                if raw.get(2).is_some() {
                    add_parameter(&mut parameters, args, "lam", 2, None, line, column);
                }
                if raw.get(3).is_some() {
                    add_parameter(&mut parameters, args, "qubit", 3, None, line, column);
                }

                description = format!(
                    "θ: {}, φ: {}, λ: {}, q: {}",
                    raw_arg(&raw, 0),
                    raw_arg(&raw, 1),
                    raw_arg(&raw, 2),
                    raw_arg(&raw, 3),
                );

                if let (Some(theta), Some(phi), Some(lambda), Some(qubit)) =
                    (number(0), number(1), number(2), integer(3))
                {
                    operation = Some(GateOperation::U {
                        theta,
                        phi,
                        lambda,
                        qubit,
                    });
                }
            }
            "swap" | "cz" => {
                if !raw.is_empty() {
                    add_parameter(&mut parameters, args, "qubit 1", 0, None, line, column);
                }
                if raw.get(1).is_some() {
                    add_parameter(&mut parameters, args, "qubit 2", 1, None, line, column);
                }

                description = format!("q1: {}, q2: {}", raw_arg(&raw, 0), raw_arg(&raw, 1));

                if let (Some(qubit1), Some(qubit2)) = (integer(0), integer(1)) {
                    operation = Some(match method.as_str() {
                        "swap" => GateOperation::Swap { qubit1, qubit2 },
                        _ => GateOperation::CZ { qubit1, qubit2 },
                    });
                }
            }
            "ch" | "cx" | "cy" => {
                if !raw.is_empty() {
                    add_parameter(&mut parameters, args, "control", 0, None, line, column);
                }
                if raw.get(1).is_some() {
                    add_parameter(&mut parameters, args, "target", 1, None, line, column);
                }

                description = format!("ctrl: {}, tgt: {}", raw_arg(&raw, 0), raw_arg(&raw, 1));

                if let (Some(control), Some(target)) = (integer(0), integer(1)) {
                    operation = Some(match method.as_str() {
                        "ch" => GateOperation::CH { control, target },
                        "cx" => GateOperation::CX { control, target },
                        _ => GateOperation::CY { control, target },
                    });
                }
            }
            "cp" => {
                if !raw.is_empty() {
                    add_parameter(&mut parameters, args, "theta", 0, None, line, column);
                }
                if raw.get(1).is_some() {
                    add_parameter(&mut parameters, args, "control", 1, None, line, column);
                }
                if raw.get(2).is_some() {
                    add_parameter(&mut parameters, args, "target", 2, None, line, column);
                }

                description = format!(
                    "θ: {}, ctrl: {}, tgt: {}",
                    raw_arg(&raw, 0),
                    raw_arg(&raw, 1),
                    raw_arg(&raw, 2),
                );

                if let (Some(theta), Some(qubit1), Some(qubit2)) =
                    (number(0), integer(1), integer(2))
                {
                    operation = Some(GateOperation::CP {
                        theta,
                        qubit1,
                        qubit2,
                    });
                }
            }
            "cswap" => {
                if !raw.is_empty() {
                    add_parameter(&mut parameters, args, "control", 0, None, line, column);
                }
                if raw.get(1).is_some() {
                    add_parameter(&mut parameters, args, "target 1", 1, None, line, column);
                }
                if raw.get(2).is_some() {
                    add_parameter(&mut parameters, args, "target 2", 2, None, line, column);
                }

                description = format!(
                    "ctrl: {}, tgt: {}, {}",
                    raw_arg(&raw, 0),
                    raw_arg(&raw, 1),
                    raw_arg(&raw, 2),
                );

                if let (Some(control), Some(target1), Some(target2)) =
                    (integer(0), integer(1), integer(2))
                {
                    operation = Some(GateOperation::CSwap {
                        control,
                        target1,
                        target2,
                    });
                }
            }
            "ccx" => {
                if !raw.is_empty() {
                    add_parameter(&mut parameters, args, "control 1", 0, None, line, column);
                }
                if raw.get(1).is_some() {
                    add_parameter(&mut parameters, args, "control 2", 1, None, line, column);
                }
                if raw.get(2).is_some() {
                    add_parameter(&mut parameters, args, "target", 2, None, line, column);
                }

                description = format!(
                    "ctrl: {}, {}, tgt: {}",
                    raw_arg(&raw, 0),
                    raw_arg(&raw, 1),
                    raw_arg(&raw, 2),
                );

                if let (Some(control1), Some(control2), Some(target)) =
                    (integer(0), integer(1), integer(2))
                {
                    operation = Some(GateOperation::CCX {
                        control1,
                        control2,
                        target,
                    });
                }
            }
            "ccz" => {
                if !raw.is_empty() {
                    add_parameter(&mut parameters, args, "control 1", 0, None, line, column);
                }
                if raw.get(1).is_some() {
                    add_parameter(&mut parameters, args, "control 2", 1, None, line, column);
                }
                if raw.get(2).is_some() {
                    add_parameter(&mut parameters, args, "target", 2, None, line, column);
                }

                description = format!(
                    "ctrl: {}, {}, tgt: {}",
                    raw_arg(&raw, 0),
                    raw_arg(&raw, 1),
                    raw_arg(&raw, 2),
                );

                if let (Some(qubit1), Some(qubit2), Some(qubit3)) =
                    (integer(0), integer(1), integer(2))
                {
                    operation = Some(GateOperation::CCZ {
                        qubit1,
                        qubit2,
                        qubit3,
                    });
                }
            }
            "measure" => {
                if !raw.is_empty() {
                    add_parameter(&mut parameters, args, "qubit", 0, None, line, column);
                }
                if raw.get(1).is_some() {
                    add_parameter(&mut parameters, args, "clbit", 1, None, line, column);
                }

                description = format!("q: {} → c: {}", raw_arg(&raw, 0), raw_arg(&raw, 1));

                if let (Some(qubit), Some(bit)) = (integer(0), integer(1)) {
                    operation = Some(GateOperation::Measure { qubit, bit });
                }
            }
            _ => {
                if matches!(gate_method, "draw" | "copy" | "qasm" | "cls" | "to_gate") {
                    return None;
                }

                for index in 0..raw.len() {
                    add_parameter(
                        &mut parameters,
                        args,
                        &format!("param_{}", index + 1),
                        index,
                        None,
                        line,
                        column,
                    );
                }

                description = raw.join(", ");
                reason = Some(format!("unsupported gate: {gate_method}"));
            }
        }
    }

    if operation.is_none() && reason.is_none() {
        reason = Some(format!("unsupported arguments: {}", raw.join(", ")));
    }

    Some(ParsedGate {
        gate_name,
        display_name,
        description,
        line,
        column,
        end_line,
        end_column,
        parameters,
        operation,
        reason,
    })
}

#[expect(clippy::unreachable)]
fn single_qubit_operation(method: &str, qubit: usize) -> GateOperation {
    match method {
        "id" => GateOperation::id(qubit),
        "h" => GateOperation::h(qubit),
        "x" => GateOperation::x(qubit),
        "y" => GateOperation::y(qubit),
        "z" => GateOperation::z(qubit),
        "s" => GateOperation::s(qubit),
        "sdg" => GateOperation::sdg(qubit),
        "sx" => GateOperation::sx(qubit),
        "t" => GateOperation::t(qubit),
        "tdg" => GateOperation::tdg(qubit),
        _ => unreachable!("only single-qubit gate methods reach this helper"),
    }
}

#[must_use]
fn raw_arg<'a>(raw: &[&'a str], index: usize) -> &'a str {
    raw.get(index).copied().unwrap_or("undefined")
}

fn add_parameter(
    parameters: &mut Vec<GateParameter>,
    args: &[Argument],
    name: &str,
    index: usize,
    value: Option<&str>,
    line: usize,
    column: usize,
) {
    let argument = args.get(index);
    let value = value
        .or_else(|| argument.map(|argument| argument.text.as_str()))
        .unwrap_or("");

    parameters.push(GateParameter {
        name: name.to_string(),
        value: value.to_string(),
        line: argument.map_or(line, |argument| argument.line),
        column: argument.map_or(column, |argument| argument.column),
    });
}

fn append_parameters(
    parameters: &mut Vec<GateParameter>,
    args: &[Argument],
    line: usize,
    column: usize,
) {
    if !args.is_empty() {
        add_parameter(parameters, args, "gate", 0, None, line, column);
    }

    if args.len() > 1 {
        add_parameter(parameters, args, "qubits", 1, None, line, column);
    }

    if args.len() > 2 {
        add_parameter(parameters, args, "clbits", 2, None, line, column);
    }
}

fn extract_first_integer(text: &str) -> Option<usize> {
    let digits: String = text
        .chars()
        .skip_while(|character| !character.is_ascii_digit())
        .take_while(|character| character.is_ascii_digit())
        .collect();

    digits.parse().ok()
}

fn extract_power(text: &str) -> Option<&str> {
    let start = text.find("power(")? + "power(".len();
    let rest = text.get(start..)?;
    let end = rest.find(')')?;

    rest.get(..end)
}
