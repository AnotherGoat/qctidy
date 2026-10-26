use std::collections::{HashMap, HashSet};

use qctidy::number::{truncate_usize_to_f64, truncate_usize_to_isize};
use qctidy::{EdgeType, GateType, Graph};
use qctidy_ports::{AnalysisMetrics, DeltaAnalysisMetrics, DetailedAnalysisMetrics};

/// Calculate metrics for the provided graph.
#[must_use]
pub fn calculate_metrics(graph: &Graph) -> AnalysisMetrics {
    use GateType::*;

    let qubit_count = graph.qubit_count();
    let x_count = count_gates(graph, X);
    let y_count = count_gates(graph, Y);
    let z_count = count_gates(graph, Z);
    let measured_qubit_count = count_measured_qubits(graph);

    AnalysisMetrics {
        qubit_count,
        bit_count: graph.bit_count(),
        time_step_count: graph.time_step_count(),
        x_count,
        y_count,
        z_count,
        pauli_count: x_count + y_count + z_count,
        hadamard_count: count_gates(graph, H),
        rotation_count: count_rotation_gates(graph),
        phase_count: count_gates(graph, P),
        square_root_count: count_square_root_gates(graph),
        t_count: count_gates(graph, T) + count_gates(graph, TDG),
        swap_count: count_gates(graph, Swap),
        cx_count: count_gates(graph, CX),
        ccx_count: count_gates(graph, CCX),
        operation_count: graph.operation_count(),
        gate_count: graph.gate_count(),
        measurement_count: graph.measurement_count(),
        single_gate_count: count_single_gates(graph),
        controlled_gate_count: count_controlled_gates(graph),
        measured_qubit_count,
        unmeasured_qubit_count: qubit_count.saturating_sub(measured_qubit_count),
        gate_types_count: count_gate_types(graph),
    }
}

/// Compare metrics between the provided graphs.
#[must_use]
pub fn compare_metrics(old: &Graph, new: &Graph) -> DeltaAnalysisMetrics {
    let old_metrics = calculate_metrics(old);
    let new_metrics = calculate_metrics(new);

    DeltaAnalysisMetrics {
        qubit_count: delta(old_metrics.qubit_count, new_metrics.qubit_count),
        bit_count: delta(old_metrics.bit_count, new_metrics.bit_count),
        time_step_count: delta(old_metrics.time_step_count, new_metrics.time_step_count),
        x_count: delta(old_metrics.x_count, new_metrics.x_count),
        y_count: delta(old_metrics.y_count, new_metrics.y_count),
        z_count: delta(old_metrics.z_count, new_metrics.z_count),
        pauli_count: delta(old_metrics.pauli_count, new_metrics.pauli_count),
        hadamard_count: delta(old_metrics.hadamard_count, new_metrics.hadamard_count),
        rotation_count: delta(old_metrics.rotation_count, new_metrics.rotation_count),
        phase_count: delta(old_metrics.phase_count, new_metrics.phase_count),
        square_root_count: delta(old_metrics.square_root_count, new_metrics.square_root_count),
        t_count: delta(old_metrics.t_count, new_metrics.t_count),
        swap_count: delta(old_metrics.swap_count, new_metrics.swap_count),
        cx_count: delta(old_metrics.cx_count, new_metrics.cx_count),
        ccx_count: delta(old_metrics.ccx_count, new_metrics.ccx_count),
        operation_count: delta(old_metrics.operation_count, new_metrics.operation_count),
        gate_count: delta(old_metrics.gate_count, new_metrics.gate_count),
        measurement_count: delta(old_metrics.measurement_count, new_metrics.measurement_count),
        single_gate_count: delta(old_metrics.single_gate_count, new_metrics.single_gate_count),
        controlled_gate_count: delta(
            old_metrics.controlled_gate_count,
            new_metrics.controlled_gate_count,
        ),
        measured_qubit_count: delta(
            old_metrics.measured_qubit_count,
            new_metrics.measured_qubit_count,
        ),
        unmeasured_qubit_count: delta(
            old_metrics.unmeasured_qubit_count,
            new_metrics.unmeasured_qubit_count,
        ),
        gate_types_count: delta(old_metrics.gate_types_count, new_metrics.gate_types_count),
    }
}

