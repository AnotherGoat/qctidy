use crate::models::BackendProfile;
use qsimplify::{GateType, Graph, Position};
use std::collections::HashSet;

pub(crate) fn get_gate_cost(gate: GateType, profile: &BackendProfile) -> f64 {
    use GateType::*;
    match gate {
        P | RZ | ID => profile.phase_cost,
        SX | SY => profile.sqrt_cost,
        X | Y | Z | RX | RY | H | S | SDG | T | TDG | U => profile.single_qubit_cost,
        Measure => profile.measure_cost,
        Swap => profile.swap_cost,
        CX | CY | CZ | CH | CP => profile.two_qubit_cost,
        CCX | CCZ | CSwap => profile.three_qubit_cost,
    }
}

/// Estimates total quantum circuit execution time using critical-path analysis.
pub fn estimate_execution_time(graph: &Graph, shots: usize, profile: &BackendProfile) -> f64 {
    if graph.is_empty() {
        return (shots as f64) * profile.repetition_delay_cost;
    }

    let num_qubits = graph.height();
    let mut qubit_times = vec![0.0; num_qubits];
    let mut processed_nodes: HashSet<Position> = HashSet::new();

    for node in graph.iter_nodes_ordered_by_column() {
        let pos = node.position();
        if processed_nodes.contains(&pos) {
            continue;
        }

        let gate_type = node.r#type();
        let mut involved_qubits = Vec::new();
        let mut queue = vec![pos];

        while let Some(current_pos) = queue.pop() {
            if processed_nodes.insert(current_pos) {
                involved_qubits.push(current_pos.row());

                for neighbor_pos in graph.iter_semantic_neighbors_from(current_pos) {
                    if !processed_nodes.contains(&neighbor_pos) {
                        queue.push(neighbor_pos);
                    }
                }
            }
        }

        let gate_cost = get_gate_cost(gate_type, profile);

        let mut start_time = 0.0;
        for &q in &involved_qubits {
            if qubit_times[q] > start_time {
                start_time = qubit_times[q];
            }
        }

        let end_time = start_time + gate_cost;

        for &q in &involved_qubits {
            qubit_times[q] = end_time;
        }
    }

    let critical_path_time = qubit_times.into_iter().fold(0.0, f64::max);

    (shots as f64) * (critical_path_time + profile.repetition_delay_cost)
}
