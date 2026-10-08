use std::fmt;

use getset::{CopyGetters, Getters};
use newgen::New;

/// Metadata associated with a graph fix rule.
#[derive(Debug, Clone, Copy, Getters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct RuleMetadata {
    /// The short, stable code for this rule, such as `R001`.
    #[get = "pub"]
    code: RuleCode,
    /// The descriptive name of this rule.
    #[get = "pub"]
    name: &'static str,
    /// A human-readable description of what this rule does.
    #[get = "pub"]
    description: &'static str,
    /// The category that this rule can be classified into.
    #[get_copy = "pub"]
    category: RuleCategory,
    #[get_copy = "pub"]
    priority: u32,
}

/// Short, stable identifier for a fix rule.
pub type RuleCode = &'static str;

/// Fine-grained classification for fix rules.
///
/// Categories describe the specific family or identity class that a rule belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleCategory {
    /// Rules that normalize equivalent circuit representations into a canonical form.
    Canonicalization,
    /// Rules that remove trivial but redundant repeated reversible gate operations.
    ///
    /// All the patterns in this category should be obvious at a glance, but harder to find in larger circuits.
    /// Examples:
    /// - `H H => ()`
    /// - `CX CX => ()`
    Redundancy,
    /// Rulest that reduce consecutive phase gates (Z, S, T) into smaller sets.
    ///
    /// Examples:
    /// - `S S => Z`
    /// - `T T => S`
    PhaseCompaction,
    /// Rules that change the basis of a set of operations, usually flipping the X-space and Z-space.
    ///
    /// Examples:
    /// - `H X H => Z`
    /// - `H Z H => X`
    BasisChange,
    /// Rules that reverse control and target qubits.
    ControlReversal,
    PauliPropagation,
    /// Rules that replace common gate decompositions into the gate they represent.
    GateSynthesis,
    /// Rules that reduce the number of CX gates found in a circuit.
    CxReduction,
    /// Rules that merge adjacent angle rotations.
    ///
    /// Examples:
    /// - `RX(a) RX(b) => RX(a+b)`
    AngleMerging,
}

impl fmt::Display for RuleCategory {
    /// Obtain the name of this rule category as a lowercase, hyphenated string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RuleCategory::*;

        let name = match *self {
            Canonicalization => "canonicalization",
            Redundancy => "redundancy",
            PhaseCompaction => "phase-compaction",
            BasisChange => "basis-change",
            ControlReversal => "control-reversal",
            PauliPropagation => "pauli-propagation",
            GateSynthesis => "gate-synthesis",
            CxReduction => "cx-reduction",
            AngleMerging => "angle-merging",
        };

        write!(f, "{name}")
    }
}

impl RuleCategory {
    /// The single-letter code for this category, used to select all its rules.
    #[must_use]
    pub const fn code(self) -> &'static str {
        use RuleCategory::*;

        match self {
            Canonicalization => "C",
            Redundancy => "R",
            PhaseCompaction => "P",
            BasisChange => "B",
            ControlReversal => "V",
            PauliPropagation => "Q",
            GateSynthesis => "G",
            CxReduction => "X",
            AngleMerging => "A",
        }
    }
}
