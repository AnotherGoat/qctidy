use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use function_name::named;

use crate::{
    CanonicalizationRule, GateOperation, GateType, Graph, NodeView, Position, RuleCategory,
    RuleMetadata, RuleRegistry, domain::math,
};

pub(crate) fn register(registry: &mut RuleRegistry) {
    registry.register_all(vec![
        Arc::new(compact_qubits()),
        Arc::new(compact_time_steps()),
        Arc::new(normalize_angles()),
        Arc::new(normalize_bits()),
    ]);
}

#[named]
pub(crate) fn compact_qubits() -> CanonicalizationRule {
    CanonicalizationRule::new(
        RuleMetadata::new(
            "C001",
            function_name!(),
            "Qubit indices contain gaps",
            RuleCategory::Canonicalization,
            0,
        ),
        HashSet::new(),
        1,
        1,
        Box::new(|graph: &Graph| -> Vec<HashSet<Position>> {
            let qubits: HashSet<usize> = graph
                .iter_nodes_ordered_by_qubit()
                .map(|node| node.position().qubit())
                .collect();

            if qubits.is_empty() {
                return Vec::new();
            }

            let max_qubit = *qubits
                .iter()
                .max()
                .expect("There should be at least 1 qubit");
            let has_gap = (0..=max_qubit).any(|qubit| !qubits.contains(&qubit));

            if !has_gap {
                return Vec::new();
            }

            vec![graph.iter_nodes().map(|node| node.position()).collect()]
        }),
        Box::new(|graph: &mut Graph| {
            let mut qubits: Vec<_> = graph
                .iter_nodes_ordered_by_qubit()
                .map(|node| node.position().qubit())
                .collect();

            if qubits.is_empty() {
                return false;
            }

            qubits.sort_unstable();
            qubits.dedup();

            let mapping: HashMap<usize, usize> = qubits
                .into_iter()
                .enumerate()
                .map(|(new, old)| (old, new))
                .collect();

            let needs_compaction = mapping.iter().any(|(old, new)| old != new);
            if !needs_compaction {
                return false;
            }

            let nodes: Vec<_> = graph.iter_nodes_ordered_by_qubit().collect();

            for node in nodes {
                let position = node.position();
                let new_qubit = mapping[&position.qubit()];

                if new_qubit != position.qubit() {
                    let new_position = Position::new(new_qubit, position.time_step());
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
pub(crate) fn compact_time_steps() -> CanonicalizationRule {
    CanonicalizationRule::new(
        RuleMetadata::new(
            "C002",
            function_name!(),
            "Time steps contain gaps",
            RuleCategory::Canonicalization,
            0,
        ),
        HashSet::new(),
        1,
        1,
        Box::new(|graph: &Graph| -> Vec<HashSet<Position>> {
            let time_steps: HashSet<usize> = graph
                .iter_nodes_ordered_by_time_step()
                .map(|node| node.position().time_step())
                .collect();

            let Some(&max_time_step) = time_steps.iter().max() else {
                return Vec::new();
            };
            let has_gap = (0..=max_time_step).any(|time_step| !time_steps.contains(&time_step));

            if !has_gap {
                return Vec::new();
            }

            vec![graph.iter_nodes().map(|node| node.position()).collect()]
        }),
        Box::new(|graph: &mut Graph| {
            let mut time_steps: Vec<_> = graph
                .iter_nodes_ordered_by_time_step()
                .map(|node| node.position().time_step())
                .collect();

            if time_steps.is_empty() {
                return false;
            }

            time_steps.sort_unstable();
            time_steps.dedup();

            let mapping: HashMap<usize, usize> = time_steps
                .into_iter()
                .enumerate()
                .map(|(new, old)| (old, new))
                .collect();

            let needs_compaction = mapping.iter().any(|(old, new)| old != new);
            if !needs_compaction {
                return false;
            }

            graph.remap_positions(|position| {
                Position::new(position.qubit(), mapping[&position.time_step()])
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
            "C003",
            function_name!(),
            "Gate angle is outside the [0, 4pi) range",
            RuleCategory::Canonicalization,
            0,
        ),
        vec![P, RX, RY, RZ, CP].into_iter().collect(),
        1,
        1,
        Box::new(|graph: &Graph| -> Vec<HashSet<Position>> {
            graph
                .iter_nodes_ordered_by_qubit()
                .filter(|node| {
                    node.semantic_angle()
                        .is_some_and(|angle| !(0.0..math::FULL_CYCLE).contains(&angle))
                })
                .map(|node| HashSet::from([node.position()]))
                .collect()
        }),
        Box::new(|graph: &mut Graph| {
            let nodes: Vec<NodeView> = graph
                .iter_nodes_ordered_by_qubit()
                .filter(|node| node.semantic_angle().is_some())
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

                let angle = node
                    .semantic_angle()
                    .expect("The node should have an angle");
                let position = node.position();

                let normalized = if math::is_float_close_to_zero(angle) {
                    Ok(0.0)
                } else {
                    math::normalize_angle(angle, math::FULL_CYCLE)
                };

                if let Ok(normalized) = normalized
                    && !math::are_floats_equal(angle, normalized)
                {
                    let (theta, phi, lambda) = if gate == RZ {
                        (node.theta(), Some(normalized), node.lambda())
                    } else {
                        (Some(normalized), node.phi(), node.lambda())
                    };

                    graph.replace_node(gate, position, theta, phi, lambda, node.bit());
                    changed = true;
                }
            }

            changed
        }),
    )
    .with_fix(Box::new(|graph: &Graph, positions: &HashSet<Position>| {
        let mut operations = Vec::with_capacity(positions.len());

        for position in positions {
            let node = graph.get_node(*position)?;
            let angle = node.semantic_angle()?;
            let normalized = if math::is_float_close_to_zero(angle) {
                0.0
            } else {
                math::normalize_angle(angle, math::FULL_CYCLE).ok()?
            };
            let qubit = position.qubit();

            let operation = match node.r#type() {
                P => GateOperation::try_p(normalized, qubit).ok()?,
                RX => GateOperation::try_rx(normalized, qubit).ok()?,
                RY => GateOperation::try_ry(normalized, qubit).ok()?,
                RZ => GateOperation::try_rz(normalized, qubit).ok()?,
                CP => {
                    let other = graph
                        .get_contextual_view(*position)?
                        .works_with()
                        .first()?
                        .position();

                    GateOperation::try_cp(normalized, qubit, other.qubit()).ok()?
                }
                ID | H | X | Y | Z | S | SDG | SX | SY | T | TDG | U | Measure | Swap | CH | CX
                | CY | CZ | CSwap | CCX | CCZ => return None,
            };

            operations.push(operation);
        }

        Some(operations)
    }))
}

#[named]
pub(crate) fn normalize_bits() -> CanonicalizationRule {
    CanonicalizationRule::new(
        RuleMetadata::new(
            "C004",
            function_name!(),
            "Measurement bit indices contain gaps",
            RuleCategory::Canonicalization,
            0,
        ),
        vec![GateType::Measure].into_iter().collect(),
        1,
        1,
        Box::new(|graph: &Graph| -> Vec<HashSet<Position>> {
            let measurements: Vec<NodeView> = graph
                .iter_nodes_ordered_by_qubit()
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
                .iter_nodes_ordered_by_qubit()
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
