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
    /// Rules that remove trivial but redundant repeated reversible gate operations.
    ///
    /// Examples:
    /// - `H H -> ()`
    /// - `CX CX -> ()`
    Redundancy,
    /// Rules that merge adjacent angle rotations.
    ///
    /// Examples:
    /// - `RX(a) RX(b) -> RX(a+b)`
    AngleMerging,
    /// Rules that normalize equivalent circuit representations into a canonical form.
    Normalization,
    /// Rules that don't fit cleanly into any other group.
    ///
    /// Avoided whenever possible for built-in rules.
    Miscellaneous,
}
