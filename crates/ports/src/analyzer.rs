use qctidy::Graph;
use serde::{Deserialize, Serialize};

pub trait AnalyzerPort {
    fn analyze(&self, graph: &Graph, mode: AnalysisMode) -> AnalysisResult;
    fn compare(&self, old: &Graph, new: &Graph, mode: AnalysisMode) -> DeltaAnalysisResult;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisMode {
    Simple,
    Detailed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnalysisResult {
    Simple(AnalysisMetrics),
    Detailed(DetailedAnalysisMetrics),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeltaAnalysisResult {
    Simple(DeltaAnalysisMetrics),
    Detailed(DeltaDetailedAnalysisMetrics),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisMetrics {
    pub qubit_count: usize,
    pub bit_count: usize,
    pub time_step_count: usize,
    pub x_count: usize,
    pub y_count: usize,
    pub z_count: usize,
    pub pauli_count: usize,
    pub hadamard_count: usize,
    pub rotation_count: usize,
    pub phase_count: usize,
    pub square_root_count: usize,
    pub t_count: usize,
    pub swap_count: usize,
    pub cx_count: usize,
    pub ccx_count: usize,
    pub operation_count: usize,
    pub gate_count: usize,
    pub measurement_count: usize,
    pub single_gate_count: usize,
    pub controlled_gate_count: usize,
    pub measured_qubit_count: usize,
    pub unmeasured_qubit_count: usize,
    pub gate_types_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DetailedAnalysisMetrics {
    pub qubit_count: usize,
    pub time_step_count: usize,
    pub max_density: usize,
    pub average_density: f64,
    pub x_count: usize,
    pub y_count: usize,
    pub z_count: usize,
    pub pauli_count: usize,
    pub hadamard_count: usize,
    pub initial_superposition_percent: f64,
    pub other_single_gate_count: usize,
    pub single_gate_count: usize,
    pub single_controlled_gate_count: usize,
    pub swap_count: usize,
    pub cx_count: usize,
    pub cx_qubit_percent: f64,
    pub average_cx: f64,
    pub max_cx_count: usize,
    pub ccx_count: usize,
    pub ccx_qubit_percent: f64,
    pub average_ccx: f64,
    pub max_ccx_count: usize,
    pub operation_count: usize,
    pub gate_count: usize,
    pub measurement_count: usize,
    pub controlled_gate_count: usize,
    pub single_gate_percent: f64,
    pub measured_qubit_count: usize,
    pub measure_percent: f64,
    pub unmeasured_qubit_percent: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeltaAnalysisMetrics {
    pub qubit_count: isize,
    pub bit_count: isize,
    pub time_step_count: isize,
    pub x_count: isize,
    pub y_count: isize,
    pub z_count: isize,
    pub pauli_count: isize,
    pub hadamard_count: isize,
    pub rotation_count: isize,
    pub phase_count: isize,
    pub square_root_count: isize,
    pub t_count: isize,
    pub swap_count: isize,
    pub cx_count: isize,
    pub ccx_count: isize,
    pub operation_count: isize,
    pub gate_count: isize,
    pub measurement_count: isize,
    pub single_gate_count: isize,
    pub controlled_gate_count: isize,
    pub measured_qubit_count: isize,
    pub unmeasured_qubit_count: isize,
    pub gate_types_count: isize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct DeltaDetailedAnalysisMetrics {
    pub qubit_count: isize,
    pub time_step_count: isize,
    pub max_density: isize,
    pub average_density: f64,
    pub x_count: isize,
    pub y_count: isize,
    pub z_count: isize,
    pub pauli_count: isize,
    pub hadamard_count: isize,
    pub initial_superposition_percent: f64,
    pub other_single_gate_count: isize,
    pub single_gate_count: isize,
    pub single_controlled_gate_count: isize,
    pub swap_count: isize,
    pub cx_count: isize,
    pub cx_qubit_percent: f64,
    pub average_cx: f64,
    pub max_cx_count: isize,
    pub ccx_count: isize,
    pub ccx_qubit_percent: f64,
    pub average_ccx: f64,
    pub max_ccx_count: isize,
    pub operation_count: isize,
    pub gate_count: isize,
    pub measurement_count: isize,
    pub controlled_gate_count: isize,
    pub single_gate_percent: f64,
    pub measured_qubit_count: isize,
    pub measure_percent: f64,
    pub unmeasured_qubit_percent: f64,
}