const fn delta(old: usize, new: usize) -> isize {
    truncate_usize_to_isize(new) - truncate_usize_to_isize(old)
}

/// Calculate detailed metrics for the provided graph.
#[must_use]
pub fn calculate_detailed_metrics(graph: &Graph) -> DetailedAnalysisMetrics {
    use GateType::*;

    let operation_count = graph.operation_count();
    let gate_count = graph.gate_count();
    let measurement_count = graph.measurement_count();
    let x_count = count_gates(graph, X);
    let y_count = count_gates(graph, Y);
    let z_count = count_gates(graph, Z);
    let pauli_count = x_count + y_count + z_count;
    let hadamard_count = count_gates(graph, H);
    let single_gate_count = count_single_gates(graph);
    let cx_count = count_gates(graph, CX);
    let ccx_count = count_gates(graph, CCX);

    let single_gate_ratio = if gate_count == 0 {
        0.0
    } else {
        truncate_usize_to_f64(single_gate_count) / truncate_usize_to_f64(gate_count)
    };

    let measured_qubit_count = count_measured_qubits(graph);
    let measure_ratio = if graph.is_empty() || graph.qubit_count() == 0 {
        0.0
    } else {
        truncate_usize_to_f64(measured_qubit_count) / truncate_usize_to_f64(graph.qubit_count())
    };

    DetailedAnalysisMetrics {
        qubit_count: graph.qubit_count(),
        time_step_count: graph.time_step_count(),
        max_density: calculate_max_density(graph),
        average_density: calculate_average_density(graph),
        x_count,
        y_count,
        z_count,
        pauli_count,
        hadamard_count,
        initial_superposition_ratio: calculate_superposition_ratio(graph),
        single_gate_count,
        other_single_gate_count: single_gate_count
            .saturating_sub(pauli_count)
            .saturating_sub(hadamard_count),
        single_controlled_gate_count: count_single_controlled_gates(graph),
        swap_count: count_gates(graph, Swap),
        cx_count,
        cx_qubit_ratio: calculate_cx_qubit_ratio(graph),
        average_cx: if graph.is_empty() || graph.qubit_count() == 0 {
            0.0
        } else {
            truncate_usize_to_f64(cx_count) / truncate_usize_to_f64(graph.qubit_count())
        },
        max_cx_count: count_max_cx(graph),
        ccx_count,
        ccx_qubit_ratio: calculate_ccx_qubit_ratio(graph),
        average_ccx: if graph.is_empty() || graph.qubit_count() == 0 {
            0.0
        } else {
            truncate_usize_to_f64(ccx_count) / truncate_usize_to_f64(graph.qubit_count())
        },
        max_ccx_count: count_max_ccx(graph),
        operation_count,
        gate_count,
        measurement_count,
        controlled_gate_count: count_controlled_gates(graph),
        single_gate_ratio,
        measured_qubit_count,
        measure_ratio,
        unmeasured_qubit_ratio: 1.0 - measure_ratio,
    }
}

