use std::collections::HashMap;

use crate::{GateType, Graph, NodeView, Position, domain::math};

/// Normalize a `Graph` by performing various transformations that reduce the contents of the graph without altering its meaning and results.
///
/// Identity nodes are removed.
/// Empty rows and columns are removed.
/// Angles are normalized by ensuring that they are between 0 and 4pi.
/// Bits are normalized by ensuring that they are between 0 and `bits` - 1.
pub fn normalize(graph: &mut Graph) {
    remove_identity_nodes(graph);
    compact_rows(graph);
    compact_columns(graph);
    normalize_angles(graph);
    normalize_bits(graph);
}

fn remove_identity_nodes(graph: &mut Graph) {
    let positions: Vec<_> = graph
        .iter_nodes_ordered_by_row()
        .filter(|node| node.r#type() == GateType::ID)
        .map(|node| node.position())
        .collect();

    for position in positions {
        graph.remove_node(position);
    }
}

fn compact_rows(graph: &mut Graph) {
    let mut rows: Vec<_> = graph
        .iter_nodes_ordered_by_row()
        .map(|node| node.position().row())
        .collect();

    rows.sort_unstable();
    rows.dedup();

    let mapping: HashMap<usize, usize> = rows
        .into_iter()
        .enumerate()
        .map(|(new, old)| (old, new))
        .collect();

    remap_rows(graph, &mapping);
}

fn remap_rows(graph: &mut Graph, mapping: &HashMap<usize, usize>) {
    let nodes: Vec<_> = graph.iter_nodes_ordered_by_row().collect();

    for node in nodes {
        let position = node.position();
        let new_row = mapping[&position.row()];

        if new_row != position.row() {
            let new_position = Position::new(new_row, position.column());
            graph.move_node(position, new_position).unwrap();
        }
    }
}

fn compact_columns(graph: &mut Graph) {
    let mut columns: Vec<_> = graph
        .iter_nodes_ordered_by_column()
        .map(|node| node.position().column())
        .collect();

    columns.sort_unstable();
    columns.dedup();

    let mapping: HashMap<usize, usize> = columns
        .into_iter()
        .enumerate()
        .map(|(new, old)| (old, new))
        .collect();

    graph.remap_positions(|position| Position::new(position.row(), mapping[&position.column()]));
}

fn normalize_angles(graph: &mut Graph) {
    let nodes: Vec<NodeView> = graph.iter_nodes_ordered_by_row().collect();

    for node in nodes {
        normalize_angle(graph, node);
    }
}

fn normalize_angle(graph: &mut Graph, node: NodeView) {
    let gate = node.r#type();
    let position = node.position();

    let angle = match node.angle() {
        Some(found) => found,
        None => return,
    };

    let normalized = match gate {
        r#type if r#type.is_phase() || r#type.is_rotation() => {
            match math::normalize_angle(angle, math::FULL_CYCLE) {
                Ok(normalized) => normalized,
                Err(_) => return,
            }
        }
        _ => return,
    };

    graph.replace_node(gate, position, Some(normalized), node.bit());
}

fn normalize_bits(graph: &mut Graph) {
    let measurements: Vec<_> = graph
        .iter_nodes_ordered_by_row()
        .filter(|node| node.r#type() == GateType::Measure)
        .collect();

    let mut unique_bits: Vec<_> = measurements.iter().filter_map(|node| node.bit()).collect();

    unique_bits.sort_unstable();
    unique_bits.dedup();

    let mapping: HashMap<usize, usize> = unique_bits
        .into_iter()
        .enumerate()
        .map(|(new, old)| (old, new))
        .collect();

    for node in measurements {
        let position = node.position();
        let old_bit = node.bit().unwrap();
        let new_bit = mapping[&old_bit];

        graph.replace_node(GateType::Measure, position, None, Some(new_bit));
    }
}
