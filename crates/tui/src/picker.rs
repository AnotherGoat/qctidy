use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use qctidy::{Circuit, GateOperation, GateOperationError, GateType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgKind {
    Qubit,
    Angle,
    Bit,
}

#[derive(Debug, Clone)]
pub struct ArgPrompt {
    pub kind: ArgKind,
    pub prompt: &'static str,
}

#[derive(Debug, Clone)]
pub struct GateEntry {
    pub gate_type: GateType,
    pub names: &'static [&'static str],
    pub qubits: &'static str,
}

pub(crate) const ALL_ENTRIES: &[GateEntry] = &[
    GateEntry {
        gate_type: GateType::ID,
        names: &["id", "i", "identity"],
        qubits: "1",
    },
    GateEntry {
        gate_type: GateType::H,
        names: &["h", "hadamard"],
        qubits: "1",
    },
    GateEntry {
        gate_type: GateType::X,
        names: &["x", "not"],
        qubits: "1",
    },
    GateEntry {
        gate_type: GateType::Y,
        names: &["y"],
        qubits: "1",
    },
    GateEntry {
        gate_type: GateType::Z,
        names: &["z"],
        qubits: "1",
    },
    GateEntry {
        gate_type: GateType::P,
        names: &["p", "phase"],
        qubits: "1 + theta",
    },
    GateEntry {
        gate_type: GateType::RX,
        names: &["rx"],
        qubits: "1 + theta",
    },
    GateEntry {
        gate_type: GateType::RY,
        names: &["ry"],
        qubits: "1 + theta",
    },
    GateEntry {
        gate_type: GateType::RZ,
        names: &["rz"],
        qubits: "1 + phi",
    },
    GateEntry {
        gate_type: GateType::S,
        names: &["s", "sz", "sqrtz"],
        qubits: "1",
    },
    GateEntry {
        gate_type: GateType::SDG,
        names: &["sdg", "sd", "szd", "szdg", "sqrtzd", "sqrtzdg"],
        qubits: "1",
    },
    GateEntry {
        gate_type: GateType::SX,
        names: &["sx", "sqrtx"],
        qubits: "1",
    },
    GateEntry {
        gate_type: GateType::SY,
        names: &["sy", "sqrty"],
        qubits: "1",
    },
    GateEntry {
        gate_type: GateType::T,
        names: &["t"],
        qubits: "1",
    },
    GateEntry {
        gate_type: GateType::TDG,
        names: &["tdg", "td"],
        qubits: "1",
    },
    GateEntry {
        gate_type: GateType::Measure,
        names: &["m", "measure"],
        qubits: "1 + bit",
    },
    GateEntry {
        gate_type: GateType::Swap,
        names: &["swap"],
        qubits: "2",
    },
    GateEntry {
        gate_type: GateType::CH,
        names: &["ch"],
        qubits: "2",
    },
    GateEntry {
        gate_type: GateType::CX,
        names: &["cx", "cnot"],
        qubits: "2",
    },
    GateEntry {
        gate_type: GateType::CY,
        names: &["cy"],
        qubits: "2",
    },
    GateEntry {
        gate_type: GateType::CZ,
        names: &["cz"],
        qubits: "2",
    },
    GateEntry {
        gate_type: GateType::CP,
        names: &["cp", "cphase"],
        qubits: "2 + theta",
    },
    GateEntry {
        gate_type: GateType::CSwap,
        names: &["cswap", "fredkin"],
        qubits: "3",
    },
    GateEntry {
        gate_type: GateType::CCX,
        names: &["ccx", "ccnot", "toffoli"],
        qubits: "3",
    },
    GateEntry {
        gate_type: GateType::CCZ,
        names: &["ccz"],
        qubits: "3",
    },
];

pub fn filter_entries(query: &str) -> Vec<usize> {
    if query.is_empty() {
        return (0..ALL_ENTRIES.len()).collect();
    }

    let matcher = SkimMatcherV2::default();
    let mut scored: Vec<(i64, usize)> = ALL_ENTRIES
        .iter()
        .enumerate()
        .filter_map(|(idx, entry)| {
            let best = entry
                .names
                .iter()
                .filter_map(|name| matcher.fuzzy_match(name, query))
                .max();
            best.map(|score| (score, idx))
        })
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, idx)| idx).collect()
}

