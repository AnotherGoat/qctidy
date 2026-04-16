use crate::domain::GateType;

#[derive(Debug, Clone)]
pub enum QuantumGate {
    H { qubit: usize },
    X { qubit: usize },
    CX { control: usize, target: usize },
    Measure { qubit: usize, bit: usize },
}

pub struct GateOperation {
    pub r#type: GateType,
    pub qubits: Vec<usize>,
    pub bits: Vec<usize>,
    pub parameters: Vec<f64>,
}
