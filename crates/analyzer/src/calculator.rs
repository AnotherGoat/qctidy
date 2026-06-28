use std::collections::{HashMap, HashSet};

use qsimplify::{EdgeType, GateType, Graph};
use crate::metrics::{DeltaMetrics, DetailedMetrics, Metrics};

/// Calculate metrics for the provided graph.
pub fn calculate_metrics(graph: &Graph) -> Metrics {
    let qubit_count = graph.height();
    let x_count = count_gates(graph, GateType::X);
    let y_count = count_gates(graph, GateType::Y);
    let z_count = count_gates(graph, GateType::Z);
    let measure_count = count_measured_qubits(graph);

    Metrics {
        qubit_count,
        depth: graph.width(),
        x_count,
        y_count,
        z_count,
        pauli_count: x_count + y_count + z_count,
        hadamard_count: count_gates(graph, GateType::H),
        rotation_count: count_rotation_gates(graph),
        square_root_count: count_square_root_gates(graph),
        measure_count,
        swap_count: count_gates(graph, GateType::Swap),
        cx_count: count_gates(graph, GateType::CX),
        gate_count: count_total_gates(graph),
        single_gate_count: count_single_gates(graph),
        controlled_gate_count: count_controlled_gates(graph),
        ancilla_qubit_count: qubit_count.saturating_sub(measure_count),
        gate_types_count: count_gate_types(graph),
    }
}

/// Compare metrics between the provided graphs.
pub fn compare_metrics(old: &Graph, new: &Graph) -> DeltaMetrics {
    let old_metrics = calculate_metrics(old);
    let new_metrics = calculate_metrics(new);

    DeltaMetrics {
        qubit_count: delta(old_metrics.qubit_count, new_metrics.qubit_count),
        depth: delta(old_metrics.depth, new_metrics.depth),
        x_count: delta(old_metrics.x_count, new_metrics.x_count),
        y_count: delta(old_metrics.y_count, new_metrics.y_count),
        z_count: delta(old_metrics.z_count, new_metrics.z_count),
        pauli_count: delta(old_metrics.pauli_count, new_metrics.pauli_count),
        hadamard_count: delta(old_metrics.hadamard_count, new_metrics.hadamard_count),
        rotation_count: delta(old_metrics.rotation_count, new_metrics.rotation_count),
        square_root_count: delta(old_metrics.square_root_count, new_metrics.square_root_count),
        measure_count: delta(old_metrics.measure_count, new_metrics.measure_count),
        swap_count: delta(old_metrics.swap_count, new_metrics.swap_count),
        cx_count: delta(old_metrics.cx_count, new_metrics.cx_count),
        gate_count: delta(old_metrics.gate_count, new_metrics.gate_count),
        single_gate_count: delta(old_metrics.single_gate_count, new_metrics.single_gate_count),
        controlled_gate_count: delta(old_metrics.controlled_gate_count, new_metrics.controlled_gate_count),
        ancilla_qubit_count: delta(old_metrics.ancilla_qubit_count, new_metrics.ancilla_qubit_count),
        gate_types_count: delta(old_metrics.gate_types_count, new_metrics.gate_types_count),
    }
}

fn delta(old: usize, new: usize) -> Option<isize> {
    let diff = new as isize - old as isize;
    if diff == 0 {
        None
    } else {
        Some(diff)
    }
}