pub fn gate_signature(gate_type: GateType) -> Vec<ArgPrompt> {
    match gate_type {
        GateType::ID
        | GateType::H
        | GateType::X
        | GateType::Y
        | GateType::Z
        | GateType::S
        | GateType::SDG
        | GateType::SX
        | GateType::SY
        | GateType::T
        | GateType::TDG => {
            vec![ArgPrompt {
                kind: ArgKind::Qubit,
                prompt: "qubit",
            }]
        }
        GateType::P | GateType::RX | GateType::RY => {
            vec![
                ArgPrompt {
                    kind: ArgKind::Angle,
                    prompt: "theta (e.g., pi/2, 1.5)",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "qubit",
                },
            ]
        }
        GateType::RZ => {
            vec![
                ArgPrompt {
                    kind: ArgKind::Angle,
                    prompt: "phi (e.g., pi/2, 1.5)",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "qubit",
                },
            ]
        }
        GateType::U => {
            vec![
                ArgPrompt {
                    kind: ArgKind::Angle,
                    prompt: "theta (e.g., pi/2, 1.5)",
                },
                ArgPrompt {
                    kind: ArgKind::Angle,
                    prompt: "phi (e.g., pi/2, 1.5)",
                },
                ArgPrompt {
                    kind: ArgKind::Angle,
                    prompt: "lambda (e.g., pi/2, 1.5)",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "qubit",
                },
            ]
        }
        GateType::CH | GateType::CX | GateType::CY => {
            vec![
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "control qubit",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "target qubit",
                },
            ]
        }
        GateType::Swap | GateType::CZ => {
            vec![
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "first qubit",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "second qubit",
                },
            ]
        }
        GateType::CP => {
            vec![
                ArgPrompt {
                    kind: ArgKind::Angle,
                    prompt: "theta (e.g., pi/2, 1.5)",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "first qubit",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "second qubit",
                },
            ]
        }
        GateType::CSwap => {
            vec![
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "control qubit",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "first target qubit",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "second target qubit",
                },
            ]
        }
        GateType::CCX => {
            vec![
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "first control qubit",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "second control qubit",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "target qubit",
                },
            ]
        }
        GateType::CCZ => {
            vec![
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "first qubit",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "second qubit",
                },
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "third qubit",
                },
            ]
        }
        GateType::Measure => {
            vec![
                ArgPrompt {
                    kind: ArgKind::Qubit,
                    prompt: "qubit",
                },
                ArgPrompt {
                    kind: ArgKind::Bit,
                    prompt: "classical bit",
                },
            ]
        }
    }
}

#[derive(Debug, Clone)]
pub enum AddState {
    Selecting {
        query: String,
        indices: Vec<usize>,
        selected: usize,
    },
    EnteringArgs {
        gate_type: GateType,
        args: Vec<ArgPrompt>,
        current_arg: usize,
        values: Vec<String>,
    },
}

impl AddState {
    pub fn new() -> Self {
        AddState::Selecting {
            query: String::new(),
            indices: (0..ALL_ENTRIES.len()).collect(),
            selected: 0,
        }
    }

    fn reselect(&mut self) {
        if let AddState::Selecting {
            query,
            indices,
            selected,
        } = self
        {
            let old_type = indices
                .get(*selected)
                .copied()
                .and_then(|i| ALL_ENTRIES.get(i));
            let filtered = filter_entries(query);
            *selected = old_type
                .and_then(|old| {
                    filtered
                        .iter()
                        .position(|&i| ALL_ENTRIES[i].gate_type == old.gate_type)
                })
                .unwrap_or(0);
            *indices = filtered;
        }
    }

