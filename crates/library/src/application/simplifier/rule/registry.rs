use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use crate::{RuleId, SimplificationRule, application::simplifier::rule::default};

pub(crate) static DEFAULT_RULE_REGISTRY: LazyLock<RuleRegistry> = LazyLock::new(|| {
    let mut registry = RuleRegistry::new();
    default::register_all(&mut registry);
    registry
});

/// A registry of unique graph simplification rules.
#[derive(Default, Debug, Clone)]
pub struct RuleRegistry {
    rules: HashMap<RuleId, Arc<dyn SimplificationRule>>,
}

impl RuleRegistry {
    /// Create a new empty `RuleRegistry`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new rule to the registry.
    ///
    /// Automatically replaces any existing rule with the same ID.
    pub fn register(&mut self, rule: Arc<dyn SimplificationRule>) {
        self.rules.insert(rule.metadata().id(), rule);
    }

    /// Get a rule by its ID.
    ///
    /// Returns `None` if the rule does not exist.
    pub fn get(&self, id: &str) -> Option<Arc<dyn SimplificationRule>> {
        self.rules.get(id).cloned()
    }

    /// Iterate over all the rules in the registry, in arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn SimplificationRule>> {
        self.rules.values()
    }
}
