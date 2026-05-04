use std::collections::HashSet;

use crate::GateType;
use thiserror::Error;

/// Error that can occur when building a `GateOperation`.
#[derive(Debug, Error)]
pub enum GateOperationError {
    #[error("Angle must be finite, got {angle}")]
    NonFiniteAngle { angle: f64 },
    #[error("Qubits {qubits} must be different")]
    RepeatedQubits { qubits: String },
}

fn check_indices(fields: &[&str], qubits: &[usize]) -> Result<(), GateOperationError> {
    if qubits.len() != qubits.iter().collect::<HashSet<_>>().len() {
        return Err(GateOperationError::RepeatedQubits {
            qubits: format_multiple_fields(fields),
        });
    }

    Ok(())
}

fn format_multiple_fields(fields: &[&str]) -> String {
    if fields.is_empty() {
        return String::new();
    }

    if fields.len() == 1 {
        return format!("'{}'", fields[0]);
    }

    if fields.len() == 2 {
        return format!("'{}' and '{}'", fields[0], fields[1]);
    }

    let fields_except_last: Vec<&str> = fields.iter().take(fields.len() - 1).copied().collect();
    let last_field = fields.last().expect("There should be at least 3 fields");

    format!("'{}' and '{}'", fields_except_last.join(", "), last_field)
}

