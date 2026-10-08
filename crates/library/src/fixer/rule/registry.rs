use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use crate::{Rule, RuleCode, fixer::rule::default};

pub(crate) static DEFAULT_RULE_REGISTRY: LazyLock<RuleRegistry> = LazyLock::new(|| {
    let mut registry = RuleRegistry::new();
    default::register_all(&mut registry);
    registry
});

/// A registry of unique graph fix rules.
#[derive(Default, Debug, Clone)]
pub struct RuleRegistry {
    rules: HashMap<RuleCode, Arc<dyn Rule>>,
}

impl RuleRegistry {
    /// Create a new empty `RuleRegistry`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new rule to the registry.
    ///
    /// Automatically replaces any existing rule with the same code.
    pub fn register(&mut self, rule: Arc<dyn Rule>) {
        self.rules.insert(rule.metadata().code(), rule);
    }

    /// Add multiple rules to the registry at once.
    ///
    /// Automatically replaces any existing rules with the same codes.
    pub fn register_all(&mut self, rules: Vec<Arc<dyn Rule>>) {
        for rule in rules {
            self.register(rule);
        }
    }

    /// Get a rule by its code.
    ///
    /// Returns `None` if the rule does not exist.
    #[must_use]
    pub fn get(&self, code: &str) -> Option<Arc<dyn Rule>> {
        self.rules.get(code).cloned()
    }

    /// Iterate over all the rules in the registry, in arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn Rule>> {
        self.rules.values()
    }
}
