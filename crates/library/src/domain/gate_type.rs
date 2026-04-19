use std::{fmt, str::FromStr};

/// Type of a quantum gate supported by this library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateType {
    /// The identity gate. It has no practical effect.
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

impl fmt::Display for GateType {
    /// Obtain the name of this gate type as a lowercase string.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GateType::*;

        let name = match self {
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

        write!(formatter, "{}", name)
    }
}

impl FromStr for GateType {
    type Err = String;

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
            "m" | "measure" => Ok(Measure),
            "swap" => Ok(Swap),
            "ch" => Ok(CH),
            "cx" | "cnot" => Ok(CX),
            "cy" => Ok(CY),
            "cz" => Ok(CZ),
            "cp" | "cphase" => Ok(CP),
            "cswap" | "fredkin" => Ok(CSwap),
            "ccx" | "ccnot" => Ok(CCX),
            "ccz" | "toffoli" => Ok(CCZ),
            _ => Err(format!("Unknown gate type: {}", name)),
        }
    }
}

impl GateType {
    /// Check whether this gate type is a phase gate or not.
    pub(crate) fn is_phase(self) -> bool {
        use GateType::*;
        matches!(self, P | CP)
    }

    /// Check whether this gate type is a rotation gate or not,
    pub(crate) fn is_rotation(self) -> bool {
        use GateType::*;
        matches!(self, RX | RY | RZ)
    }

    /// Check whether this gate type is a square root or not.
    pub(crate) fn is_square_root(self) -> bool {
        use GateType::*;
        matches!(self, S | SDG | SX | SY)
    }

    /// Get the number of qubits used by this type of gate.
    ///
    /// Identity gates return 0 because they add no value to the circuit.
    pub(crate) fn qubit_count(self) -> usize {
        use GateType::*;
        match self {
            ID => 0,
            Swap | CH | CX | CY | CZ | CP => 2,
            CSwap | CCX | CCZ => 3,
            _ => 1,
        }
    }

    /// Check whether this gate type uses multiple qubits or not.
    pub(crate) fn is_multi_qubit(self) -> bool {
        self.qubit_count() > 1
    }

    /// Get the number of control qubits used by this type of gate.
    ///
    /// If a gate has control qubits, it also has at least one target qubit.
    pub(crate) fn control_qubit_count(self) -> usize {
        use GateType::*;
        match self {
            CH | CX | CY | CZ | CP | CSwap => 1,
            CCX | CCZ => 2,
            _ => 0,
        }
    }

    /// Check whether this gate type has control and target qubits or not.
    pub(crate) fn is_controlled(self) -> bool {
        self.control_qubit_count() > 0
    }

    /// Get the number of qubits affected by this type of gate.
    ///
    /// Identity and measurement gates return 0 because they target a single qubit, but make no changes to it.
    /// Having a target qubit doesn't guarantee that the gate has a control qubit.
    /// All single-qubit gates have a target qubit.
    pub(crate) fn target_qubit_count(self) -> usize {
        use GateType::*;
        match self {
            ID | Measure => 0,
            Swap | CSwap => 2,
            _ => 1,
        }
    }

    /// Check whether this gate type has a single control and a single target qubit.
    pub(crate) fn is_single_controlled(self) -> bool {
        self.control_qubit_count() == 1 && self.target_qubit_count() == 1
    }

    /// Get the number of classical bits used by this type of gate.
    pub(crate) fn bit_count(self) -> usize {
        use GateType::*;

        match self {
            Measure => 1,
            _ => 0,
        }
    }
}
