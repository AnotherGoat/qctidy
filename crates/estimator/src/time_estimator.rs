use std::collections::HashSet;
use qsimplify::{Graph, GateType, Position};
use crate::models::BackendProfile;

/// Devuelve el costo abstracto (peso heurístico) de una compuerta según el perfil del hardware
fn get_gate_cost(gate: GateType, profile: &BackendProfile) -> f64 {
    use GateType::*;
    match gate {
        // Phase / RZ gates
        P | RZ | ID => profile.phase_cost,
        // Sqrt gates
        SX | SY => profile.sqrt_cost,
        // Single qubit gates
        X | Y | Z | RX | RY | H | S | SDG | T | TDG | U => profile.single_qubit_cost,
        // Measure
        Measure => profile.measure_cost,
        // Swap
        Swap => profile.swap_cost,
        // Two qubit gates
        CX | CY | CZ | CH | CP => profile.two_qubit_cost,
        // Three qubit gates
        CCX | CCZ | CSwap => profile.three_qubit_cost,
    }
}

/// Estima el tiempo total de ejecución de un circuito cuántico utilizando 
/// un algoritmo de Ruta Crítica (Critical Path) en un DAG simulado.
pub fn estimate_execution_time(graph: &Graph, shots: usize, profile: &BackendProfile) -> f64 {
    if graph.is_empty() {
        return (shots as f64) * profile.repetition_delay_cost;
    }

    let num_qubits = graph.height();
    // Rastrea el tiempo acumulado (reloj) para cada qubit
    let mut qubit_times = vec![0.0; num_qubits];
    let mut processed_nodes: HashSet<Position> = HashSet::new();

    // Iteramos ordenados por columnas para respetar el flujo del tiempo (topológico)
    for node in graph.iter_nodes_ordered_by_column() {
        let pos = node.position();
        if processed_nodes.contains(&pos) {
            continue;
        }

        let gate_type = node.r#type();
        let mut involved_qubits = Vec::new();
        let mut queue = vec![pos];

        // BFS para agrupar todas las partes de una compuerta multi-qubit (ej. control y target)
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

        // La compuerta inicia cuando TODOS los qubits involucrados están listos (sincronización)
        let mut start_time = 0.0;
        for &q in &involved_qubits {
            if qubit_times[q] > start_time {
                start_time = qubit_times[q];
            }
        }

        let end_time = start_time + gate_cost;

        // Actualizar el reloj de todos los qubits involucrados con el tiempo de finalización
        for &q in &involved_qubits {
            qubit_times[q] = end_time;
        }
    }

    // El camino crítico es el tiempo máximo alcanzado por cualquier qubit
    let critical_path_time = qubit_times.into_iter().fold(0.0, f64::max);
    
    // Calcular el tiempo heurístico final sumando el retraso de repetición
    (shots as f64) * (critical_path_time + profile.repetition_delay_cost)
}

#[cfg(test)]
mod tests {
    use super::*;
    use qsimplify::GraphBuilder;

    #[test]
    fn test_get_gate_cost() {
        let profile = BackendProfile::default();
        assert_eq!(get_gate_cost(GateType::H, &profile), 1.0); // single
        assert_eq!(get_gate_cost(GateType::CX, &profile), 15.0); // two qubit
        assert_eq!(get_gate_cost(GateType::CCX, &profile), 80.0); // three qubit
    }

    #[test]
    fn test_estimate_execution_time_parallelism() {
        let profile = BackendProfile::default();
        let mut builder = GraphBuilder::new(3);
        // Columna 0: Dos H en paralelo (qubits 0 y 1)
        builder.push_h(0);
        builder.push_h(1);
        // Columna 1: Un CNOT que sincroniza los qubits 0 y 1
        builder.push_cx(0, 1).unwrap();
        
        let graph = builder.build();
        let shots = 10;
        
        let time = estimate_execution_time(&graph, shots, &profile);
        
        // H = 1.0, CX = 15.0. 
        // Qubit 0: H (1.0), Qubit 1: H (1.0) -> Start CX at 1.0 -> End CX at 16.0
        // Critical path = 16.0
        // Total = 10 * (16.0 + delay(100.0)) = 1160.0
        assert_eq!(time, 1160.0);
    }
}
