use getset::{CopyGetters, Getters};
use inew::New;

use crate::domain::GateType;

/// An operation applied to a circuit, also known as a quantum gate.
#[derive(Debug, Clone, New)]
#[new(pub, const, no_prefix)]
pub enum GateOperation {
    ID {
        qubit: usize,
    },
    H {
        qubit: usize,
    },
    X {
        qubit: usize,
    },
    Y {
        qubit: usize,
    },
    Z {
        qubit: usize,
    },
    P {
        angle: f64,
        qubit: usize,
    },
    RX {
        angle: f64,
        qubit: usize,
    },
    RY {
        angle: f64,
        qubit: usize,
    },
    RZ {
        angle: f64,
        qubit: usize,
    },
    S {
        qubit: usize,
    },
    SDG {
        qubit: usize,
    },
    SX {
        qubit: usize,
    },
    SY {
        qubit: usize,
    },
    T {
        qubit: usize,
    },
    TDG {
        qubit: usize,
    },
    Measure {
        qubit: usize,
        bit: usize,
    },
    Swap {
        qubit1: usize,
        qubit2: usize,
    },
    CH {
        control: usize,
        target: usize,
    },
    CX {
        control: usize,
        target: usize,
    },
    CY {
        control: usize,
        target: usize,
    },
    CZ {
        qubit1: usize,
        qubit2: usize,
    },
    CP {
        angle: f64,
        control: usize,
        target: usize,
    },
    CSwap {
        control: usize,
        target1: usize,
        target2: usize,
    },
    CCX {
        control1: usize,
        control2: usize,
        target: usize,
    },
    CCZ {
        qubit1: usize,
        qubit2: usize,
        qubit3: usize,
    },
}

impl GateOperation {
    /// Get the type of this gate operation.
    pub fn r#type(&self) -> GateType {
        use GateOperation::*;

        match self {
            ID { .. } => GateType::ID,
            H { .. } => GateType::H,
            X { .. } => GateType::X,
            Y { .. } => GateType::Y,
            Z { .. } => GateType::Z,
            P { .. } => GateType::P,
            RX { .. } => GateType::RX,
            RY { .. } => GateType::RY,
            RZ { .. } => GateType::RZ,
            S { .. } => GateType::S,
            SDG { .. } => GateType::SDG,
            SX { .. } => GateType::SX,
            SY { .. } => GateType::SY,
            T { .. } => GateType::T,
            TDG { .. } => GateType::TDG,
            Measure { .. } => GateType::Measure,
            Swap { .. } => GateType::Swap,
            CH { .. } => GateType::CH,
            CX { .. } => GateType::CX,
            CY { .. } => GateType::CY,
            CZ { .. } => GateType::CZ,
            CP { .. } => GateType::CP,
            CSwap { .. } => GateType::CSwap,
            CCX { .. } => GateType::CCX,
            CCZ { .. } => GateType::CCZ,
        }
    }

    /// Get the qubits affected by this gate operation.
    ///
    /// ID gates return an empty vector because they add no value to the circuit.
    pub fn qubits(&self) -> Vec<usize> {
        use GateOperation::*;

        match self {
            ID { .. } => vec![],
            H { qubit } => vec![*qubit],
            X { qubit } => vec![*qubit],
            Y { qubit } => vec![*qubit],
            Z { qubit } => vec![*qubit],
            P { qubit, .. } => vec![*qubit],
            RX { qubit, .. } => vec![*qubit],
            RY { qubit, .. } => vec![*qubit],
            RZ { qubit, .. } => vec![*qubit],
            S { qubit } => vec![*qubit],
            SDG { qubit } => vec![*qubit],
            SX { qubit } => vec![*qubit],
            SY { qubit } => vec![*qubit],
            T { qubit } => vec![*qubit],
            TDG { qubit } => vec![*qubit],
            Measure { qubit, .. } => vec![*qubit],
            Swap { qubit1, qubit2 } => vec![*qubit1, *qubit2],
            CH { control, target } => vec![*control, *target],
            CX { control, target } => vec![*control, *target],
            CY { control, target } => vec![*control, *target],
            CZ { qubit1, qubit2 } => vec![*qubit1, *qubit2],
            CP {
                control, target, ..
            } => vec![*control, *target],
            CSwap {
                control,
                target1,
                target2,
            } => vec![*control, *target1, *target2],
            CCX {
                control1,
                control2,
                target,
            } => vec![*control1, *control2, *target],
            CCZ {
                qubit1,
                qubit2,
                qubit3,
            } => vec![*qubit1, *qubit2, *qubit3],
        }
    }

    /// Get the classical bits affected by this gate operation.
    pub fn bits(&self) -> Vec<usize> {
        use GateOperation::*;

        match self {
            Measure { bit, .. } => vec![*bit],
            _ => vec![],
        }
    }
}

/// An operation applied to a circuit, also known as a quantum gate.
///
/// It's designed to be similar to Qiskit's operations.
#[derive(Debug, Getters, CopyGetters)]
pub struct GateOperation2 {
    /// The type of gate operation.
    #[get_copy = "pub"]
    r#type: GateType,
    /// The qubits affected by the operation.
    #[get = "pub"]
    qubits: Vec<usize>,
    /// The bits affected by the operation.
    #[get = "pub"]
    bits: Vec<usize>,
    /// The parameters (angles) associated with the operation.
    #[get = "pub"]
    parameters: Vec<f64>,
}

impl GateOperation2 {
    /// Create a new `GateOperation`.
    pub fn new(
        r#type: GateType,
        qubits: Vec<usize>,
        bits: Vec<usize>,
        parameters: Vec<f64>,
    ) -> Self {
        Self {
            r#type,
            qubits,
            bits,
            parameters,
        }
    }
}
