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

use std::collections::HashSet;
use std::sync::{Arc, LazyLock};

use getset::{CopyGetters, Getters};
use newgen::New;

use crate::{
    CanonicalizationRule, GateOperation, Graph, PatternMatch, PatternRule, Position, Rule,
    RuleConfiguration, RuleMetadata, RuleRegistry, RuleSeverity,
    fixer::{
        pattern::{
            cache::{GateTypeBitset, GraphCache},
            replacer,
        },
        rule::registry::DEFAULT_RULE_REGISTRY,
    },
};

/// A diagnostic describing a fixable match in a graph.
///
/// Follows the Language Server Protocol (LSP) notion of a diagnostic: a severity and
/// the positions affected by the match, produced by the rule that was detected.
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
    /// Positions affected by the detected match, sorted by qubit and then time step.
    #[get = "pub"]
    positions: Vec<Position>,
    /// The matched pattern, when the rule is a pattern rule.
    #[get = "pub"]
    pattern_match: Option<PatternMatch>,
    /// The operations that replace the match, when the rule can compute them per match.
    #[get = "pub"]
    replacement: Option<Vec<GateOperation>>,
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

            let severity = self.configuration.severity(rule.metadata());

            if let Some(pattern_rule) = rule.as_any().downcast_ref::<PatternRule>() {
                for pattern_match in pattern_rule.find_matches_with_cache(graph, &graph_cache) {
                    let mut positions: Vec<Position> = pattern_match.covered_positions().clone();

                    positions.sort_unstable();

                    diagnostics.push(Diagnostic::new(
                        *rule.metadata(),
                        severity,
                        positions,
                        Some(pattern_match),
                        None,
                    ));
                }
            } else {
                for matched_positions in rule.detect(graph) {
                    let replacement = rule.fix_operations(graph, &matched_positions);
                    let mut positions: Vec<Position> = matched_positions.into_iter().collect();

                    positions.sort_unstable();

                    diagnostics.push(Diagnostic::new(
                        *rule.metadata(),
                        severity,
                        positions,
                        None,
                        replacement,
                    ));
                }
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

    /// Apply the fix that produced the given diagnostic, at exactly that match.
    ///
    /// Returns the operations that replace the matched pattern, in source order,
    /// or `None` when the diagnostic's rule has no applicable match (for example,
    /// canonicalization rules).
    pub fn apply(&self, graph: &mut Graph, diagnostic: &Diagnostic) -> Option<Vec<GateOperation>> {
        let rule = self
            .rules
            .iter()
            .find(|rule| rule.metadata().code() == diagnostic.metadata().code())?;

        let pattern_rule = rule.as_any().downcast_ref::<PatternRule>()?;

        let positions: HashSet<Position> = diagnostic.positions().iter().copied().collect();
        let pattern_match = pattern_rule.find_match(graph, &positions)?;
        let replacement_operations = replacer::replacement_operations(pattern_rule, &pattern_match);

        replacer::apply_match(graph, pattern_rule, &pattern_match);

        Some(replacement_operations)
    }

    /// Compute the operations that replace the matched pattern, without modifying the graph.
    #[must_use]
    pub fn replacement(&self, diagnostic: &Diagnostic) -> Option<Vec<GateOperation>> {
        let rule = self
            .rules
            .iter()
            .find(|rule| rule.metadata().code() == diagnostic.metadata().code())?;

        if let Some(pattern_rule) = rule.as_any().downcast_ref::<PatternRule>() {
            let pattern_match = diagnostic.pattern_match().as_ref()?;

            return Some(replacer::replacement_operations(
                pattern_rule,
                pattern_match,
            ));
        }

        diagnostic.replacement().clone()
    }
}

/// Fixer with every default rule enabled, built once and shared by the free functions below.
static DEFAULT_FIXER: LazyLock<Fixer> = LazyLock::new(|| {
    Fixer::new(
        &DEFAULT_RULE_REGISTRY,
        vec![],
        &RuleConfiguration::new(RuleSeverity::Warn),
    )
});

pub fn fix(mut graph: Graph, iterations: u32) -> Graph {
    DEFAULT_FIXER.fix(&mut graph, iterations);
    graph
}

/// Find all fixable patterns in a graph, without modifying it.
///
/// Only the rules that are not disabled in the given configuration are checked.
#[must_use]
pub fn detect(graph: &Graph, configuration: &RuleConfiguration) -> Vec<Diagnostic> {
    Fixer::new(&DEFAULT_RULE_REGISTRY, vec![], configuration).detect(graph)
}

/// Apply the fix that produced the given diagnostic to `graph`.
///
/// Returns the operations that replace the matched pattern, or `None` when the
/// diagnostic's rule has no applicable match.
pub fn apply(graph: &mut Graph, diagnostic: &Diagnostic) -> Option<Vec<GateOperation>> {
    DEFAULT_FIXER.apply(graph, diagnostic)
}

/// Compute the operations that replace the matched pattern described by the
/// diagnostic, without modifying `graph`.
#[must_use]
pub fn replacement(diagnostic: &Diagnostic) -> Option<Vec<GateOperation>> {
    DEFAULT_FIXER.replacement(diagnostic)
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
    if rule.minimum_qubit_count() > graph.qubit_count()
        || rule.minimum_time_step_count() > graph.time_step_count()
    {
        return false;
    }

    let required = rule.required_gate_types();

    if required.is_empty() {
        return true;
    }

    // Pattern rules need every gate type of their pattern. Canonicalization rules
    // list the gate types they can act on, so any of them is enough.
    if rule.as_any().is::<CanonicalizationRule>() {
        required
            .iter()
            .any(|gate_type| gate_types.contains(*gate_type))
    } else {
        required
            .iter()
            .all(|gate_type| gate_types.contains(*gate_type))
    }
}