/// A quantum gate operation, which can be applied to a circuit.
///
/// Prefer using the safe constructors instead of directly using the `GateOperation` enum variants.
#[derive(Debug, Clone, Copy)]
#[must_use]
pub enum GateOperation {
    /// The identity gate. It has no practical effect.
    ID {
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit Hadamard gate.
    H {
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit X gate. Also known as the NOT gate.
    X {
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit Y gate.
    Y {
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit Z gate.
    Z {
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit phase gate.
    P {
        /// The phase shift applied to this gate.
        angle: f64,
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit rotation gate, which rotates around the X axis.
    RX {
        /// The angle that this gate is rotated by.
        angle: f64,
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit rotation gate, which rotates around the Y axis.
    RY {
        /// The angle that this gate is rotated by.
        angle: f64,
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit rotation gate, which rotates around the Z axis.
    RZ {
        /// The angle that this gate is rotated by.
        angle: f64,
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit S gate. Also known as the sqrt(Z) gate.
    S {
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit S dagger gate. The conjugate transpose of the S gate.
    SDG {
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit sqrt(X) gate.
    SX {
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit sqrt(Y) gate.
    SY {
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit T gate. Equivalent to Z^0.25.
    T {
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit T dagger gate. The conjugate transpose of the T gate.
    TDG {
        /// The qubit that this gate is placed on.
        qubit: usize,
    },
    /// Single-qubit measurement gate. Stores its results on a particular bit.
    Measure {
        /// The qubit that this gate is placed on.
        qubit: usize,
        /// The bit to store the measurement result in.
        bit: usize,
    },
    /// Two-qubit SWAP gate.
    Swap {
        /// The first qubit, which swaps with the second.
        qubit1: usize,
        /// The second qubit, which swaps with the first.
        qubit2: usize,
    },
    /// Two-qubit controlled Hadamard gate.
    CH {
        /// The control qubit.
        control: usize,
        /// The target qubit.
        target: usize,
    },
    /// Two-qubit controlled X gate. Also known as the CNOT gate.
    CX {
        /// The control qubit.
        control: usize,
        /// The target qubit.
        target: usize,
    },
    /// Two-qubit controlled Y gate.
    CY {
        /// The control qubit.
        control: usize,
        /// The target qubit.
        target: usize,
    },
    /// Two-qubit controlled Z gate.
    CZ {
        /// The first qubit.
        qubit1: usize,
        /// The second qubit.
        qubit2: usize,
    },
    /// Two-qubit controlled phase gate.
    CP {
        /// The phase shift applied to this gate.
        angle: f64,
        /// The first qubit.
        qubit1: usize,
        /// The second qubit.
        qubit2: usize,
    },
    /// Three-qubit controlled SWAP gate. Also known as the Fredkin gate.
    CSwap {
        control: usize,
        /// The first target qubit, which swaps with the second.
        target1: usize,
        /// The second target qubit, which swaps with the first.
        target2: usize,
    },
    /// Three-qubit X Gate, controlled by 2 qubits. Also known as the Toffoli gate.
    CCX {
        /// The first control qubit.
        control1: usize,
        /// The second control qubit.
        control2: usize,
        /// The target qubit.
        target: usize,
    },
    /// Three-qubit Z Gate, controlled by 2 qubits.
    CCZ {
        /// The first qubit.
        qubit1: usize,
        /// The second qubit.
        qubit2: usize,
        /// The third qubit.
        qubit3: usize,
    },
}

impl GateOperation {
    pub const fn id(qubit: usize) -> Self {
        Self::ID { qubit }
    }

    pub const fn h(qubit: usize) -> Self {
        Self::H { qubit }
    }

    pub const fn x(qubit: usize) -> Self {
        Self::X { qubit }
    }

    pub const fn y(qubit: usize) -> Self {
        Self::Y { qubit }
    }

    pub const fn z(qubit: usize) -> Self {
        Self::Z { qubit }
    }

    pub const fn try_p(angle: f64, qubit: usize) -> Result<Self, GateOperationError> {
        if !angle.is_finite() {
            return Err(GateOperationError::NonFiniteAngle { angle });
        }

        Ok(Self::P { angle, qubit })
    }

    pub const fn try_rx(angle: f64, qubit: usize) -> Result<Self, GateOperationError> {
        if !angle.is_finite() {
            return Err(GateOperationError::NonFiniteAngle { angle });
        }

        Ok(Self::RX { angle, qubit })
    }

    pub const fn try_ry(angle: f64, qubit: usize) -> Result<Self, GateOperationError> {
        if !angle.is_finite() {
            return Err(GateOperationError::NonFiniteAngle { angle });
        }

        Ok(Self::RY { angle, qubit })
    }

    pub const fn try_rz(angle: f64, qubit: usize) -> Result<Self, GateOperationError> {
        if !angle.is_finite() {
            return Err(GateOperationError::NonFiniteAngle { angle });
        }

        Ok(Self::RZ { angle, qubit })
    }

    pub const fn s(qubit: usize) -> Self {
        Self::S { qubit }
    }

    pub const fn sdg(qubit: usize) -> Self {
        Self::SDG { qubit }
    }

    pub const fn sx(qubit: usize) -> Self {
        Self::SX { qubit }
    }

    pub const fn sy(qubit: usize) -> Self {
        Self::SY { qubit }
    }

    pub const fn t(qubit: usize) -> Self {
        Self::T { qubit }
    }

    pub const fn tdg(qubit: usize) -> Self {
        Self::TDG { qubit }
    }

    pub const fn measure(qubit: usize, bit: usize) -> Self {
        Self::Measure { qubit, bit }
    }

    pub fn try_swap(qubit1: usize, qubit2: usize) -> Result<Self, GateOperationError> {
        check_indices(&["qubit1", "qubit2"], &[qubit1, qubit2])?;
        Ok(Self::Swap { qubit1, qubit2 })
    }

    pub fn try_ch(control: usize, target: usize) -> Result<Self, GateOperationError> {
        check_indices(&["control", "target"], &[control, target])?;
        Ok(Self::CH { control, target })
    }

    pub fn try_cx(control: usize, target: usize) -> Result<Self, GateOperationError> {
        check_indices(&["control", "target"], &[control, target])?;
        Ok(Self::CX { control, target })
    }

    pub fn try_cy(control: usize, target: usize) -> Result<Self, GateOperationError> {
        check_indices(&["control", "target"], &[control, target])?;
        Ok(Self::CY { control, target })
    }

    pub fn try_cz(qubit1: usize, qubit2: usize) -> Result<Self, GateOperationError> {
        check_indices(&["qubit1", "qubit2"], &[qubit1, qubit2])?;
        Ok(Self::CZ { qubit1, qubit2 })
    }

    pub fn try_cp(angle: f64, qubit1: usize, qubit2: usize) -> Result<Self, GateOperationError> {
        if !angle.is_finite() {
            return Err(GateOperationError::NonFiniteAngle { angle });
        }

        check_indices(&["qubit1", "qubit2"], &[qubit1, qubit2])?;

        Ok(Self::CP {
            angle,
            qubit1,
            qubit2,
        })
    }

    pub fn try_c_swap(
        control: usize,
        target1: usize,
        target2: usize,
    ) -> Result<Self, GateOperationError> {
        check_indices(
            &["control", "target1", "target2"],
            &[control, target1, target2],
        )?;

        Ok(Self::CSwap {
            control,
            target1,
            target2,
        })
    }

    pub fn try_ccx(
        control1: usize,
        control2: usize,
        target: usize,
    ) -> Result<Self, GateOperationError> {
        check_indices(
            &["control1", "control2", "target"],
            &[control1, control2, target],
        )?;

        Ok(Self::CCX {
            control1,
            control2,
            target,
        })
    }

    pub fn try_ccz(
        qubit1: usize,
        qubit2: usize,
        qubit3: usize,
    ) -> Result<Self, GateOperationError> {
        check_indices(&["qubit1", "qubit2", "qubit3"], &[qubit1, qubit2, qubit3])?;

        Ok(Self::CCZ {
            qubit1,
            qubit2,
            qubit3,
        })
    }

    /// The type of this gate, which is essentially a flattened version of `GateOperation`.
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

    /// Get all the qubits used by this operation.
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

    /// Get all the bits used by this operation.
    #[must_use]
    pub fn bits(&self) -> Vec<usize> {
        use GateOperation::*;

        match *self {
            Measure { bit, .. } => vec![bit],
            _ => vec![],
        }
    }
}
