pub mod calculator;
pub mod metrics;

use qctidy::Graph;
use qctidy_ports::{
    AnalysisMetrics, AnalysisMode, AnalysisResult, AnalyzerPort, DeltaAnalysisMetrics,
    DeltaAnalysisResult, DeltaDetailedAnalysisMetrics, DetailedAnalysisMetrics,
};

#[derive(Debug, Copy, Clone)]
pub struct AnalyzerAdapter;

impl AnalyzerPort for AnalyzerAdapter {
    fn analyze(&self, graph: &Graph, mode: AnalysisMode) -> AnalysisResult {
        match mode {
            AnalysisMode::Simple => {
                AnalysisResult::Simple(calculator::calculate_metrics(graph).into())
            }
            AnalysisMode::Detailed => AnalysisResult::Detailed(calculate_detailed(graph)),
        }
    }

    fn compare(&self, old: &Graph, new: &Graph, mode: AnalysisMode) -> DeltaAnalysisResult {
        match mode {
            AnalysisMode::Simple => {
                DeltaAnalysisResult::Simple(calculator::compare_metrics(old, new).into())
            }
            AnalysisMode::Detailed => {
                DeltaAnalysisResult::Detailed(compare_detailed_metrics(old, new))
            }
        }
    }
}

fn calculate_detailed(graph: &Graph) -> DetailedAnalysisMetrics {
    calculator::calculate_detailed_metrics(graph).into()
}

fn compare_detailed_metrics(old: &Graph, new: &Graph) -> DeltaDetailedAnalysisMetrics {
    let old_metrics = calculate_detailed(old);
    let new_metrics = calculate_detailed(new);

    DeltaDetailedAnalysisMetrics {
        width: delta_usize(old_metrics.width, new_metrics.width),
        depth: delta_usize(old_metrics.depth, new_metrics.depth),
        max_density: delta_usize(old_metrics.max_density, new_metrics.max_density),
        average_density: delta_f64(old_metrics.average_density, new_metrics.average_density),
        x_count: delta_usize(old_metrics.x_count, new_metrics.x_count),
        y_count: delta_usize(old_metrics.y_count, new_metrics.y_count),
        z_count: delta_usize(old_metrics.z_count, new_metrics.z_count),
        pauli_count: delta_usize(old_metrics.pauli_count, new_metrics.pauli_count),
        hadamard_count: delta_usize(old_metrics.hadamard_count, new_metrics.hadamard_count),
        initial_superposition_percent: delta_f64(
            old_metrics.initial_superposition_percent,
            new_metrics.initial_superposition_percent,
        ),
        other_single_qubit_count: delta_usize(
            old_metrics.other_single_qubit_count,
            new_metrics.other_single_qubit_count,
        ),
        single_qubit_count: delta_usize(
            old_metrics.single_qubit_count,
            new_metrics.single_qubit_count,
        ),
        single_controlled_qubit_count: delta_usize(
            old_metrics.single_controlled_qubit_count,
            new_metrics.single_controlled_qubit_count,
        ),
        swap_count: delta_usize(old_metrics.swap_count, new_metrics.swap_count),
        cnot_count: delta_usize(old_metrics.cnot_count, new_metrics.cnot_count),
        cnot_qubit_percent: delta_f64(
            old_metrics.cnot_qubit_percent,
            new_metrics.cnot_qubit_percent,
        ),
        average_cnot: delta_f64(old_metrics.average_cnot, new_metrics.average_cnot),
        max_cnot: delta_usize(old_metrics.max_cnot, new_metrics.max_cnot),
        toffoli_count: delta_usize(old_metrics.toffoli_count, new_metrics.toffoli_count),
        toffoli_qubit_percent: delta_f64(
            old_metrics.toffoli_qubit_percent,
            new_metrics.toffoli_qubit_percent,
        ),
        average_toffoli: delta_f64(old_metrics.average_toffoli, new_metrics.average_toffoli),
        max_toffoli: delta_usize(old_metrics.max_toffoli, new_metrics.max_toffoli),
        gate_count: delta_usize(old_metrics.gate_count, new_metrics.gate_count),
        controlled_gate_count: delta_usize(
            old_metrics.controlled_gate_count,
            new_metrics.controlled_gate_count,
        ),
        single_qubit_percent: delta_f64(
            old_metrics.single_qubit_percent,
            new_metrics.single_qubit_percent,
        ),
        measure_count: delta_usize(old_metrics.measure_count, new_metrics.measure_count),
        measure_percent: delta_f64(old_metrics.measure_percent, new_metrics.measure_percent),
        auxiliary_percent: delta_f64(old_metrics.auxiliary_percent, new_metrics.auxiliary_percent),
    }
}

