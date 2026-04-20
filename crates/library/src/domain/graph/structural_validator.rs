use std::collections::HashSet;

use crate::{Graph, GraphError};

/// Validate the structure of a `Graph`.
///
/// This only is part of the validation process. To fully validate a `Graph`, use `Graph::validate` instead.
pub(super) fn validate(graph: &Graph) -> Result<(), GraphError> {
    check_dangling_nodes(graph)?;
    check_edge_mirroring(graph)?;
    check_duplicate_edges(graph)?;
    Ok(())
}

fn check_dangling_nodes(graph: &Graph) -> Result<(), GraphError> {
    for (start, edges) in &graph.edges_out {
        if !graph.nodes.contains_key(start) {
            return Err(GraphError::DanglingStartNode { from: *start });
        }

        for edge in edges {
            if !graph.nodes.contains_key(&edge.other) {
                return Err(GraphError::DanglingEndNode {
                    edge_type: edge.edge_type,
                    from: *start,
                    to: edge.other,
                });
            }
        }
    }

    Ok(())
}

fn check_edge_mirroring(graph: &Graph) -> Result<(), GraphError> {
    for (start, edges) in &graph.edges_out {
        for edge in edges {
            let is_symmetric = graph
                .edges_in
                .get(&edge.other)
                .map(|other_edges| {
                    other_edges.iter().any(|other_edge| {
                        other_edge.other == *start && other_edge.edge_type == edge.edge_type
                    })
                })
                .unwrap_or(false);

            if !is_symmetric {
                return Err(GraphError::MissingReverseEdge {
                    from: *start,
                    to: edge.other,
                    edge_type: edge.edge_type,
                });
            }
        }
    }

    Ok(())
}

fn check_duplicate_edges(graph: &Graph) -> Result<(), GraphError> {
    for (start, edges) in &graph.edges_out {
        let mut seen = HashSet::new();

        for edge in edges {
            if !seen.insert((edge.edge_type, edge.other)) {
                return Err(GraphError::DuplicateEdge {
                    from: *start,
                    to: edge.other,
                    edge_type: edge.edge_type,
                });
            }
        }
    }

    Ok(())
}
