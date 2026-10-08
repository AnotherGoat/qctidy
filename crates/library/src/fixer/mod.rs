/// Contains unitary matrix calculations for quantum graphs.
pub(crate) mod matrix_calculator;
pub(crate) mod pattern;
pub(crate) mod rule;

#[cfg(test)]
mod fix_tests;
#[cfg(test)]
pub(crate) mod fixer_mother;
#[cfg(test)]
mod matrix_calculator_tests;

use std::sync::Arc;

use getset::{CopyGetters, Getters};
use newgen::New;

use crate::{
    Graph, PatternRule, Position, Rule, RuleConfiguration, RuleMetadata, RuleRegistry,
    RuleSeverity,
    fixer::{
        pattern::cache::{GateTypeBitset, GraphCache},
        rule::registry::DEFAULT_RULE_REGISTRY,
    },
};

/// A diagnostic describing a fixable pattern in a graph.
#[derive(Debug, Clone, Getters, CopyGetters, New)]
#[new(pub)]
#[must_use]
pub struct Diagnostic {
    /// Metadata of the rule that was detected.
    #[get = "pub"]
    metadata: RuleMetadata,
    /// Severity reported for the detected match.
    #[get_copy = "pub"]
    severity: RuleSeverity,
    /// Positions affected by the detected match, sorted by row and then column.
    #[get = "pub"]
    positions: Vec<Position>,
}

/// A fixer for quantum graphs.
#[derive(Debug)]
pub struct Fixer {
    rules: Vec<Arc<dyn Rule>>,
    configuration: RuleConfiguration,
}

impl Fixer {
    #[must_use]
    pub fn new(
        registry: &RuleRegistry,
        custom_rules: Vec<PatternRule>,
        configuration: &RuleConfiguration,
    ) -> Self {
        let mut rules: Vec<Arc<dyn Rule>> = vec![];

        for rule in registry.iter() {
            let severity = configuration.severity(rule.metadata());

            if severity != RuleSeverity::Off {
                rules.push(Arc::clone(rule));
            }
        }

        for custom_rule in custom_rules {
            rules.push(Arc::new(custom_rule));
        }

        Self {
            rules,
            configuration: configuration.clone(),
        }
    }

    /// Apply the fix algorithm to the given graph.
    ///
    /// Stops early if no changes are detected between two iterations.
    pub fn fix(&self, graph: &mut Graph, max_iterations: u32) {
        fix_internal(graph, &self.rules, max_iterations);
    }

    /// Find all matches for every enabled rule, without modifying the graph.
    ///
    /// Diagnostics are ordered by rule code, then by the positions they affect.
    #[must_use]
    pub fn detect(&self, graph: &Graph) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let graph_cache = GraphCache::from_graph(graph);
        let gate_types = collect_gate_types(graph);

        for rule in &self.rules {
            if !is_applicable(rule.as_ref(), graph, gate_types) {
                continue;
            }

            let matches = rule.as_any().downcast_ref::<PatternRule>().map_or_else(
                || rule.detect(graph),
                |pattern_rule| pattern_rule.detect_with_cache(graph, &graph_cache),
            );

            for matched_positions in matches {
                let mut positions: Vec<Position> = matched_positions.into_iter().collect();

                positions.sort_unstable();

                diagnostics.push(Diagnostic::new(
                    *rule.metadata(),
                    self.configuration.severity(rule.metadata()),
                    positions,
                ));
            }
        }

        diagnostics.sort_by(|left, right| {
            left.metadata
                .code()
                .cmp(right.metadata.code())
                .then_with(|| left.positions.cmp(&right.positions))
        });

        diagnostics
    }
}

pub fn fix(mut graph: Graph, iterations: u32) -> Graph {
    let fixer = Fixer::new(
        &DEFAULT_RULE_REGISTRY,
        vec![],
        &RuleConfiguration::new(RuleSeverity::Warn),
    );

    fixer.fix(&mut graph, iterations);
    graph
}

/// Find all fixable patterns in a graph, without modifying it.
///
/// Only the rules that are not disabled in the given configuration are checked.
#[must_use]
pub fn detect(graph: &Graph, configuration: &RuleConfiguration) -> Vec<Diagnostic> {
    Fixer::new(&DEFAULT_RULE_REGISTRY, vec![], configuration).detect(graph)
}

/// List the metadata of every built-in fix rule, ordered by code.
#[must_use]
pub fn default_rules() -> Vec<RuleMetadata> {
    let mut rules: Vec<RuleMetadata> = DEFAULT_RULE_REGISTRY
        .iter()
        .map(|rule| *rule.metadata())
        .collect();

    rules.sort_by_key(|metadata| *metadata.code());

    rules
}

fn fix_internal(graph: &mut Graph, rules: &[Arc<dyn Rule>], max_iterations: u32) {
    for _ in 0..max_iterations {
        let mut changed = false;
        let mut gate_types = collect_gate_types(graph);

        for rule in rules {
            if !is_applicable(rule.as_ref(), graph, gate_types) {
                continue;
            }

            if rule.apply(graph) {
                changed = true;
                gate_types = collect_gate_types(graph);
            }
        }

        if !changed {
            break;
        }
    }
}

fn collect_gate_types(graph: &Graph) -> GateTypeBitset {
    // The set of gate types present in the graph.
    let mut gate_types = GateTypeBitset::new();

    for node in graph.iter_nodes() {
        gate_types.insert(node.r#type());
    }

    gate_types
}

fn is_applicable(rule: &dyn Rule, graph: &Graph, gate_types: GateTypeBitset) -> bool {
    // Whether a rule can possibly match the graph, based on its quick checks.
    //
    // Skipping impossible rules avoids running their matcher over the whole graph.
    rule.minimum_height() <= graph.height()
        && rule.minimum_width() <= graph.width()
        && rule
            .required_gate_types()
            .iter()
            .all(|gate_type| gate_types.contains(*gate_type))
}