impl From<metrics::Metrics> for AnalysisMetrics {
    fn from(metrics: metrics::Metrics) -> Self {
        Self {
            qubit_count: metrics.qubit_count,
            depth: metrics.depth,
            x_count: metrics.x_count,
            y_count: metrics.y_count,
            z_count: metrics.z_count,
            pauli_count: metrics.pauli_count,
            hadamard_count: metrics.hadamard_count,
            rotation_count: metrics.rotation_count,
            square_root_count: metrics.square_root_count,
            measure_count: metrics.measure_count,
            swap_count: metrics.swap_count,
            cx_count: metrics.cx_count,
            gate_count: metrics.gate_count,
            single_gate_count: metrics.single_gate_count,
            controlled_gate_count: metrics.controlled_gate_count,
            auxiliary_qubit_count: metrics.auxiliary_qubit_count,
            gate_types_count: metrics.gate_types_count,
        }
    }
}

impl From<metrics::DeltaMetrics> for DeltaAnalysisMetrics {
    fn from(metrics: metrics::DeltaMetrics) -> Self {
        Self {
            qubit_count: metrics.qubit_count,
            depth: metrics.depth,
            x_count: metrics.x_count,
            y_count: metrics.y_count,
            z_count: metrics.z_count,
            pauli_count: metrics.pauli_count,
            hadamard_count: metrics.hadamard_count,
            rotation_count: metrics.rotation_count,
            square_root_count: metrics.square_root_count,
            measure_count: metrics.measure_count,
            swap_count: metrics.swap_count,
            cx_count: metrics.cx_count,
            gate_count: metrics.gate_count,
            single_gate_count: metrics.single_gate_count,
            controlled_gate_count: metrics.controlled_gate_count,
            auxiliary_qubit_count: metrics.auxiliary_qubit_count,
            gate_types_count: metrics.gate_types_count,
        }
    }
}

impl From<metrics::DetailedMetrics> for DetailedAnalysisMetrics {
    fn from(metrics: metrics::DetailedMetrics) -> Self {
        Self {
            width: metrics.width,
            depth: metrics.depth,
            max_density: metrics.max_density,
            average_density: metrics.average_density,
            x_count: metrics.x_count,
            y_count: metrics.y_count,
            z_count: metrics.z_count,
            pauli_count: metrics.pauli_count,
            hadamard_count: metrics.hadamard_count,
            initial_superposition_percent: metrics.initial_superposition_percent,
            other_single_qubit_count: metrics.other_single_qubit_count,
            single_qubit_count: metrics.single_qubit_count,
            single_controlled_qubit_count: metrics.single_controlled_qubit_count,
            swap_count: metrics.swap_count,
            cnot_count: metrics.cnot_count,
            cnot_qubit_percent: metrics.cnot_qubit_percent,
            average_cnot: metrics.average_cnot,
            max_cnot: metrics.max_cnot,
            toffoli_count: metrics.toffoli_count,
            toffoli_qubit_percent: metrics.toffoli_qubit_percent,
            average_toffoli: metrics.average_toffoli,
            max_toffoli: metrics.max_toffoli,
            gate_count: metrics.gate_count,
            controlled_gate_count: metrics.controlled_gate_count,
            single_qubit_percent: metrics.single_qubit_percent,
            measure_count: metrics.measure_count,
            measure_percent: metrics.measure_percent,
            auxiliary_percent: metrics.auxiliary_percent,
        }
    }
}

fn delta_usize(old: usize, new: usize) -> Option<isize> {
    let diff = new as isize - old as isize;
    (diff != 0).then_some(diff)
}

fn delta_f64(old: f64, new: f64) -> Option<f64> {
    let diff = new - old;
    (diff != 0.0).then_some(diff)
}
