use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::GATE_METADATAS;

#[derive(Debug, Clone, Error)]
pub enum GateTypeError {
    #[error("Unknown gate type with index '{index}'")]
    UnknownGateIndex { index: u8 },
    #[error("Unknown gate type name or alias '{name}'")]
    UnknownGateName { name: String },
}

/// Type of a quantum gate supported by this library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GateType {
    /// The identity gate. It explicitly occupies a position in the graph.
    ID,
    /// Single-qubit Hadamard gate.
    H,
    /// Single-qubit X gate. Also known as the NOT gate.
    X,
    /// Single-qubit Y gate.
    Y,
    /// Single-qubit Z gate.
    Z,
    /// Single-qubit phase gate.
    P,
    /// Single-qubit rotation gate, which rotates around the X axis.
    RX,
    /// Single-qubit rotation gate, which rotates around the Y axis.
    RY,
    /// Single-qubit rotation gate, which rotates around the Z axis.
    RZ,
    /// Single-qubit S gate. Also known as the sqrt(Z) gate.
    S,
    /// Single-qubit S dagger gate. The conjugate transpose of the S gate.
    SDG,
    /// Single-qubit sqrt(X) gate.
    SX,
    /// Single-qubit sqrt(Y) gate.
    SY,
    /// Single-qubit T gate. Equivalent to Z^0.25.
    T,
    /// Single-qubit T dagger gate. The conjugate transpose of the T gate.
    TDG,
    /// Single-qubit general unitary gate with three rotation angles: theta, phi, lambda.
    U,
    /// Single-qubit measurement gate. Stores its results on a particular bit.
    Measure,
    /// Two-qubit SWAP gate.
    Swap,
    /// Two-qubit controlled Hadamard gate.
    CH,
    /// Two-qubit controlled X gate. Also known as the CNOT gate.
    CX,
    /// Two-qubit controlled Y gate.
    CY,
    /// Two-qubit controlled Z gate.
    CZ,
    /// Two-qubit controlled phase gate.
    CP,
    /// Three-qubit controlled SWAP gate. Also known as the Fredkin gate.
    CSwap,
    /// Three-qubit X Gate, controlled by 2 qubits. Also known as the Toffoli gate.
    CCX,
    /// Three-qubit Z Gate, controlled by 2 qubits.
    CCZ,
}

impl From<GateType> for u8 {
    #[expect(clippy::as_conversions)]
    fn from(gate_type: GateType) -> Self {
        gate_type as Self
    }
}

impl TryFrom<u8> for GateType {
    type Error = GateTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GateType::*;

        match value {
            0 => Ok(ID),
            1 => Ok(H),
            2 => Ok(X),
            3 => Ok(Y),
            4 => Ok(Z),
            5 => Ok(P),
            6 => Ok(RX),
            7 => Ok(RY),
            8 => Ok(RZ),
            9 => Ok(S),
            10 => Ok(SDG),
            11 => Ok(SX),
            12 => Ok(SY),
            13 => Ok(T),
            14 => Ok(TDG),
            15 => Ok(U),
            16 => Ok(Measure),
            17 => Ok(Swap),
            18 => Ok(CH),
            19 => Ok(CX),
            20 => Ok(CY),
            21 => Ok(CZ),
            22 => Ok(CP),
            23 => Ok(CSwap),
            24 => Ok(CCX),
            25 => Ok(CCZ),
            _ => Err(GateTypeError::UnknownGateIndex { index: value }),
        }
    }
}

impl From<GateType> for usize {
    fn from(gate_type: GateType) -> Self {
        gate_type as Self
    }
}

impl TryFrom<usize> for GateType {
    type Error = GateTypeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_from(value as u8)
    }
}

impl fmt::Display for GateType {
    /// Obtain the name of this gate type as a lowercase string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GateType::*;

