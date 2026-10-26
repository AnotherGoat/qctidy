pub mod calculator;

use qctidy::Graph;
use qctidy::number::truncate_usize_to_isize;
use qctidy_ports::{
    AnalysisMode, AnalysisResult, AnalyzerPort, DeltaAnalysisResult, DeltaDetailedAnalysisMetrics,
    DetailedAnalysisMetrics,
};

#[derive(Debug, Copy, Clone)]
pub struct AnalyzerAdapter;

impl AnalyzerPort for AnalyzerAdapter {
    fn analyze(&self, graph: &Graph, mode: AnalysisMode) -> AnalysisResult {
        match mode {
            AnalysisMode::Simple => AnalysisResult::Simple(calculator::calculate_metrics(graph)),
            AnalysisMode::Detailed => AnalysisResult::Detailed(calculate_detailed(graph)),
        }
    }

    fn compare(&self, old: &Graph, new: &Graph, mode: AnalysisMode) -> DeltaAnalysisResult {
        match mode {
            AnalysisMode::Simple => {
                DeltaAnalysisResult::Simple(calculator::compare_metrics(old, new))
            }
            AnalysisMode::Detailed => {
                DeltaAnalysisResult::Detailed(compare_detailed_metrics(old, new))
            }
        }
    }
}

fn calculate_detailed(graph: &Graph) -> DetailedAnalysisMetrics {
    calculator::calculate_detailed_metrics(graph)
}

fn compare_detailed_metrics(old: &Graph, new: &Graph) -> DeltaDetailedAnalysisMetrics {
    let old_metrics = calculate_detailed(old);
    let new_metrics = calculate_detailed(new);

    DeltaDetailedAnalysisMetrics {
        qubit_count: delta_usize(old_metrics.qubit_count, new_metrics.qubit_count),
        time_step_count: delta_usize(old_metrics.time_step_count, new_metrics.time_step_count),
        max_density: delta_usize(old_metrics.max_density, new_metrics.max_density),
        average_density: delta_f64(old_metrics.average_density, new_metrics.average_density),
        x_count: delta_usize(old_metrics.x_count, new_metrics.x_count),
        y_count: delta_usize(old_metrics.y_count, new_metrics.y_count),
        z_count: delta_usize(old_metrics.z_count, new_metrics.z_count),
        pauli_count: delta_usize(old_metrics.pauli_count, new_metrics.pauli_count),
        hadamard_count: delta_usize(old_metrics.hadamard_count, new_metrics.hadamard_count),
        initial_superposition_ratio: delta_f64(
            old_metrics.initial_superposition_ratio,
            new_metrics.initial_superposition_ratio,
        ),
        other_single_gate_count: delta_usize(
            old_metrics.other_single_gate_count,
            new_metrics.other_single_gate_count,
        ),
        single_gate_count: delta_usize(
            old_metrics.single_gate_count,
            new_metrics.single_gate_count,
        ),
        single_controlled_gate_count: delta_usize(
            old_metrics.single_controlled_gate_count,
            new_metrics.single_controlled_gate_count,
        ),
        swap_count: delta_usize(old_metrics.swap_count, new_metrics.swap_count),
        cx_count: delta_usize(old_metrics.cx_count, new_metrics.cx_count),
        cx_qubit_ratio: delta_f64(old_metrics.cx_qubit_ratio, new_metrics.cx_qubit_ratio),
        average_cx: delta_f64(old_metrics.average_cx, new_metrics.average_cx),
        max_cx_count: delta_usize(old_metrics.max_cx_count, new_metrics.max_cx_count),
        ccx_count: delta_usize(old_metrics.ccx_count, new_metrics.ccx_count),
        ccx_qubit_ratio: delta_f64(old_metrics.ccx_qubit_ratio, new_metrics.ccx_qubit_ratio),
        average_ccx: delta_f64(old_metrics.average_ccx, new_metrics.average_ccx),
        max_ccx_count: delta_usize(old_metrics.max_ccx_count, new_metrics.max_ccx_count),
        operation_count: delta_usize(old_metrics.operation_count, new_metrics.operation_count),
        gate_count: delta_usize(old_metrics.gate_count, new_metrics.gate_count),
        measurement_count: delta_usize(
            old_metrics.measurement_count,
            new_metrics.measurement_count,
        ),
        controlled_gate_count: delta_usize(
            old_metrics.controlled_gate_count,
            new_metrics.controlled_gate_count,
        ),
        single_gate_ratio: delta_f64(old_metrics.single_gate_ratio, new_metrics.single_gate_ratio),
        measured_qubit_count: delta_usize(
            old_metrics.measured_qubit_count,
            new_metrics.measured_qubit_count,
        ),
        measure_ratio: delta_f64(old_metrics.measure_ratio, new_metrics.measure_ratio),
        unmeasured_qubit_ratio: delta_f64(
            old_metrics.unmeasured_qubit_ratio,
            new_metrics.unmeasured_qubit_ratio,
        ),
    }
}

const fn delta_usize(old: usize, new: usize) -> isize {
    truncate_usize_to_isize(new) - truncate_usize_to_isize(old)
}

const fn delta_f64(old: f64, new: f64) -> f64 {
    new - old
}

#[cfg(test)]
mod calculator_tests;
