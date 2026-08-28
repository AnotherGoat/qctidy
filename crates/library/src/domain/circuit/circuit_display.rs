use std::fmt;
use std::fmt::Write;

use newgen::New;

use crate::AngleFormat;
use crate::GATE_METADATAS;
use crate::GateOperation;
use crate::GateType;
use crate::PiFormat;
use crate::display::angle_formatter;

use super::Circuit;

const CONTROL_NODE: &str = "■";
const SWAP_NODE: &str = "×";

#[derive(Debug, New)]
#[new(const)]
struct PackedColumn<'a> {
    used_qubits: Vec<bool>,
    operations: Vec<&'a GateOperation>,
    spans: Vec<GateSpan>,
}

#[derive(Debug, New)]
#[new(const)]
struct GateSpan {
    min: usize,
    max: usize,
}

#[derive(Debug, New)]
#[new(const)]
struct DisplayCells {
    labels: Vec<Vec<Option<String>>>,
    kinds: Vec<Vec<CellKind>>,
    column_widths: Vec<usize>,
}

#[derive(Debug, New)]
#[new(const)]
struct AppliedOperation {
    qubits: Vec<usize>,
    labels: Vec<String>,
    kinds: Vec<CellKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellKind {
    Gate,
    Control,
    Target,
    Swap,
    HorizontalWire,
    VerticalWire,
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display(PiFormat::Fancy))
    }
}

impl Circuit {
    #[must_use]
    pub fn display(&self, pi_format: PiFormat) -> String {
        let operations = self.operations();
        let rows = self.qubit_count();

        if operations.is_empty() || rows == 0 {
            return String::new();
        }

        let packed = pack_operations(operations, rows);
        let DisplayCells {
            labels,
            kinds,
            column_widths,
        } = build_cells(&packed, rows, pi_format);
        let label_padding = format!("{}: ", rows - 1).len();
        let mut output = String::new();

        let has_top_border = packed
            .iter()
            .enumerate()
            .any(|(col, _)| matches!(kinds[col][0], CellKind::Gate | CellKind::Target));

        for display_row in 0..calculate_display_rows(rows) {
            let is_first_border = display_row == 0 && !has_top_border;

            if display_row > 0 && !is_first_border {
                output.push('\n');
            }

            if is_first_border {
                continue;
            }

            let qubit = displayed_to_qubit(display_row);

            if is_content_row(display_row) {
                render_qubit_prefix(qubit, &mut output);
                render_content_row(
                    qubit,
                    packed.len(),
                    &column_widths,
                    &labels,
                    &kinds,
                    &mut output,
                );
            } else {
                add_padding(label_padding, &mut output);
                render_border_row(qubit, &column_widths, &kinds, &packed, rows, &mut output);

                if has_qubit_above(qubit) && has_qubit_below(qubit, rows) {
                    trim_current_line_end(&mut output);
                }
            }
        }

        output
    }
}

fn pack_operations(operations: &[GateOperation], rows: usize) -> Vec<PackedColumn<'_>> {
    let mut packed: Vec<PackedColumn> = Vec::new();

    for operation in operations {
        let qubits: Vec<usize> = operation.qubits();
        let mut found = false;

        for column in &mut packed {
            let overlaps = qubits.iter().any(|&qubit| column.used_qubits[qubit]);

            if !overlaps {
                if qubits.len() > 1 {
                    let min = qubits
                        .iter()
                        .copied()
                        .min()
                        .expect("There should be at least 2 qubits");
                    let max = qubits
                        .iter()
                        .copied()
                        .max()
                        .expect("There should be at least 2 qubits");

                    for qubit in min..=max {
                        column.used_qubits[qubit] = true;
                    }

                    column.spans.push(GateSpan::new(min, max));
                } else {
                    for &qubit in &qubits {
                        column.used_qubits[qubit] = true;
                    }
                }

                column.operations.push(operation);
                found = true;
                break;
            }
        }

        if !found {
            let mut used_qubits = vec![false; rows];

            if qubits.len() > 1 {
                let min = qubits
                    .iter()
                    .copied()
                    .min()
                    .expect("There should be at least 2 qubits");
                let max = qubits
                    .iter()
                    .copied()
                    .max()
                    .expect("There should be at least 2 qubits");

                for qubit in min..=max {
                    used_qubits[qubit] = true;
                }
            } else {
                for &qubit in &qubits {
                    used_qubits[qubit] = true;
                }
            }

            let mut spans = Vec::new();

            if qubits.len() > 1 {
                let min = qubits
                    .iter()
                    .copied()
                    .min()
                    .expect("There should be at least 2 qubits");
                let max = qubits
                    .iter()
                    .copied()
                    .max()
                    .expect("There should be at least 2 qubits");
                spans.push(GateSpan::new(min, max));
            }

            packed.push(PackedColumn {
                used_qubits,
                operations: vec![operation],
                spans,
            });
        }
    }

    packed
}

