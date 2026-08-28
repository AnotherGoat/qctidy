use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use newgen::New;

use crate::RuleId;

/// Configuration for graph simplification rules.
#[derive(Debug, Clone, Getters, CopyGetters, New)]
#[new(pub)]
pub struct RuleConfiguration {
    /// The default lint level for all the rules.
    #[get_copy = "pub"]
    default: RuleLevel,
    /// Overrides for specific rules.
    #[get = "pub"]
    #[new(default)]
    rules: HashMap<RuleId, RuleLevel>,
}

impl RuleConfiguration {
    /// Enable the rule with the given ID.
    pub fn enable(&mut self, id: RuleId) {
        self.rules.insert(id, RuleLevel::Apply);
    }

    /// Detect the rule with the given ID.
    pub fn detect(&mut self, id: RuleId) {
        self.rules.insert(id, RuleLevel::Detect);
    }

    /// Disable the rule with the given ID.
    pub fn disable(&mut self, id: RuleId) {
        self.rules.insert(id, RuleLevel::Off);
    }

    /// Get the current lint level for the rule with the given ID.
    #[must_use]
    pub fn level(&self, id: RuleId) -> RuleLevel {
        self.rules.get(id).copied().unwrap_or(self.default)
    }
}

/// Lint level for a graph simplification rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleLevel {
    /// The rule will be skipped.
    Off,
    /// The rule will be detected in the final iteration, without modifying the graph.
    Detect,
    /// The rule will be applied on every iteration, modifying the graph.
    Apply,
}
