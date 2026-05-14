/// Contains unitary matrix calculations for quantum graphs.
pub(crate) mod matrix_calculator;
pub(crate) mod pattern;
pub(crate) mod rule;

#[cfg(test)]
mod matrix_calculator_tests;

use std::sync::Arc;

use crate::{Graph, PatternRule, RuleConfiguration, RuleLevel, RuleRegistry, SimplificationRule};

/// A simplifier for quantum graphs.
pub struct Simplifier {
    rules: Vec<Arc<dyn SimplificationRule>>,
}

impl Simplifier {
    pub fn new(
        registry: &RuleRegistry,
        custom_rules: Vec<PatternRule>,
        configuration: &RuleConfiguration,
    ) -> Self {
        let mut rules: Vec<Arc<dyn SimplificationRule>> = vec![];

        for rule in registry.iter() {
            let level = configuration.level(rule.metadata().id());

            if level != RuleLevel::Off {
                rules.push(rule.clone());
            }
        }

        for custom_rule in custom_rules {
            rules.push(Arc::new(custom_rule));
        }

        Self { rules }
    }

    /// Apply the simplification algorithm to the given graph.
    ///
    /// Stops early if no changes are detected between two iterations.
    pub fn simplify(&self, graph: &mut Graph, max_iterations: u32) {
        self.simplify_internal(graph, &self.rules, max_iterations);
    }

    pub fn simplify_with_rules(
        &self,
        graph: &mut Graph,
        extra_rules: Vec<PatternRule>,
        max_iterations: u32,
    ) {
        let mut rules = self.rules.clone();

        for extra_rule in extra_rules {
            rules.push(Arc::new(extra_rule));
        }

        self.simplify_internal(graph, &rules, max_iterations);
    }

    fn simplify_internal(
        &self,
        graph: &mut Graph,
        rules: &Vec<Arc<dyn SimplificationRule>>,
        max_iterations: u32,
    ) {
        for _ in 0..max_iterations {
            let mut changed = false;

            for rule in rules {
                if rule.apply(graph) {
                    changed = true;
                }
            }

            if !changed {
                break;
            }
        }
    }
}

pub const fn simplify(graph: Graph, _iterations: u32) -> Graph {
    graph
}

pub fn simplify_with_rules(
    graph: Graph,
    _rules: Vec<Arc<dyn SimplificationRule>>,
    _iterations: u32,
) -> Graph {
    graph
}