/// Calculate detailed metrics for the provided graph.
pub fn calculate_detailed_metrics(graph: &Graph) -> DetailedMetrics {
    let gate_count = count_total_gates(graph);
    let x_count = count_gates(graph, GateType::X);
    let y_count = count_gates(graph, GateType::Y);
    let z_count = count_gates(graph, GateType::Z);
    let pauli_count = x_count + y_count + z_count;
    let hadamard_count = count_gates(graph, GateType::H);
    let single_gate_count = count_single_gates(graph);
    let cnot_count = count_gates(graph, GateType::CX);
    let toffoli_count = count_gates(graph, GateType::CCX);
    
    let single_qubit_percent = if gate_count == 0 {
        0.0
    } else {
        single_gate_count as f64 / gate_count as f64
    };
    
    let measure_count = count_measured_qubits(graph);
    let measure_percent = if graph.is_empty() || graph.height() == 0 {
        0.0
    } else {
        measure_count as f64 / graph.height() as f64
    };

    DetailedMetrics {
        width: graph.height(),
        depth: graph.width(),
        max_density: calculate_max_density(graph),
        average_density: calculate_average_density(graph),
        x_count,
        y_count,
        z_count,
        pauli_count,
        hadamard_count,
        initial_superposition_percent: calculate_superposition_percent(graph),
        single_qubit_count: single_gate_count,
        other_single_qubit_count: single_gate_count.saturating_sub(pauli_count).saturating_sub(hadamard_count),
        single_controlled_qubit_count: count_single_controlled_gates(graph),
        swap_count: count_gates(graph, GateType::Swap),
        cnot_count,
        cnot_qubit_percent: calculate_cnot_qubit_percent(graph),
        average_cnot: if graph.is_empty() || graph.height() == 0 { 0.0 } else { cnot_count as f64 / graph.height() as f64 },
        max_cnot: count_max_cnot(graph),
        toffoli_count,
        toffoli_qubit_percent: calculate_toffoli_qubit_percent(graph),
        average_toffoli: if graph.is_empty() || graph.height() == 0 { 0.0 } else { toffoli_count as f64 / graph.height() as f64 },
        max_toffoli: count_max_toffoli(graph),
        gate_count,
        controlled_gate_count: count_controlled_gates(graph),
        single_qubit_percent,
        measure_count,
        measure_percent,
        ancilla_percent: 1.0 - measure_percent,
    }
}

