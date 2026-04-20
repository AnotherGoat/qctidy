use std::fmt;

use crate::domain::Graph;

impl fmt::Display for Graph {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Nodes:")?;

        let nodes = self.iter_nodes_ordered_by_column().collect::<Vec<_>>();

        if nodes.is_empty() {
            writeln!(formatter, "(empty)")?;
        }

        for node in nodes {
            writeln!(formatter, "{node}")?;
        }

        writeln!(formatter, "\nEdges:")?;

        let edges = self.iter_edges_by_column().collect::<Vec<_>>();

        if edges.is_empty() {
            write!(formatter, "(empty)")?;
        }

        let mut current = None;

        for (index, edge) in edges.iter().enumerate() {
            let position = edge.start().position();

            match current {
                Some(previous) if previous != position => {
                    current = Some(position);
                    writeln!(formatter, "")?;
                }
                None => {
                    current = Some(position);
                }
                _ => {}
            }

            write!(formatter, "{edge}")?;

            if index != edges.len() - 1 {
                writeln!(formatter, "")?;
            }
        }

        Ok(())
    }
}
