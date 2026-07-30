use qctidy::Graph;
use thiserror::Error;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalysisMetrics {
    pub qubit_count: usize,
    pub depth: usize,
    pub x_count: usize,
    pub y_count: usize,
    pub z_count: usize,
    pub pauli_count: usize,
    pub hadamard_count: usize,
    pub rotation_count: usize,
    pub square_root_count: usize,
    pub measure_count: usize,
    pub swap_count: usize,
    pub cx_count: usize,
    pub gate_count: usize,
    pub single_gate_count: usize,
    pub controlled_gate_count: usize,
    pub auxiliary_qubit_count: usize,
    pub gate_types_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DetailedAnalysisMetrics {
    pub width: usize,
    pub depth: usize,
    pub max_density: usize,
    pub average_density: f64,
    pub x_count: usize,
    pub y_count: usize,
    pub z_count: usize,
    pub pauli_count: usize,
    pub hadamard_count: usize,
    pub initial_superposition_percent: f64,
    pub other_single_qubit_count: usize,
    pub single_qubit_count: usize,
    pub single_controlled_qubit_count: usize,
    pub swap_count: usize,
    pub cnot_count: usize,
    pub cnot_qubit_percent: f64,
    pub average_cnot: f64,
    pub max_cnot: usize,
    pub toffoli_count: usize,
    pub toffoli_qubit_percent: f64,
    pub average_toffoli: f64,
    pub max_toffoli: usize,
    pub gate_count: usize,
    pub controlled_gate_count: usize,
    pub single_qubit_percent: f64,
    pub measure_count: usize,
    pub measure_percent: f64,
    pub auxiliary_percent: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DeltaAnalysisMetrics {
    pub qubit_count: Option<isize>,
    pub depth: Option<isize>,
    pub x_count: Option<isize>,
    pub y_count: Option<isize>,
    pub z_count: Option<isize>,
    pub pauli_count: Option<isize>,
    pub hadamard_count: Option<isize>,
    pub rotation_count: Option<isize>,
    pub square_root_count: Option<isize>,
    pub measure_count: Option<isize>,
    pub swap_count: Option<isize>,
    pub cx_count: Option<isize>,
    pub gate_count: Option<isize>,
    pub single_gate_count: Option<isize>,
    pub controlled_gate_count: Option<isize>,
    pub auxiliary_qubit_count: Option<isize>,
    pub gate_types_count: Option<isize>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DeltaDetailedAnalysisMetrics {
    pub width: Option<isize>,
    pub depth: Option<isize>,
    pub max_density: Option<isize>,
    pub average_density: Option<f64>,
    pub x_count: Option<isize>,
    pub y_count: Option<isize>,
    pub z_count: Option<isize>,
    pub pauli_count: Option<isize>,
    pub hadamard_count: Option<isize>,
    pub initial_superposition_percent: Option<f64>,
    pub other_single_qubit_count: Option<isize>,
    pub single_qubit_count: Option<isize>,
    pub single_controlled_qubit_count: Option<isize>,
    pub swap_count: Option<isize>,
    pub cnot_count: Option<isize>,
    pub cnot_qubit_percent: Option<f64>,
    pub average_cnot: Option<f64>,
    pub max_cnot: Option<isize>,
    pub toffoli_count: Option<isize>,
    pub toffoli_qubit_percent: Option<f64>,
    pub average_toffoli: Option<f64>,
    pub max_toffoli: Option<isize>,
    pub gate_count: Option<isize>,
    pub controlled_gate_count: Option<isize>,
    pub single_qubit_percent: Option<f64>,
    pub measure_count: Option<isize>,
    pub measure_percent: Option<f64>,
    pub auxiliary_percent: Option<f64>,
}

#[derive(Debug, Clone, Copy, Error)]
pub enum AnalysisError {}