fn count_total_gates(graph: &Graph) -> usize {
    if graph.is_empty() {
        return 0;
    }
    
    let mut counts = HashMap::new();
    for node in graph.iter_nodes() {
        *counts.entry(node.r#type()).or_insert(0) += 1;
    }
    
    let mut total = 0;
    for (gate_type, occurrences) in counts {
        let size = gate_type.qubit_count();
        if size > 0 {
            total += occurrences / size;
        }
    }
    total
}

fn count_gates(graph: &Graph, gate_type: GateType) -> usize {
    let size = gate_type.qubit_count();
    if size == 0 {
        return 0;
    }

    let nodes = graph.iter_nodes().filter(|node| node.r#type() == gate_type).count();
    nodes / size
}

fn calculate_superposition_percent(graph: &Graph) -> f64 {
    if graph.is_empty() || graph.height() == 0 {
        return 0.0;
    }

    let hadamard_count = graph.iter_nodes()
        .filter(|node| node.position().column() == 0 && node.r#type() == GateType::H)
        .count();

    hadamard_count as f64 / graph.height() as f64
}

fn count_single_gates(graph: &Graph) -> usize {
    graph.iter_nodes()
        .filter(|node| node.r#type().qubit_count() == 1)
        .count()
}

fn count_single_controlled_gates(graph: &Graph) -> usize {
    graph.iter_nodes()
        .filter(|node| node.r#type().is_single_controlled())
        .count() / 2 // Divided by 2 since they take 2 qubits
}

fn count_controlled_gates(graph: &Graph) -> usize {
    let mut counts = HashMap::new();
    for node in graph.iter_nodes().filter(|n| n.r#type().is_controlled()) {
        *counts.entry(node.r#type()).or_insert(0) += 1;
    }
    
    let mut total = 0;
    for (gate_type, occurrences) in counts {
        let size = gate_type.qubit_count();
        if size > 0 {
            total += occurrences / size;
        }
    }
    total
}

fn count_rotation_gates(graph: &Graph) -> usize {
    graph.iter_nodes()
        .filter(|node| node.r#type().is_rotation())
        .count()
}

fn count_square_root_gates(graph: &Graph) -> usize {
    graph.iter_nodes()
        .filter(|node| node.r#type().is_square_root())
        .count()
}

fn count_measured_qubits(graph: &Graph) -> usize {
    if graph.is_empty() {
        return 0;
    }
    graph.iter_nodes()
        .filter(|node| node.r#type() == GateType::Measure)
        .map(|node| node.position().row())
        .collect::<HashSet<_>>()
        .len()
}

fn count_gate_types(graph: &Graph) -> usize {
    graph.iter_nodes()
        .filter(|node| node.r#type() != GateType::ID)
        .map(|node| node.r#type())
        .collect::<HashSet<_>>()
        .len()
}

fn calculate_column_density(graph: &Graph, column: usize) -> usize {
    let mut count = 0;
    let mut counted_gates = HashMap::new();
    
    for node in graph.iter_nodes().filter(|node| node.position().column() == column) {
        *counted_gates.entry(node.r#type()).or_insert(0) += 1;
    }
    
    for (gate_type, occurences) in counted_gates {
        let size = gate_type.qubit_count();
        if size > 0 {
            count += occurences / size;
        }
    }
    count
}

fn calculate_max_density(graph: &Graph) -> usize {
    (0..graph.width())
        .map(|col| calculate_column_density(graph, col))
        .max()
        .unwrap_or(0)
}

fn calculate_average_density(graph: &Graph) -> f64 {
    if graph.is_empty() || graph.width() == 0 {
        return 0.0;
    }
    let total: usize = (0..graph.width())
        .map(|col| calculate_column_density(graph, col))
        .sum();
    
    total as f64 / graph.width() as f64
}

fn calculate_cnot_qubit_percent(graph: &Graph) -> f64 {
    if graph.is_empty() || graph.height() == 0 {
        return 0.0;
    }
    let rows: HashSet<_> = graph.iter_nodes()
        .filter(|node| node.r#type() == GateType::CX)
        .map(|node| node.position().row())
        .collect();
    
    rows.len() as f64 / graph.height() as f64
}

fn count_max_cnot(graph: &Graph) -> usize {
    if graph.is_empty() {
        return 0;
    }
    
    let mut counts = HashMap::new();
    for edge in graph.iter_edges() {
        if edge.r#type() == EdgeType::Targets && edge.start().r#type() == GateType::CX {
            // Target edge from control to target. The control is start().
            *counts.entry(edge.start().position().row()).or_insert(0) += 1;
        }
    }
    counts.into_values().max().unwrap_or(0)
}

fn calculate_toffoli_qubit_percent(graph: &Graph) -> f64 {
    if graph.is_empty() || graph.height() == 0 {
        return 0.0;
    }
    let rows: HashSet<_> = graph.iter_nodes()
        .filter(|node| node.r#type() == GateType::CCX)
        .map(|node| node.position().row())
        .collect();
    
    rows.len() as f64 / graph.height() as f64
}

fn count_max_toffoli(graph: &Graph) -> usize {
    if graph.is_empty() {
        return 0;
    }
    
    let mut counts = HashMap::new();
    for edge in graph.iter_edges() {
        if edge.r#type() == EdgeType::Targets && edge.start().r#type() == GateType::CCX {
            *counts.entry(edge.start().position().row()).or_insert(0) += 1;
        }
    }
    counts.into_values().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use qsimplify::GraphBuilder;

    #[test]
    fn test_calculate_metrics() {
        let mut builder = GraphBuilder::new(2);
        builder.push_h(0);
        builder.push_cx(0, 1).unwrap();
        let graph = builder.build();

        let metrics = calculate_metrics(&graph);

        assert_eq!(metrics.qubit_count, 2);
        assert_eq!(metrics.gate_count, 2);
        assert_eq!(metrics.hadamard_count, 1);
        assert_eq!(metrics.cx_count, 1);
        assert_eq!(metrics.x_count, 0);
        assert_eq!(metrics.single_gate_count, 1);
        assert_eq!(metrics.controlled_gate_count, 1);
    }
    
    #[test]
    fn test_empty_graph_does_not_panic() {
        let graph = Graph::new(0);
        let detailed = calculate_detailed_metrics(&graph);
        assert_eq!(detailed.average_density, 0.0);
        assert_eq!(detailed.single_qubit_percent, 0.0);
        assert_eq!(detailed.cnot_qubit_percent, 0.0);
    }
}
