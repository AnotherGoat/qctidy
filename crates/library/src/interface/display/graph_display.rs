use std::fmt;

use faer::{Mat, complex::Complex64};

use crate::{
    Graph,
    application::simplifier::matrix_calculator,
    interface::display::angle_formatter::{self, PiFormat},
};

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Nodes:")?;

        let nodes = self.iter_nodes_ordered_by_column().collect::<Vec<_>>();

        if nodes.is_empty() {
            writeln!(f, "(empty)")?;
        }

        for node in nodes {
            writeln!(f, "{node}")?;
        }

        writeln!(f, "\nEdges:")?;

        let edges = self.iter_edges_by_column().collect::<Vec<_>>();

        if edges.is_empty() {
            write!(f, "(empty)")?;
        }

        let mut current = None;

        for (index, edge) in edges.iter().enumerate() {
            let position = edge.start().position();

            match current {
                Some(previous) if previous != position => {
                    current = Some(position);
                    writeln!(f)?;
                }
                None => {
                    current = Some(position);
                }
                _ => {}
            }

            write!(f, "{edge}")?;

            if index != edges.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Graph {
    /// Get an alternative string representation of the graph, as a 2D grid.
    #[must_use]
    pub fn display_grid(&self) -> String {
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
                    label.push_str(&angle_formatter::format(angle, PiFormat::Lowercase));
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
}
