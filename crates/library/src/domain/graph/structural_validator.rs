use std::collections::HashSet;

use crate::{EdgeType, Graph, GraphError, Position};

/// Validate the structure of a `Graph`.
///
/// This only is part of the validation process. To fully validate a `Graph`, use `Graph::validate` instead.
pub(super) fn validate(graph: &Graph) -> Result<(), GraphError> {
    check_dangling_nodes(graph)?;
    check_edge_mirroring(graph)?;
    check_duplicate_edges(graph)?;
    check_row_order_edges(graph)?;
    Ok(())
}

/// Check that every node's `Right` edges match its row neighbors.
///
/// A `Right` edge must connect a node to the next node in its row (and the previous
/// one, since `Right` is bidirectional). Any other `Right` edge is inconsistent.
fn check_row_order_edges(graph: &Graph) -> Result<(), GraphError> {
    for &position in graph.nodes.keys() {
        let expected_next: Vec<Position> = graph.next_in_row(position).into_iter().collect();
        let expected_previous: Vec<Position> =
            graph.previous_in_row(position).into_iter().collect();

        if right_neighbors(graph.edges_out.get(&position)) != expected_next
            || right_neighbors(graph.edges_in.get(&position)) != expected_previous
        {
            return Err(GraphError::InvalidRowOrder { position });
        }
    }

    Ok(())
}

fn right_neighbors(edges: Option<&Vec<super::EdgeData>>) -> Vec<Position> {
    edges
        .into_iter()
        .flatten()
        .filter(|edge| edge.edge_type == EdgeType::Right)
        .map(|edge| edge.other)
        .collect()
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
            let is_symmetric = graph.edges_in.get(&edge.other).is_some_and(|other_edges| {
                other_edges.iter().any(|other_edge| {
                    other_edge.other == *start && other_edge.edge_type == edge.edge_type
                })
            });

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
