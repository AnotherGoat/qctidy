use inew::New;

use crate::GateType;

/// An operation applied to a circuit, also known as a quantum gate.
#[derive(Debug, Clone, Copy, New)]
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
        qubit1: usize,
        qubit2: usize,
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
    #[must_use]
    pub const fn r#type(&self) -> GateType {
        use GateOperation::*;

        match *self {
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
    #[must_use]
    pub fn qubits(&self) -> Vec<usize> {
        use GateOperation::*;

        match *self {
            ID { .. } => vec![],
            H { qubit }
            | X { qubit }
            | Y { qubit }
            | Z { qubit }
            | P { qubit, .. }
            | RX { qubit, .. }
            | RY { qubit, .. }
            | RZ { qubit, .. }
            | S { qubit }
            | SDG { qubit }
            | SX { qubit }
            | SY { qubit }
            | T { qubit }
            | TDG { qubit }
            | Measure { qubit, .. } => vec![qubit],
            Swap { qubit1, qubit2 } | CZ { qubit1, qubit2 } | CP { qubit1, qubit2, .. } => {
                vec![qubit1, qubit2]
            }
            CH { control, target } | CX { control, target } | CY { control, target } => {
                vec![control, target]
            }
            CSwap {
                control,
                target1,
                target2,
            } => vec![control, target1, target2],
            CCX {
                control1,
                control2,
                target,
            } => vec![control1, control2, target],
            CCZ {
                qubit1,
                qubit2,
                qubit3,
            } => vec![qubit1, qubit2, qubit3],
        }
    }

    /// Get the classical bits affected by this gate operation.
    #[must_use]
    pub fn bits(&self) -> Vec<usize> {
        use GateOperation::*;

        match *self {
            Measure { bit, .. } => {
                vec![bit]
            }
            _ => vec![],
        }
    }
}
