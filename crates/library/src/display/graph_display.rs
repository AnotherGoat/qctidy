use std::fmt;
use std::fmt::Write;

use crate::{
    AngleFormat, Graph,
    display::{
        angle_formatter::{self, PiFormat},
        number_formatter,
    },
    simplifier::matrix_calculator,
};

/// Format used to display Dirac bra-ket notation.
#[derive(Debug, Clone, Copy)]
pub enum DiracFormat {
    /// Display bras (rows) as `<10|` and kets (columns) as `|10>`, ASCII-compatible.
    Ascii,
    /// Display bras (rows) as `⟨10∣` and kets (columns) as `∣10⟩`.
    Fancy,
    /// Don't use bra-ket notation, just keep the indices.
    None,
}

enum DiracPart {
    Bra,
    Ket,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_nodes_and_edges(PiFormat::Lowercase))
    }
}

impl Graph {
    /// Display a `Graph` as customizable list of nodes and edges.
    ///
    /// Gives an output similar to `to_string`, but with more control over its format.
    #[must_use]
    pub fn display_nodes_and_edges(&self, pi_format: PiFormat) -> String {
        let mut output = "Nodes:\n".to_owned();

        let nodes = self.iter_nodes_ordered_by_column().collect::<Vec<_>>();

        if nodes.is_empty() {
            output.push_str("(empty)\n");
        }

        for node in nodes {
            writeln!(output, "{}", node.display(pi_format))
                .expect("String should always be writable");
        }

        output.push_str("\nEdges:\n");

        let edges = self.iter_edges_by_column().collect::<Vec<_>>();

        if edges.is_empty() {
            output.push_str("(empty)");
        }

        let mut current = None;

        for (index, edge) in edges.iter().enumerate() {
            let position = edge.start().position();

            match current {
                Some(previous) if previous != position => {
                    current = Some(position);
                    output.push('\n');
                }
                None => {
                    current = Some(position);
                }
                _ => {}
            }

            write!(output, "{}", edge.display(pi_format))
                .expect("String should always be writable");

            if index != edges.len() - 1 {
                output.push('\n');
            }
        }

        output
    }

    /// Get an alternative string representation of the graph, as a 2D grid.
    #[must_use]
    pub fn display_grid(&self, pi_format: PiFormat) -> String {
        if self.is_empty() {
            return "(empty)".to_owned();
        }

        let height = self.height();
        let width = self.width();

        let mut grid = vec![vec![".".to_owned(); width]; height];

        for position in self.iter_positions_ordered_by_row() {
            if let Some(node) = self.get_node(position) {
                let mut label = node.r#type().to_string().to_ascii_uppercase();

                if let Some(angle) = node.angle() {
                    label.push('(');
                    label.push_str(&angle_formatter::format(
                        angle,
                        AngleFormat::Algebra,
                        pi_format,
                    ));
                    label.push(')');
                }

                if let Some(bit) = node.bit() {
                    label.push('(');
                    label.push_str(&bit.to_string());
                    label.push(')');
                }

                grid[position.row()][position.column()] = label;
            }
        }

        let mut column_widths = vec![0; width];

        for column in 0..width {
            let max_length = (0..height)
                .map(|row| grid[row][column].len())
                .max()
                .unwrap_or(0);

            column_widths[column] = max_length;
        }

        let mut rows = Vec::new();

        for (row_index, row) in grid.iter().enumerate() {
            let formatted: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(column, value)| format!("{:<width$}", value, width = column_widths[column]))
                .collect();

            let line = formatted.join("   ");
            let trimmed = line.trim_end();

            rows.push(format!("{row_index}: {trimmed}"));
        }

        rows.join("\n")
    }

    pub fn display_matrix(&self, dirac_format: DiracFormat) -> String {
        let matrix = matrix_calculator::graph_circuit_matrix(self);
        let qubit_count = self.height();
        let size = 1 << qubit_count;

        let grid: Vec<Vec<String>> = (0..size)
            .map(|row| {
                (0..size)
                    .map(|column| number_formatter::format_complex(matrix[(row, column)]))
                    .collect()
            })
            .collect();

        let mut column_widths = vec![0_usize; size];

        for column in 0..size {
            column_widths[column] = (0..size)
                .map(|row| grid[row][column].len())
                .max()
                .unwrap_or(1);
        }

        let row_labels: Vec<String> = (0..size)
            .map(|row| format_dirac(row, qubit_count, DiracPart::Bra, dirac_format))
            .collect();
        let row_label_width = row_labels.iter().map(String::len).max().unwrap_or(1);
        let column_labels: Vec<String> = (0..size)
            .map(|column| format_dirac(column, qubit_count, DiracPart::Ket, dirac_format))
            .collect();

        let mut output = String::new();

        output.push_str(&" ".repeat(row_label_width));
        output.push_str("  ");

        for column in 0..size {
            let width = column_widths[column].max(column_labels[column].len());
            write!(output, "{:>width$}", column_labels[column], width = width)
                .expect("String should always be writable");

            if column != size - 1 {
                output.push_str("  ");
            }
        }
        output.push('\n');

        for row in 0..size {
            write!(
                output,
                "{:>width$}  ",
                row_labels[row],
                width = row_label_width
            )
            .expect("String should always be writable");

            for column in 0..size {
                let width = column_widths[column].max(column_labels[column].len());
                write!(output, "{:>width$}", grid[row][column], width = width)
                    .expect("String should always be writable");

                if column != size - 1 {
                    output.push_str("  ");
                }
            }

            if row != size - 1 {
                output.push('\n');
            }
        }

        output
    }
}

fn format_dirac(index: usize, qubit_count: usize, part: DiracPart, format: DiracFormat) -> String {
    use DiracFormat::*;
    use DiracPart::*;

    let bits: String = (0..qubit_count)
        .rev()
        .map(|bit| if (index >> bit) & 1 == 1 { '1' } else { '0' })
        .collect();

    match (part, format) {
        (Bra, Ascii) => format!("<{bits}|"),
        (Bra, Fancy) => format!("⟨{bits}∣"),
        (Ket, Ascii) => format!("|{bits}>"),
        (Ket, Fancy) => format!("∣{bits}⟩"),
        (_, None) => bits,
    }
}
