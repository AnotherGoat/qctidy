pub(crate) mod configuration;
pub(crate) mod default;
pub(crate) mod metadata;
pub(crate) mod pattern_rule;
pub(crate) mod registry;

#[cfg(test)]
pub(crate) mod pattern_rule_tests;
#[cfg(test)]
pub(crate) mod registry_tests;

use std::fmt::Debug;

use thiserror::Error;

use crate::{GateType, Graph, GraphError, PatternMatch, PatternRuleSide, RuleMetadata};

#[derive(Debug, Clone, Copy, Error)]
pub enum RuleBuildError {
    #[error("{side} graph is not valid")]
    InvalidSide {
        side: PatternRuleSide,
        error: GraphError,
    },
    #[error("{side} graph contains forbidden gate of type {gate_type}")]
    ContainsForbiddenGate {
        side: PatternRuleSide,
        gate_type: GateType,
    },
    #[error("Pattern LHS cannot be empty")]
    EmptyPattern,
    #[error("RHS height of {rhs_height} cannot exceed LHS height of {lhs_height}")]
    InvalidHeight {
        lhs_height: usize,
        rhs_height: usize,
    },
    #[error("LHS graph contains gaps between rows")]
    NonContiguousPattern,
    #[error("LHS and RHS are identical")]
    IdenticalGraphs,
    #[error("RHS contains more gates ({rhs_gate_count}) than LHS ({lhs_gate_count})")]
    NonCompactingRule {
        lhs_gate_count: usize,
        rhs_gate_count: usize,
    },
    #[error("LHS and RHS are not equivalent")]
    NonEquivalentGraphs,
    #[error("invalid anchor")]
    InvalidAnchor,
}

/// A rule that can be applied to a `Graph` to reduce its size.
pub trait SimplificationRule: Debug + Send + Sync {
    /// Metadata associated with the rule.
    fn metadata(&self) -> &RuleMetadata;

    /// Return all matches of the rule in the given graph.
    fn find_matches(&self, graph: &Graph) -> Vec<PatternMatch>;

    /// Apply the rule once, then return true if a change occurred.
    fn apply(&self, graph: &mut Graph) -> bool;
}
