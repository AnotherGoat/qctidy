use getset::{CopyGetters, Getters};
use inew::New;

/// Metadata associated with a graph simplification rule.
#[derive(Debug, Clone, Copy, Getters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct RuleMetadata {
    /// The unique identifier for this rule.
    #[get = "pub"]
    id: RuleId,
    /// A human-readable description of what this rule does.
    #[get = "pub"]
    description: &'static str,
    /// The group that this rule can be classified into.
    #[get_copy = "pub"]
    group: RuleGroup,
    #[get_copy = "pub"]
    priority: u32,
}

/// Unique identifier for a simplification rule.
pub type RuleId = &'static str;

/// Fine-grained classification for simplification rules.
///
/// Groups describe the specific family or identity class that a rule belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleGroup {
    /// Rules that normalize equivalent circuit representations into a canonical form.
    Canonicalization,
    /// Rules that remove trivial but redundant repeated reversible gate operations.
    ///
    /// All the patterns in this groups should be obvious at a glance, but harder to find in larger circuits.
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
    /// Rules that don't fit cleanly into any other group.
    ///
    /// Avoided whenever possible for built-in rules.
    Miscellaneous,
}
