use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Defines a set of metrics that can be used to estimate the complexity and quality of a quantum circuit.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Metrics {
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

/// Defines the difference in metrics between two quantum circuits.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct DeltaMetrics {
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

/// Represents quality metrics for a quantum circuit, providing insights into its structure and gate usage.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct DetailedMetrics {
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

impl Display for DetailedMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Circuit Size:")?;
        writeln!(f, "- Width: {}", self.width)?;
        writeln!(f, "- Depth: {}", self.depth)?;
        writeln!(f)?;
        writeln!(f, "Circuit Density:")?;
        writeln!(f, "- MaxDens (Max Density): {}", self.max_density)?;
        writeln!(
            f,
            "- AvgDens (Average Density): {:.2}",
            self.average_density
        )?;
        writeln!(f)?;
        writeln!(f, "Single-Qubit Gates:")?;
        writeln!(f, "- NoP-X (Pauli-X Count): {}", self.x_count)?;
        writeln!(f, "- NoP-Y (Pauli-Y Count): {}", self.y_count)?;
        writeln!(f, "- NoP-Z (Pauli-Z Count): {}", self.z_count)?;
        writeln!(f, "- TNo-P (Total Pauli Count): {}", self.pauli_count)?;
        writeln!(f, "- NoH (Hadamard Count): {}", self.hadamard_count)?;
        writeln!(
            f,
            "- %SpposQ (Initial Superposition Percent): {:.2}",
            self.initial_superposition_percent
        )?;
        writeln!(
            f,
            "- NoOtherSG (Other Single-Qubit Gates Count): {}",
            self.other_single_qubit_count
        )?;
        writeln!(
            f,
            "- TNoSQG (Total Single-Qubit Gates): {}",
            self.single_qubit_count
        )?;
        writeln!(
            f,
            "- TNoSQG (Controlled Single-Qubit Gates): {}",
            self.single_controlled_qubit_count
        )?;
        writeln!(f)?;
        writeln!(f, "Multi-Qubit Gates:")?;
        writeln!(f, "- NoSWAP (SWAP Count): {}", self.swap_count)?;
        writeln!(f, "- NoCNOT (CNOT Count): {}", self.cnot_count)?;
        writeln!(
            f,
            "- %QinCNOT (Ratio of qubits affected by CNOT Gates): {:.2}",
            self.cnot_qubit_percent
        )?;
        writeln!(
            f,
            "- AvgCNOT (Average CNOT Gates): {:.2}",
            self.average_cnot
        )?;
        writeln!(f, "- MaxCNOT (Max CNOT Gates): {}", self.max_cnot)?;
        writeln!(f, "- NoToff (Toffoli Count): {}", self.toffoli_count)?;
        writeln!(
            f,
            "- %QinToff (Ratio of qubits affected by Toffoli Gates): {:.2}",
            self.toffoli_qubit_percent
        )?;
        writeln!(
            f,
            "- AvgToff (Average Toffoli Gates): {:.2}",
            self.average_toffoli
        )?;
        writeln!(f, "- MaxToff (Max Toffoli Gates): {}", self.max_toffoli)?;
        writeln!(f)?;
        writeln!(f, "All Gates in the Circuit:")?;
        writeln!(f, "- NoGates (Total Gate Count): {}", self.gate_count)?;
        writeln!(
            f,
            "- NoCAnyG/NoCGates (Controlled Gate Count): {}",
            self.controlled_gate_count
        )?;
        writeln!(
            f,
            "- %SGates (Single Gate Percent): {:.2}",
            self.single_qubit_percent
        )?;
        writeln!(f)?;
        writeln!(f, "Measurement Gates:")?;
        writeln!(f, "- NoQM (Measured Qubit Count): {}", self.measure_count)?;
        writeln!(
            f,
            "- %QM (Measured Qubit Percent): {:.2}",
            self.measure_percent
        )?;
        writeln!(f)?;
        writeln!(f, "Other Measurements:")?;
        write!(
            f,
            "- %Anc (Ancilla Qubit Percent): {:.2}",
            self.auxiliary_percent
        )
    }
}
