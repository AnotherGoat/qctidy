use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use function_name::named;

use crate::{
    CanonicalizationRule, GateType, Graph, NodeView, Position, RuleGroup, RuleMetadata,
    RuleRegistry, domain::math,
};

pub(crate) fn register(registry: &mut RuleRegistry) {
    registry.register_all(vec![
        Arc::new(compact_rows()),
        Arc::new(compact_columns()),
        Arc::new(normalize_angles()),
        Arc::new(normalize_bits()),
    ]);
}

#[named]
pub(crate) fn compact_rows() -> CanonicalizationRule {
    CanonicalizationRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts rows by removing gaps between qubit indices.",
            RuleGroup::Canonicalization,
            0,
        ),
        HashSet::new(),
        1,
        1,
        Box::new(|graph: &Graph| -> Vec<HashSet<Position>> {
            let rows: HashSet<usize> = graph
                .iter_nodes_ordered_by_row()
                .map(|node| node.position().row())
                .collect();

            if rows.is_empty() {
                return Vec::new();
            }

            let max_row = *rows.iter().max().expect("There should be at least 1 row");
            let has_gap = (0..=max_row).any(|row| !rows.contains(&row));

            if !has_gap {
                return Vec::new();
            }

            vec![graph.iter_nodes().map(|node| node.position()).collect()]
        }),
        Box::new(|graph: &mut Graph| {
            let mut rows: Vec<_> = graph
                .iter_nodes_ordered_by_row()
                .map(|node| node.position().row())
                .collect();

            if rows.is_empty() {
                return false;
            }

            rows.sort_unstable();
            rows.dedup();

            let mapping: HashMap<usize, usize> = rows
                .into_iter()
                .enumerate()
                .map(|(new, old)| (old, new))
                .collect();

            let needs_compaction = mapping.iter().any(|(old, new)| old != new);
            if !needs_compaction {
                return false;
            }

            let nodes: Vec<_> = graph.iter_nodes_ordered_by_row().collect();

            for node in nodes {
                let position = node.position();
                let new_row = mapping[&position.row()];

                if new_row != position.row() {
                    let new_position = Position::new(new_row, position.column());
                    graph
                        .move_node(position, new_position)
                        .expect("Node at start position should exist");
                }
            }

            true
        }),
    )
}

#[named]
pub(crate) fn compact_columns() -> CanonicalizationRule {
    CanonicalizationRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts columns by removing gaps between time steps.",
            RuleGroup::Canonicalization,
            0,
        ),
        HashSet::new(),
        1,
        1,
        Box::new(|graph: &Graph| -> Vec<HashSet<Position>> {
            let columns: HashSet<usize> = graph
                .iter_nodes_ordered_by_column()
                .map(|node| node.position().column())
                .collect();

            if columns.is_empty() {
                return Vec::new();
            }

            let max_column = *columns.iter().max().unwrap();
            let has_gap = (0..=max_column).any(|column| !columns.contains(&column));

            if !has_gap {
                return Vec::new();
            }

            vec![graph.iter_nodes().map(|node| node.position()).collect()]
        }),
        Box::new(|graph: &mut Graph| {
            let mut columns: Vec<_> = graph
                .iter_nodes_ordered_by_column()
                .map(|node| node.position().column())
                .collect();

            if columns.is_empty() {
                return false;
            }

            columns.sort_unstable();
            columns.dedup();

            let mapping: HashMap<usize, usize> = columns
                .into_iter()
                .enumerate()
                .map(|(new, old)| (old, new))
                .collect();

            let needs_compaction = mapping.iter().any(|(old, new)| old != new);
            if !needs_compaction {
                return false;
            }

            graph.remap_positions(|position| {
                Position::new(position.row(), mapping[&position.column()])
            });
            true
        }),
    )
}

#[named]
pub(crate) fn normalize_angles() -> CanonicalizationRule {
    use GateType::*;

    CanonicalizationRule::new(
        RuleMetadata::new(
            function_name!(),
            "Normalizes all gate angles to the [0, 4pi) range.",
            RuleGroup::Canonicalization,
            0,
        ),
        vec![P, RX, RY, RZ, CP].into_iter().collect(),
        1,
        1,
        Box::new(|graph: &Graph| -> Vec<HashSet<Position>> {
            graph
                .iter_nodes_ordered_by_row()
                .filter(|node| {
                    node.angle()
                        .is_some_and(|angle| !(0.0_f64..math::FULL_CYCLE).contains(&angle))
                })
                .map(|node| HashSet::from([node.position()]))
                .collect()
        }),
        Box::new(|graph: &mut Graph| {
            let nodes: Vec<NodeView> = graph
                .iter_nodes_ordered_by_row()
                .filter(|node| node.angle().is_some())
                .collect();

            if nodes.is_empty() {
                return false;
            }

            let mut changed = false;

            for node in nodes {
                let gate = node.r#type();

                if !(gate.is_phase() || gate.is_rotation()) {
                    continue;
                }

                let angle = node.angle().expect("The node should have an angle");
                let position = node.position();

                if let Ok(normalized) = math::normalize_angle(angle, math::FULL_CYCLE)
                    && !math::are_floats_equal(angle, normalized)
                {
                    graph.replace_node(gate, position, Some(normalized), None, None, node.bit());
                    changed = true;
                }
            }

            changed
        }),
    )
}

#[named]
pub(crate) fn normalize_bits() -> CanonicalizationRule {
    CanonicalizationRule::new(
        RuleMetadata::new(
            function_name!(),
            "Normalizes measurement bit indices to a compact range starting from 0.",
            RuleGroup::Canonicalization,
            0,
        ),
        vec![GateType::Measure].into_iter().collect(),
        1,
        1,
        Box::new(|graph: &Graph| -> Vec<HashSet<Position>> {
            let measurements: Vec<NodeView> = graph
                .iter_nodes_ordered_by_row()
                .filter(|node| node.r#type() == GateType::Measure)
                .collect();

            if measurements.is_empty() {
                return Vec::new();
            }

            let mut unique_bits: Vec<usize> =
                measurements.iter().filter_map(NodeView::bit).collect();
            unique_bits.sort_unstable();
            unique_bits.dedup();

            let first_gap_bit = unique_bits
                .windows(2)
                .find_map(|pair| (pair[1] != pair[0] + 1).then(|| pair[1]));

            first_gap_bit.map_or_else(Vec::new, |threshold| {
                measurements
                    .iter()
                    .filter(|node| node.bit().is_some_and(|bit| bit >= threshold))
                    .map(|node| HashSet::from([node.position()]))
                    .collect()
            })
        }),
        Box::new(|graph: &mut Graph| {
            let measurements: Vec<_> = graph
                .iter_nodes_ordered_by_row()
                .filter(|node| node.r#type() == GateType::Measure)
                .collect();

            if measurements.is_empty() {
                return false;
            }

            let mut unique_bits: Vec<_> = measurements.iter().filter_map(NodeView::bit).collect();
            unique_bits.sort_unstable();
            unique_bits.dedup();

            let mapping: HashMap<usize, usize> = unique_bits
                .into_iter()
                .enumerate()
                .map(|(new, old)| (old, new))
                .collect();

            let needs_remap = mapping.iter().any(|(old, new)| old != new);
            if !needs_remap {
                return false;
            }

            for node in measurements {
                let position = node.position();
                let old_bit = node.bit().expect("Measure node should have a bit");
                let new_bit = mapping[&old_bit];
                graph.replace_node(GateType::Measure, position, None, None, None, Some(new_bit));
            }

            true
        }),
    )
}
