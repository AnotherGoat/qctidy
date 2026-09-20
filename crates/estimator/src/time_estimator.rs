use crate::models::BackendProfile;
use qctidy::{GateType, Graph, Position};
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
        let position = node.position();
        if processed_nodes.contains(&position) {
            continue;
        }

        let gate_type = node.r#type();
        let mut involved_qubits = Vec::new();
        let mut queue = vec![position];

        while let Some(current_position) = queue.pop() {
            if !processed_nodes.insert(current_position) {
                continue;
            }

            involved_qubits.push(current_position.row());

            queue.extend(
                graph
                    .iter_semantic_neighbors_from(current_position)
                    .filter(|neighbor| !processed_nodes.contains(neighbor)),
            );
        }

        let gate_cost = get_gate_cost(gate_type, profile);

        let start_time = involved_qubits
            .iter()
            .map(|&qubit| qubit_times[qubit])
            .fold(0.0, f64::max);

        let end_time = start_time + gate_cost;

        for &qubit in &involved_qubits {
            qubit_times[qubit] = end_time;
        }
    }

    let critical_path_time = qubit_times.into_iter().fold(0.0, f64::max);

    (shots as f64) * (critical_path_time + profile.repetition_delay_cost)
}