        let name = match *self {
            ID => "id",
            H => "h",
            X => "x",
            Y => "y",
            Z => "z",
            P => "p",
            RX => "rx",
            RY => "ry",
            RZ => "rz",
            S => "s",
            SDG => "sdg",
            SX => "sx",
            SY => "sy",
            T => "t",
            TDG => "tdg",
            U => "u",
            Measure => "m",
            Swap => "swap",
            CH => "ch",
            CX => "cx",
            CY => "cy",
            CZ => "cz",
            CP => "cp",
            CSwap => "cswap",
            CCX => "ccx",
            CCZ => "ccz",
        };

        write!(f, "{name}")
    }
}

impl FromStr for GateType {
    type Err = GateTypeError;

    /// Create a gate type from a string written in any case combination.
    ///
    /// Fails if the string is not a known gate type.
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        use GateType::*;

        match name.to_ascii_lowercase().as_str() {
            "i" | "id" | "identity" => Ok(ID),
            "h" | "hadamard" => Ok(H),
            "x" | "not" => Ok(X),
            "y" => Ok(Y),
            "z" => Ok(Z),
            "p" | "phase" => Ok(P),
            "rx" => Ok(RX),
            "ry" => Ok(RY),
            "rz" => Ok(RZ),
            "s" | "sz" | "sqrtz" => Ok(S),
            "sd" | "sdg" | "szd" | "szdg" | "sqrtzd" | "sqrtzdg" => Ok(SDG),
            "sx" | "sqrtx" => Ok(SX),
            "sy" | "sqrty" => Ok(SY),
            "t" => Ok(T),
            "td" | "tdg" => Ok(TDG),
            "u" | "u3" => Ok(U),
            "m" | "measure" => Ok(Measure),
            "swap" => Ok(Swap),
            "ch" => Ok(CH),
            "cx" | "cnot" => Ok(CX),
            "cy" => Ok(CY),
            "cz" => Ok(CZ),
            "cp" | "cphase" => Ok(CP),
            "cswap" | "fredkin" => Ok(CSwap),
            "ccx" | "ccnot" | "toffoli" => Ok(CCX),
            "ccz" => Ok(CCZ),
            _ => Err(GateTypeError::UnknownGateName {
                name: name.to_owned(),
            }),
        }
    }
}

impl GateType {
    /// Get the number of qubits used by this type of gate.
    pub(crate) fn qubit_count(self) -> usize {
        GATE_METADATAS[usize::from(self)].qubit_count()
    }

    /// Check whether this gate type uses multiple qubits or not.
    pub(crate) fn is_multi_qubit(self) -> bool {
        self.qubit_count() > 1
    }

    /// Get the number of control qubits used by this type of gate.
    ///
    /// If a gate has control qubits, it also has at least one target qubit.
    pub(crate) fn control_qubit_count(self) -> usize {
        GATE_METADATAS[usize::from(self)].control_qubit_count()
    }

    /// Check whether this gate type has control and target qubits or not.
    pub(crate) fn is_controlled(self) -> bool {
        self.control_qubit_count() > 0
    }

    /// Get the number of qubits affected by this type of gate.
    ///
    /// Measurement gates return 0 because they don't modify the quantum state.
    /// Having a target qubit doesn't guarantee that the gate has a control qubit.
    /// All single-qubit gates have a target qubit.
    pub(crate) fn target_qubit_count(self) -> usize {
        GATE_METADATAS[usize::from(self)].target_qubit_count()
    }

    /// Check whether this gate type has a single control and a single target qubit.
    pub(crate) fn is_single_controlled(self) -> bool {
        self.control_qubit_count() == 1 && self.target_qubit_count() == 1
    }

    /// Get the number of classical bits used by this type of gate.
    pub(crate) fn bit_count(self) -> usize {
        GATE_METADATAS[usize::from(self)].bit_count()
    }

    /// Check whether this gate type is a phase gate or not.
    pub(crate) fn is_phase(self) -> bool {
        GATE_METADATAS[usize::from(self)].is_phase()
    }

    /// Check whether this gate type is a rotation gate or not.
    pub(crate) fn is_rotation(self) -> bool {
        GATE_METADATAS[usize::from(self)].is_rotation()
    }

    /// Check whether this gate type is a square root or not.
    pub(crate) fn is_square_root(self) -> bool {
        GATE_METADATAS[usize::from(self)].is_square_root()
    }
}