#[expect(clippy::integer_division)]
fn count_gates(graph: &Graph, gate_type: GateType) -> usize {
    let size = gate_type.arity();
    if size == 0 {
        return 0;
    }

    let nodes = graph
        .iter_nodes()
        .filter(|node| node.r#type() == gate_type)
        .count();
    nodes / size
}

fn calculate_superposition_ratio(graph: &Graph) -> f64 {
    if graph.is_empty() || graph.qubit_count() == 0 {
        return 0.0;
    }

    let hadamard_count = graph
        .iter_nodes()
        .filter(|node| node.position().time_step() == 0 && node.r#type() == GateType::H)
        .count();

    truncate_usize_to_f64(hadamard_count) / truncate_usize_to_f64(graph.qubit_count())
}

fn count_single_gates(graph: &Graph) -> usize {
    graph
        .iter_nodes()
        .filter(|node| node.r#type().arity() == 1 && node.r#type() != GateType::Measure)
        .count()
}

#[expect(clippy::integer_division)]
fn count_single_controlled_gates(graph: &Graph) -> usize {
    graph
        .iter_nodes()
        .filter(|node| node.r#type().is_single_controlled())
        .count()
        / 2 // Divided by 2 since they take 2 qubits
}

fn count_controlled_gates(graph: &Graph) -> usize {
    let mut counts: HashMap<GateType, usize> = HashMap::new();
    for node in graph.iter_nodes().filter(|n| n.r#type().is_controlled()) {
        *counts.entry(node.r#type()).or_insert(0) += 1;
    }

    let mut total = 0;
    for (gate_type, occurrences) in counts {
        total += occurrences.checked_div(gate_type.arity()).unwrap_or(0);
    }
    total
}

fn count_rotation_gates(graph: &Graph) -> usize {
    graph
        .iter_nodes()
        .filter(|node| node.r#type().is_rotation())
        .count()
}

fn count_square_root_gates(graph: &Graph) -> usize {
    graph
        .iter_nodes()
        .filter(|node| node.r#type().is_square_root())
        .count()
}

fn count_measured_qubits(graph: &Graph) -> usize {
    if graph.is_empty() {
        return 0;
    }
    graph
        .iter_nodes()
        .filter(|node| node.r#type() == GateType::Measure)
        .map(|node| node.position().qubit())
        .collect::<HashSet<_>>()
        .len()
}

fn count_gate_types(graph: &Graph) -> usize {
    graph
        .iter_nodes()
        .filter(|node| node.r#type() != GateType::ID)
        .map(|node| node.r#type())
        .collect::<HashSet<_>>()
        .len()
}

fn calculate_time_step_density(graph: &Graph, time_step: usize) -> usize {
    let mut count = 0;
    let mut counted_gates: HashMap<GateType, usize> = HashMap::new();

    for node in graph
        .iter_nodes()
        .filter(|node| node.position().time_step() == time_step)
    {
        *counted_gates.entry(node.r#type()).or_insert(0) += 1;
    }

    for (gate_type, occurences) in counted_gates {
        count += occurences.checked_div(gate_type.arity()).unwrap_or(0);
    }
    count
}

fn calculate_max_density(graph: &Graph) -> usize {
    (0..graph.time_step_count())
        .map(|time_step| calculate_time_step_density(graph, time_step))
        .max()
        .unwrap_or(0)
}

fn calculate_average_density(graph: &Graph) -> f64 {
    if graph.is_empty() || graph.time_step_count() == 0 {
        return 0.0;
    }
    let total: usize = (0..graph.time_step_count())
        .map(|time_step| calculate_time_step_density(graph, time_step))
        .sum();

    truncate_usize_to_f64(total) / truncate_usize_to_f64(graph.time_step_count())
}

fn calculate_cx_qubit_ratio(graph: &Graph) -> f64 {
    if graph.is_empty() || graph.qubit_count() == 0 {
        return 0.0;
    }
    let qubits: HashSet<_> = graph
        .iter_nodes()
        .filter(|node| node.r#type() == GateType::CX)
        .map(|node| node.position().qubit())
        .collect();

    truncate_usize_to_f64(qubits.len()) / truncate_usize_to_f64(graph.qubit_count())
}

fn count_max_cx(graph: &Graph) -> usize {
    if graph.is_empty() {
        return 0;
    }

    let mut counts = HashMap::new();
    for edge in graph.iter_edges() {
        if edge.r#type() == EdgeType::Targets && edge.start().r#type() == GateType::CX {
            // Target edge from control to target. The control is start().
            *counts.entry(edge.start().position().qubit()).or_insert(0) += 1;
        }
    }
    counts.into_values().max().unwrap_or(0)
}

fn calculate_ccx_qubit_ratio(graph: &Graph) -> f64 {
    if graph.is_empty() || graph.qubit_count() == 0 {
        return 0.0;
    }
    let qubits: HashSet<_> = graph
        .iter_nodes()
        .filter(|node| node.r#type() == GateType::CCX)
        .map(|node| node.position().qubit())
        .collect();

    truncate_usize_to_f64(qubits.len()) / truncate_usize_to_f64(graph.qubit_count())
}

fn count_max_ccx(graph: &Graph) -> usize {
    if graph.is_empty() {
        return 0;
    }

    let mut counts = HashMap::new();
    for edge in graph.iter_edges() {
        if edge.r#type() == EdgeType::Targets && edge.start().r#type() == GateType::CCX {
            *counts.entry(edge.start().position().qubit()).or_insert(0) += 1;
        }
    }
    counts.into_values().max().unwrap_or(0)
}
