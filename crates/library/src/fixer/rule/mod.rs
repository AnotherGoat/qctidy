pub(crate) mod canonicalization_rule;
pub(crate) mod configuration;
pub(crate) mod default;
pub(crate) mod metadata;
pub(crate) mod pattern_rule;
pub(crate) mod registry;

#[doc(inline)]
pub use canonicalization_rule::CanonicalizationRule;

#[cfg(test)]
pub(crate) mod canonicalization_rule_tests;
#[cfg(test)]
pub(crate) mod pattern_rule_tests;
#[cfg(test)]
pub(crate) mod registry_tests;

use std::fmt::Debug;
use std::{any::Any, collections::HashSet};

use thiserror::Error;

use crate::{GateType, Graph, GraphError, PatternRuleSide, Position, RuleMetadata};

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
    #[error("RHS qubit count of {rhs_qubit_count} must match LHS qubit count of {lhs_qubit_count}")]
    InvalidQubitCount {
        lhs_qubit_count: usize,
        rhs_qubit_count: usize,
    },
    #[error("LHS graph contains gaps between rows")]
    NonContiguousPattern,
    #[error("LHS and RHS are identical")]
    IdenticalGraphs,
    #[error(
        "RHS must contain fewer operations than LHS (RHS: {rhs_operation_count}, LHS: {lhs_operation_count})"
    )]
    NonCompactingRule {
        lhs_operation_count: usize,
        rhs_operation_count: usize,
    },
    #[error("LHS and RHS are not equivalent")]
    NonEquivalentGraphs,
    #[error("invalid anchor")]
    InvalidAnchor,
}

/// A rule that can be applied to a `Graph` to reduce its size.
pub trait Rule: Debug + Send + Sync {
    /// Metadata associated with the rule.
    fn metadata(&self) -> &RuleMetadata;

    /// Return all gate types that must be present in the graph to apply the rule.
    ///
    /// This is useful to quickly skip rules that cannot be applied to a given graph.
    /// If the value is empty, it can be applied to any graph.
    fn required_gate_types(&self) -> &HashSet<GateType>;

    /// Return the minimum number of time steps required to apply this rule.
    ///
    /// This is useful to quickly skip rules that cannot be applied to a given graph.
    /// If the value is 1 or 0, it can be applied to any graph.
    fn minimum_time_step_count(&self) -> usize;

    /// Return the minimum number of qubits required to apply this rule.
    ///
    /// This is useful to quickly skip rules that cannot be applied to a given graph.
    /// If the value is 1 or 0, it can be applied to any graph.
    fn minimum_qubit_count(&self) -> usize;

    /// Return all matches of the rule in the given graph, where each match is a set of affected positions.
    fn detect(&self, graph: &Graph) -> Vec<HashSet<Position>>;

    /// Apply the rule once, then return true if a change occurred.
    fn apply(&self, graph: &mut Graph) -> bool;

    /// Downcast to `Any` for type-safe downcasting.
    fn as_any(&self) -> &dyn Any;
}
