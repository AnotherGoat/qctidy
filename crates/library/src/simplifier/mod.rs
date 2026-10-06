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

use getset::Getters;
use newgen::New;

use crate::{
    Graph, PatternRule, Position, RuleConfiguration, RuleLevel, RuleMetadata, RuleRegistry,
    SimplificationRule, simplifier::rule::registry::DEFAULT_RULE_REGISTRY,
};

/// A simplification opportunity detected in a graph.
#[derive(Debug, Clone, Getters, New)]
#[new(pub)]
#[must_use]
pub struct Detection {
    /// Metadata of the rule that was detected.
    #[get = "pub"]
    metadata: RuleMetadata,
    /// Positions affected by the detected match, sorted by row and then column.
    #[get = "pub"]
    positions: Vec<Position>,
}

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

    /// Find all matches for every enabled rule, without modifying the graph.
    ///
    /// Detections are ordered by rule ID, then by the positions they affect.
    #[must_use]
    pub fn detect(&self, graph: &Graph) -> Vec<Detection> {
        let mut detections = Vec::new();

        for rule in &self.rules {
            for matched_positions in rule.detect(graph) {
                let mut positions: Vec<Position> = matched_positions.into_iter().collect();

                positions.sort_unstable();

                detections.push(Detection::new(*rule.metadata(), positions));
            }
        }

        detections.sort_by(|left, right| {
            left.metadata
                .id()
                .cmp(right.metadata.id())
                .then_with(|| left.positions.cmp(&right.positions))
        });

        detections
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

/// Find all simplification opportunities in a graph, without modifying it.
///
/// Only the rules that are not disabled in the given configuration are checked.
#[must_use]
pub fn detect(graph: &Graph, configuration: &RuleConfiguration) -> Vec<Detection> {
    Simplifier::new(&DEFAULT_RULE_REGISTRY, vec![], configuration).detect(graph)
}

/// List the metadata of every built-in simplification rule, ordered by ID.
#[must_use]
pub fn default_rules() -> Vec<RuleMetadata> {
    let mut rules: Vec<RuleMetadata> = DEFAULT_RULE_REGISTRY
        .iter()
        .map(|rule| *rule.metadata())
        .collect();

    rules.sort_by_key(|metadata| *metadata.id());

    rules
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