    pub fn handle_char(&mut self, c: char) {
        match self {
            AddState::Selecting { query, .. } => {
                query.push(c);
                self.reselect();
            }
            AddState::EnteringArgs {
                values,
                current_arg,
                ..
            } => {
                if let Some(value) = values.get_mut(*current_arg) {
                    value.push(c);
                }
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        match self {
            AddState::Selecting { query, .. } => {
                query.pop();
                self.reselect();
            }
            AddState::EnteringArgs {
                values,
                current_arg,
                ..
            } => {
                if let Some(value) = values.get_mut(*current_arg) {
                    value.pop();
                }
            }
        }
    }

    pub fn handle_tab(&mut self) {
        match self {
            AddState::Selecting {
                indices, selected, ..
            } => {
                if !indices.is_empty() {
                    *selected = (*selected + 1) % indices.len();
                }
            }
            AddState::EnteringArgs {
                args, current_arg, ..
            } => {
                if !args.is_empty() {
                    *current_arg = (*current_arg + 1) % args.len();
                }
            }
        }
    }

    pub fn handle_up(&mut self) {
        match self {
            AddState::Selecting {
                indices, selected, ..
            } => {
                if !indices.is_empty() && *selected > 0 {
                    *selected -= 1;
                }
            }
            AddState::EnteringArgs { current_arg, .. } => {
                *current_arg = current_arg.saturating_sub(1);
            }
        }
    }

    pub fn handle_down(&mut self) {
        match self {
            AddState::Selecting {
                indices, selected, ..
            } => {
                if !indices.is_empty() && *selected + 1 < indices.len() {
                    *selected += 1;
                }
            }
            AddState::EnteringArgs {
                args, current_arg, ..
            } => {
                if *current_arg + 1 < args.len() {
                    *current_arg += 1;
                }
            }
        }
    }

    pub fn handle_enter(&mut self) -> Result<Option<GateOperation>, String> {
        match self {
            AddState::Selecting {
                indices, selected, ..
            } => {
                let &idx = indices
                    .get(*selected)
                    .ok_or_else(|| "No gate selected".to_owned())?;
                let entry = &ALL_ENTRIES[idx];
                let gate_type = entry.gate_type;
                let args = gate_signature(gate_type);
                *self = AddState::EnteringArgs {
                    gate_type,
                    values: vec![String::new(); args.len()],
                    current_arg: 0,
                    args,
                };
                Ok(None)
            }
            AddState::EnteringArgs {
                gate_type,
                args,
                current_arg,
                values,
            } => {
                if let Some(missing_index) = values.iter().position(|value| value.trim().is_empty())
                {
                    *current_arg = missing_index;
                    return Err(format!("Enter {}", args[missing_index].prompt));
                }

                let parsed_values = parse_arg_values(args, values, current_arg)?;
                let operation = try_build_operation(*gate_type, &parsed_values)?;
                *self = AddState::new();
                Ok(Some(operation))
            }
        }
    }

    pub fn handle_escape(&mut self) {
        if matches!(self, AddState::EnteringArgs { .. }) {
            *self = AddState::new();
        } else {
            *self = AddState::new();
        }
    }
}

fn parse_arg_values(
    args: &[ArgPrompt],
    values: &[String],
    current_arg: &mut usize,
) -> Result<Vec<f64>, String> {
    let mut parsed_values = Vec::with_capacity(args.len());

    for (index, (arg, value)) in args.iter().zip(values).enumerate() {
        match parse_arg_value(arg.kind, value) {
            Ok(parsed_value) => parsed_values.push(parsed_value),
            Err(error) => {
                *current_arg = index;
                return Err(error);
            }
        }
    }

    Ok(parsed_values)
}

fn parse_arg_value(kind: ArgKind, input: &str) -> Result<f64, String> {
    let trimmed = input.trim();
    match kind {
        ArgKind::Qubit | ArgKind::Bit => {
            let value: usize = trimmed
                .parse()
                .map_err(|_| format!("Invalid integer: '{trimmed}'"))?;
            Ok(value as f64)
        }
        ArgKind::Angle => {
            if let Some(rest) = trimmed.strip_prefix("pi") {
                if rest.is_empty() {
                    return Ok(std::f64::consts::PI);
                }
                if let Some(den_str) = rest.strip_prefix('/') {
                    let den: f64 = den_str
                        .parse()
                        .map_err(|_| format!("Invalid denominator: '{den_str}'"))?;
                    if den == 0.0_f64 {
                        return Err("Division by zero".to_owned());
                    }
                    return Ok(std::f64::consts::PI / den);
                }
                return Err(format!("Invalid pi expression: '{trimmed}'"));
            }
            trimmed
                .parse::<f64>()
                .map_err(|_| format!("Invalid number: '{trimmed}'"))
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn try_build_operation(gate_type: GateType, values: &[f64]) -> Result<GateOperation, String> {
    match gate_type {
        GateType::ID => Ok(GateOperation::id(values[0] as usize)),
        GateType::H => Ok(GateOperation::h(values[0] as usize)),
        GateType::X => Ok(GateOperation::x(values[0] as usize)),
        GateType::Y => Ok(GateOperation::y(values[0] as usize)),
        GateType::Z => Ok(GateOperation::z(values[0] as usize)),
        GateType::P => GateOperation::try_p(values[0], values[1] as usize)
            .map_err(|e: GateOperationError| e.to_string()),
        GateType::RX => GateOperation::try_rx(values[0], values[1] as usize)
            .map_err(|e: GateOperationError| e.to_string()),
        GateType::RY => GateOperation::try_ry(values[0], values[1] as usize)
            .map_err(|e: GateOperationError| e.to_string()),
        GateType::RZ => GateOperation::try_rz(values[0], values[1] as usize)
            .map_err(|e: GateOperationError| e.to_string()),
        GateType::S => Ok(GateOperation::s(values[0] as usize)),
        GateType::SDG => Ok(GateOperation::sdg(values[0] as usize)),
        GateType::SX => Ok(GateOperation::sx(values[0] as usize)),
        GateType::SY => Ok(GateOperation::sy(values[0] as usize)),
        GateType::T => Ok(GateOperation::t(values[0] as usize)),
        GateType::TDG => Ok(GateOperation::tdg(values[0] as usize)),
        GateType::U => GateOperation::try_u(values[0], values[1], values[2], values[3] as usize)
            .map_err(|e: GateOperationError| e.to_string()),
        GateType::Measure => Ok(GateOperation::measure(
            values[0] as usize,
            values[1] as usize,
        )),
        GateType::Swap => GateOperation::try_swap(values[0] as usize, values[1] as usize)
            .map_err(|e: GateOperationError| e.to_string()),
        GateType::CH => GateOperation::try_ch(values[0] as usize, values[1] as usize)
            .map_err(|e: GateOperationError| e.to_string()),
        GateType::CX => GateOperation::try_cx(values[0] as usize, values[1] as usize)
            .map_err(|e: GateOperationError| e.to_string()),
        GateType::CY => GateOperation::try_cy(values[0] as usize, values[1] as usize)
            .map_err(|e: GateOperationError| e.to_string()),
        GateType::CZ => GateOperation::try_cz(values[0] as usize, values[1] as usize)
            .map_err(|e: GateOperationError| e.to_string()),
        GateType::CP => GateOperation::try_cp(values[0], values[1] as usize, values[2] as usize)
            .map_err(|e: GateOperationError| e.to_string()),
        GateType::CSwap => {
            GateOperation::try_c_swap(values[0] as usize, values[1] as usize, values[2] as usize)
                .map_err(|e: GateOperationError| e.to_string())
        }
        GateType::CCX => {
            GateOperation::try_ccx(values[0] as usize, values[1] as usize, values[2] as usize)
                .map_err(|e: GateOperationError| e.to_string())
        }
        GateType::CCZ => {
            GateOperation::try_ccz(values[0] as usize, values[1] as usize, values[2] as usize)
                .map_err(|e: GateOperationError| e.to_string())
        }
    }
}

pub const FORMATS: &[(&str, &str, &str)] = &[
    ("JSON", ".json", "Human-readable text"),
    ("XML", ".xml", "Human-readable text"),
    ("CBOR", ".cbor", "Compact binary"),
    ("MessagePack", ".msgpack", "Compact binary"),
];

pub fn format_extension(format_name: &str) -> &'static str {
    FORMATS
        .iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case(format_name))
        .map_or(".dat", |(_, ext, _)| ext)
}

pub fn detect_format(path: &str) -> Option<&'static str> {
    let lower = path.to_lowercase();
    if lower.ends_with(".json") {
        Some("json")
    } else if lower.ends_with(".xml") {
        Some("xml")
    } else if lower.ends_with(".cbor") {
        Some("cbor")
    } else if lower.ends_with(".msgpack") || lower.ends_with(".mpk") {
        Some("msgpack")
    } else {
        None
    }
}

pub fn export_circuit(circuit: &Circuit, format_name: &str) -> Result<Vec<u8>, String> {
    match format_name {
        "json" => qctidy_converter::serialize_json(circuit, true, 2).map_err(|e| e.to_string()),
        "xml" => qctidy_converter::serialize_xml(circuit, true, 2).map_err(|e| e.to_string()),
        "cbor" => qctidy_converter::serialize_cbor(circuit).map_err(|e| e.to_string()),
        "msgpack" => qctidy_converter::serialize_msgpack(circuit).map_err(|e| e.to_string()),
        _ => Err(format!("Unknown format: {format_name}")),
    }
}

pub fn import_circuit(data: &[u8], format_name: &str) -> Result<Circuit, String> {
    match format_name {
        "json" => qctidy_converter::parse_json(data).map_err(|e| e.to_string()),
        "xml" => qctidy_converter::parse_xml(data).map_err(|e| e.to_string()),
        "cbor" => qctidy_converter::parse_cbor(data).map_err(|e| e.to_string()),
        "msgpack" => qctidy_converter::parse_msgpack(data).map_err(|e| e.to_string()),
        _ => Err(format!("Unknown format: {format_name}")),
    }
}

pub fn format_gate(gate: &GateOperation) -> String {
    match *gate {
        GateOperation::ID { qubit } => format!("Id(q{qubit})"),
        GateOperation::H { qubit } => format!("H(q{qubit})"),
        GateOperation::X { qubit } => format!("X(q{qubit})"),
        GateOperation::Y { qubit } => format!("Y(q{qubit})"),
        GateOperation::Z { qubit } => format!("Z(q{qubit})"),
        GateOperation::P { theta, qubit } => format!("P({theta})(q{qubit})"),
        GateOperation::RX { theta, qubit } => format!("RX({theta})(q{qubit})"),
        GateOperation::RY { theta, qubit } => format!("RY({theta})(q{qubit})"),
        GateOperation::RZ { phi, qubit } => format!("RZ({phi})(q{qubit})"),
        GateOperation::S { qubit } => format!("S(q{qubit})"),
        GateOperation::SDG { qubit } => format!("S†(q{qubit})"),
        GateOperation::SX { qubit } => format!("√X(q{qubit})"),
        GateOperation::SY { qubit } => format!("√Y(q{qubit})"),
        GateOperation::T { qubit } => format!("T(q{qubit})"),
        GateOperation::TDG { qubit } => format!("T†(q{qubit})"),
        GateOperation::U {
            theta,
            phi,
            lambda,
            qubit,
        } => format!("U({theta},{phi},{lambda})(q{qubit})"),
        GateOperation::Measure { qubit, bit } => format!("M(q{qubit}→c{bit})"),
        GateOperation::Swap { qubit1, qubit2 } => format!("Swap(q{qubit1},q{qubit2})"),
        GateOperation::CH { control, target } => format!("CH(c{control},t{target})"),
        GateOperation::CX { control, target } => format!("CX(c{control},t{target})"),
        GateOperation::CY { control, target } => format!("CY(c{control},t{target})"),
        GateOperation::CZ { qubit1, qubit2 } => format!("CZ(q{qubit1},q{qubit2})"),
        GateOperation::CP {
            theta,
            qubit1,
            qubit2,
        } => format!("CP({theta})(q{qubit1},q{qubit2})"),
        GateOperation::CSwap {
            control,
            target1,
            target2,
        } => format!("CSwap(c{control},t{target1},t{target2})"),
        GateOperation::CCX {
            control1,
            control2,
            target,
        } => format!("CCX(c{control1},c{control2},t{target})"),
        GateOperation::CCZ {
            qubit1,
            qubit2,
            qubit3,
        } => format!("CCZ(q{qubit1},q{qubit2},q{qubit3})"),
    }
}