fn build_cells(packed: &[PackedColumn<'_>], rows: usize, pi_format: PiFormat) -> DisplayCells {
    let columns = packed.len();
    let mut labels: Vec<Vec<Option<String>>> = vec![vec![None; rows]; columns];
    let mut kinds: Vec<Vec<CellKind>> = vec![vec![CellKind::HorizontalWire; rows]; columns];
    let mut column_widths = vec![0; columns];

    for (column, packed_column) in packed.iter().enumerate() {
        for operation in &packed_column.operations {
            apply_operation_to_cell(
                operation,
                pi_format,
                column,
                &mut labels,
                &mut kinds,
                &mut column_widths,
            );
        }
    }

    DisplayCells::new(labels, kinds, column_widths)
}

fn apply_operation_to_cell(
    operation: &GateOperation,
    pi_format: PiFormat,
    column: usize,
    labels: &mut [Vec<Option<String>>],
    kinds: &mut [Vec<CellKind>],
    column_widths: &mut [usize],
) {
    use GateOperation::*;

    let AppliedOperation {
        qubits,
        labels: cell_labels,
        kinds: cell_kinds,
    } = match *operation {
        ID { qubit }
        | H { qubit }
        | X { qubit }
        | Y { qubit }
        | Z { qubit }
        | S { qubit }
        | SDG { qubit }
        | SX { qubit }
        | SY { qubit }
        | T { qubit }
        | TDG { qubit } => {
            let name = gate_display_name(operation.r#type());

            AppliedOperation::new(vec![qubit], vec![name], vec![CellKind::Gate])
        }
        P { theta, qubit } | RX { theta, qubit } | RY { theta, qubit } => {
            let formatted_theta = angle_formatter::format(theta, AngleFormat::Algebra, pi_format);
            let name = gate_display_name(operation.r#type());

            AppliedOperation::new(
                vec![qubit],
                vec![format!("{name}({formatted_theta})")],
                vec![CellKind::Gate],
            )
        }
        RZ { phi, qubit } => {
            let formatted_phi = angle_formatter::format(phi, AngleFormat::Algebra, pi_format);
            let name = gate_display_name(operation.r#type());

            AppliedOperation::new(
                vec![qubit],
                vec![format!("{name}({formatted_phi})")],
                vec![CellKind::Gate],
            )
        }
        U {
            theta,
            phi,
            lambda,
            qubit,
        } => {
            let formatted_theta = angle_formatter::format(theta, AngleFormat::Algebra, pi_format);
            let formatted_phi = angle_formatter::format(phi, AngleFormat::Algebra, pi_format);
            let formatted_lambda = angle_formatter::format(lambda, AngleFormat::Algebra, pi_format);

            AppliedOperation::new(
                vec![qubit],
                vec![format!(
                    "U({formatted_theta}, {formatted_phi}, {formatted_lambda})"
                )],
                vec![CellKind::Gate],
            )
        }
        Measure { qubit, bit } => {
            AppliedOperation::new(vec![qubit], vec![format!("M({bit})")], vec![CellKind::Gate])
        }
        Swap { qubit1, qubit2 } => AppliedOperation::new(
            vec![qubit1, qubit2],
            vec![SWAP_NODE.into(), SWAP_NODE.into()],
            vec![CellKind::Swap, CellKind::Swap],
        ),
        CH { control, target } => AppliedOperation::new(
            vec![control, target],
            vec![CONTROL_NODE.into(), "H".into()],
            vec![CellKind::Control, CellKind::Target],
        ),
        CX { control, target } => AppliedOperation::new(
            vec![control, target],
            vec![CONTROL_NODE.into(), "X".into()],
            vec![CellKind::Control, CellKind::Target],
        ),
        CY { control, target } => AppliedOperation::new(
            vec![control, target],
            vec![CONTROL_NODE.into(), "Y".into()],
            vec![CellKind::Control, CellKind::Target],
        ),
        CZ { qubit1, qubit2 } => AppliedOperation::new(
            vec![qubit1, qubit2],
            vec![CONTROL_NODE.into(), CONTROL_NODE.into()],
            vec![CellKind::Control, CellKind::Control],
        ),
        CP {
            theta,
            qubit1,
            qubit2,
        } => {
            let formatted_theta = angle_formatter::format(theta, AngleFormat::Algebra, pi_format);
            let label = format!("{CONTROL_NODE}({formatted_theta})");

            AppliedOperation::new(
                vec![qubit1, qubit2],
                vec![label.clone(), label],
                vec![CellKind::Control, CellKind::Control],
            )
        }
        CSwap {
            control,
            target1,
            target2,
        } => AppliedOperation::new(
            vec![control, target1, target2],
            vec![CONTROL_NODE.into(), SWAP_NODE.into(), SWAP_NODE.into()],
            vec![CellKind::Control, CellKind::Swap, CellKind::Swap],
        ),
        CCX {
            control1,
            control2,
            target,
        } => AppliedOperation::new(
            vec![control1, control2, target],
            vec![CONTROL_NODE.into(), CONTROL_NODE.into(), "X".into()],
            vec![CellKind::Control, CellKind::Control, CellKind::Target],
        ),
        CCZ {
            qubit1,
            qubit2,
            qubit3,
        } => AppliedOperation::new(
            vec![qubit1, qubit2, qubit3],
            vec![
                CONTROL_NODE.into(),
                CONTROL_NODE.into(),
                CONTROL_NODE.into(),
            ],
            vec![CellKind::Control, CellKind::Control, CellKind::Control],
        ),
    };

    for (index, &qubit) in qubits.iter().enumerate() {
        labels[column][qubit] = Some(cell_labels[index].clone());
        kinds[column][qubit] = cell_kinds[index];

        let char_count = cell_labels[index].chars().count();
        let cell_width = if matches!(cell_kinds[index], CellKind::Gate | CellKind::Target) {
            char_count + 4
        } else {
            char_count
        };

        column_widths[column] = column_widths[column].max(cell_width);
    }

    if qubits.len() > 1 {
        let min = qubits
            .iter()
            .copied()
            .min()
            .expect("There should be at least 2 qubits");
        let max = qubits
            .iter()
            .copied()
            .max()
            .expect("There should be at least 2 qubits");

        for qubit in min..=max {
            if labels[column][qubit].is_none() {
                kinds[column][qubit] = CellKind::VerticalWire;
            }
        }
    }
}

fn gate_display_name(gate_type: GateType) -> String {
    GATE_METADATAS[gate_type as u8 as usize]
        .display_name()
        .to_string()
}

const fn calculate_display_rows(qubit_rows: usize) -> usize {
    2 * qubit_rows + 1
}

const fn is_content_row(display_row: usize) -> bool {
    display_row % 2 == 1
}

#[expect(clippy::integer_division)]
const fn displayed_to_qubit(display_row: usize) -> usize {
    display_row / 2
}

fn render_qubit_prefix(qubit: usize, output: &mut String) {
    write!(output, "{qubit}: ").expect("String should always be writable");
}

#[expect(clippy::integer_division)]
fn render_content_row(
    qubit: usize,
    columns: usize,
    column_widths: &[usize],
    labels: &[Vec<Option<String>>],
    kinds: &[Vec<CellKind>],
    output: &mut String,
) {
    for column in 0..columns {
        let width = column_widths[column];

        match kinds[column][qubit] {
            CellKind::Gate | CellKind::Target => {
                let label = labels[column][qubit]
                    .as_ref()
                    .expect("Kinds and labels should be in sync");
                let content_width = width - 2;
                let label_length = label.chars().count();
                let right_padding = content_width - 1 - label_length;

                output.push('┤');
                output.push(' ');
                output.push_str(label);

                for _ in 0..right_padding {
                    output.push(' ');
                }

                output.push('├');
            }
            CellKind::Control | CellKind::Swap => {
                let label = labels[column][qubit]
                    .as_ref()
                    .expect("Kinds and labels should be in sync");
                let character_count = label.chars().count();
                let left_padding = (width - character_count) / 2;
                let right_padding = width - character_count - left_padding;

                for _ in 0..left_padding {
                    output.push('─');
                }

                output.push_str(label);

                for _ in 0..right_padding {
                    output.push('─');
                }
            }
            CellKind::VerticalWire => {
                let left_padding = width / 2;
                let right_padding = width - left_padding - 1;

                for _ in 0..left_padding {
                    output.push('─');
                }

                output.push('│');

                for _ in 0..right_padding {
                    output.push('─');
                }
            }
            CellKind::HorizontalWire => {
                for _ in 0..width {
                    output.push('─');
                }
            }
        }
    }
}

fn add_padding(label_padding: usize, output: &mut String) {
    for _ in 0..label_padding {
        output.push(' ');
    }
}

#[expect(clippy::integer_division)]
fn render_border_row(
    qubit: usize,
    column_widths: &[usize],
    kinds: &[Vec<CellKind>],
    packed: &[PackedColumn<'_>],
    rows: usize,
    output: &mut String,
) {
    for column in 0..packed.len() {
        let width = column_widths[column];
        let above_kind = if has_qubit_above(qubit) {
            kinds[column][qubit - 1]
        } else {
            CellKind::HorizontalWire
        };
        let below_kind = if has_qubit_below(qubit, rows) {
            kinds[column][qubit]
        } else {
            CellKind::HorizontalWire
        };

        let connected = packed[column]
            .spans
            .iter()
            .any(|&GateSpan { min, max }| has_qubit_above(qubit) && min < qubit && qubit <= max);

        let above_is_boxed = matches!(above_kind, CellKind::Gate | CellKind::Target);
        let below_is_boxed = matches!(below_kind, CellKind::Gate | CellKind::Target);

        if above_is_boxed || below_is_boxed {
            let (start, end) = match (above_is_boxed, below_is_boxed) {
                (true, true) => ('├', '┤'),
                (true, false) => ('└', '┘'),
                (false, true) => ('┌', '┐'),
                (false, false) => ('─', '─'),
            };

            let interior = width.saturating_sub(2);
            let center = interior / 2;

            output.push(start);

            for index in 0..interior {
                if connected && index == center {
                    let ch = match (above_is_boxed, below_is_boxed) {
                        (true, true) => '┼',
                        (true, false) => '┬',
                        (false, true) => '┴',
                        _ => '─',
                    };
                    output.push(ch);
                } else {
                    output.push('─');
                }
            }

            output.push(end);
        } else if connected {
            for i in 0..width {
                if i == width / 2 {
                    output.push('│');
                } else {
                    output.push(' ');
                }
            }
        } else {
            for _ in 0..width {
                output.push(' ');
            }
        }
    }
}

fn trim_current_line_end(output: &mut String) {
    while output.ends_with(' ') {
        output.pop();
    }
}

const fn has_qubit_above(qubit: usize) -> bool {
    qubit > 0
}

const fn has_qubit_below(qubit: usize, qubit_count: usize) -> bool {
    qubit < qubit_count
}
