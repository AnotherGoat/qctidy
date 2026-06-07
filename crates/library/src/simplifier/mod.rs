/// Contains unitary matrix calculations for quantum graphs.
pub(crate) mod matrix_calculator;
pub(crate) mod pattern;
pub(crate) mod rule;

#[cfg(test)]
mod matrix_calculator_tests;
#[cfg(test)]
mod simplification_tests;
#[cfg(test)]
pub(crate) mod simplifier_mother;

use std::sync::Arc;

use crate::{
    Graph, PatternRule, RuleConfiguration, RuleLevel, RuleRegistry, SimplificationRule,
    simplifier::rule::registry::DEFAULT_RULE_REGISTRY,
};

/// A simplifier for quantum graphs.
#[derive(Debug)]
pub struct Simplifier {
    rules: Vec<Arc<dyn SimplificationRule>>,
}

impl Simplifier {
    #[must_use]
    pub fn new(
        registry: &RuleRegistry,
        custom_rules: Vec<PatternRule>,
        configuration: &RuleConfiguration,
    ) -> Self {
        let mut rules: Vec<Arc<dyn SimplificationRule>> = vec![];

        for rule in registry.iter() {
            let level = configuration.level(rule.metadata().id());

            if level != RuleLevel::Off {
                rules.push(Arc::clone(rule));
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
        simplify_internal(graph, &self.rules, max_iterations);
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

        simplify_internal(graph, &rules, max_iterations);
    }
}

pub fn simplify(mut graph: Graph, iterations: u32) -> Graph {
    let simplifier = Simplifier::new(
        &DEFAULT_RULE_REGISTRY,
        vec![],
        &RuleConfiguration::new(RuleLevel::Apply),
    );

    simplifier.simplify(&mut graph, iterations);
    graph
}

pub fn simplify_with_rules(
    graph: Graph,
    _rules: Vec<Arc<dyn SimplificationRule>>,
    _iterations: u32,
) -> Graph {
    graph
}

fn simplify_internal(
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
