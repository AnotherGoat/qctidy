use std::{
    any::Any,
    collections::{HashMap, HashSet},
    fmt,
};

use getset::{CopyGetters, Getters};

use crate::{
    GateType, Graph, PatternMatch, Position, Rule, RuleBuildError, RuleMetadata,
    fixer::{
        matrix_calculator,
        pattern::{anchor::Anchor, cache::GraphCache, matcher, replacer},
    },
};

/// The side of the rule that is matched.
///
/// This is only used when the rule is not valid and the side that fails must be specified.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternRuleSide {
    /// The left-hand side of the rule.
    Left,
    /// The right-hand side of the rule.
    Right,
}

impl fmt::Display for PatternRuleSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PatternRuleSide::*;

        match *self {
            Left => write!(f, "LHS"),
            Right => write!(f, "RHS"),
        }
    }
}

#[derive(Debug, Clone)]
struct GraphStatistics {
    operation_count: usize,
    gate_types: HashSet<GateType>,
    node_frequencies: HashMap<GateType, usize>,
}

impl GraphStatistics {
    fn from_graph(graph: &Graph) -> Self {
        let mut node_frequencies: HashMap<GateType, usize> = HashMap::new();

        for node in graph.iter_nodes() {
            *node_frequencies.entry(node.r#type()).or_insert(0) += 1;
        }

        let operation_count = node_frequencies
            .iter()
            .map(|(gate_type, count)| count / gate_type.arity())
            .sum();
        let gate_types = node_frequencies.keys().copied().collect();

        Self {
            operation_count,
            gate_types,
            node_frequencies,
        }
    }
}

/// A fix rule that matches a pattern and replaces it with another pattern.
///
/// The main type of rule used in this library.
/// Pattern rules have a specific set of requirements:
/// - LHS and RHS must be valid graphs.
/// - LHS and RHS must not contain forbidden gates: measurement.
/// - Additionally, identity is forbidden from the RHS.
/// - LHS cannot be empty.
/// - RHS cannot have more qubits than LHS.
/// - LHS must not have gaps between rows.
/// - LHS and RHS cannot be identical graphs.
/// - LHS and RHS must be equivalent in global phase.
///
/// Anchors are determined by heuristics in this order:
/// - Highest arity (qubit count).
/// - Lowest frequency (appears the least).
/// - Earliest time step.
/// - Lowest qubit index.
#[derive(Debug, Clone, Getters, CopyGetters)]
pub struct PatternRule {
    /// Metadata associated with the rule.
    metadata: RuleMetadata,
    /// The left-hand side of the rule, which is the pattern to match.
    #[get = "pub"]
    lhs: Graph,
    /// The right-hand side of the rule, which is the replacement for the matched pattern.
    #[get = "pub"]
    rhs: Graph,
    /// The gate types required to apply the rule.
    #[get = "pub"]
    gate_types: HashSet<GateType>,
    /// The number of time steps in the left-hand side of the rule.
    #[get_copy = "pub"]
    time_step_count: usize,
    /// The number of qubits covered by this rule.
    #[get_copy = "pub"]
    qubit_count: usize,
    /// Cached rows and columns of the left-hand side of the rule.
    #[get = "pub"]
    lhs_cache: GraphCache,
    /// Anchor point for the pattern to match.
    #[get_copy = "pub"]
    anchor: Anchor,
}

impl Rule for PatternRule {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn required_gate_types(&self) -> &HashSet<GateType> {
        &self.gate_types
    }

    fn minimum_time_step_count(&self) -> usize {
        self.time_step_count
    }

    fn minimum_qubit_count(&self) -> usize {
        self.qubit_count
    }

    fn detect(&self, graph: &Graph) -> Vec<HashSet<Position>> {
        self.find_matches(graph)
            .into_iter()
            .map(|pattern_match| pattern_match.covered_positions().iter().copied().collect())
            .collect()
    }

    fn apply(&self, graph: &mut Graph) -> bool {
        let matches = self.find_matches(graph);

        if matches.is_empty() {
            return false;
        }

        for pattern_match in matches {
            replacer::apply_match(graph, self, &pattern_match);
        }

        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PatternRule {
    /// Find all the matches of this rule's pattern in the given graph.
    #[must_use]
    pub fn find_matches(&self, graph: &Graph) -> Vec<PatternMatch> {
        matcher::find_matches(graph, self)
    }

    /// Find the match whose covered positions equal `positions`.
    pub(crate) fn find_match(
        &self,
        graph: &Graph,
        positions: &HashSet<Position>,
    ) -> Option<PatternMatch> {
        self.find_matches(graph).into_iter().find(|pattern_match| {
            pattern_match
                .covered_positions()
                .iter()
                .copied()
                .collect::<HashSet<_>>()
                == *positions
        })
    }

    /// Apply this rule at the match whose covered positions equal `positions`.
    ///
    /// Returns `false` when no match covers exactly those positions.
    pub fn apply_at(&self, graph: &mut Graph, positions: &HashSet<Position>) -> bool {
        let Some(pattern_match) = self.find_match(graph, positions) else {
            return false;
        };

        replacer::apply_match(graph, self, &pattern_match);

        true
    }

    /// Find all the matches of this rule's pattern, reusing a graph cache.
    ///
    /// The cache must have been built from the same, unchanged graph.
    pub(crate) fn find_matches_with_cache(
        &self,
        graph: &Graph,
        graph_cache: &GraphCache,
    ) -> Vec<PatternMatch> {
        matcher::find_matches_with_cache(graph, self, graph_cache)
    }

    /// Detect all the matches of this rule's pattern, reusing a graph cache.
    pub(crate) fn detect_with_cache(
        &self,
        graph: &Graph,
        graph_cache: &GraphCache,
    ) -> Vec<HashSet<Position>> {
        self.find_matches_with_cache(graph, graph_cache)
            .into_iter()
            .map(|pattern_match| pattern_match.covered_positions().iter().copied().collect())
            .collect()
    }

    pub fn new(metadata: RuleMetadata, lhs: Graph, rhs: Graph) -> Result<Self, RuleBuildError> {
        lhs.validate()
            .map_err(|error| RuleBuildError::InvalidSide {
                side: PatternRuleSide::Left,
                error,
            })?;

        rhs.validate()
            .map_err(|error| RuleBuildError::InvalidSide {
                side: PatternRuleSide::Right,
                error,
            })?;

        Self::validate_forbidden_lhs_gates(&lhs)?;
        Self::validate_forbidden_rhs_gates(&rhs)?;

        if lhs.is_empty() {
            return Err(RuleBuildError::EmptyPattern);
        }

        let qubit_count = lhs.qubit_count();
        let rhs_qubit_count = rhs.qubit_count();

        if qubit_count != rhs_qubit_count {
            return Err(RuleBuildError::InvalidQubitCount {
                lhs_qubit_count: qubit_count,
                rhs_qubit_count,
            });
        }

        if !Self::is_contiguous(&lhs) {
            return Err(RuleBuildError::NonContiguousPattern);
        }

        if lhs == rhs {
            return Err(RuleBuildError::IdenticalGraphs);
        }

        let lhs_statistics = GraphStatistics::from_graph(&lhs);
        let rhs_statistics = GraphStatistics::from_graph(&rhs);

        if rhs_statistics.operation_count >= lhs_statistics.operation_count {
            return Err(RuleBuildError::NonCompactingRule {
                lhs_operation_count: lhs_statistics.operation_count,
                rhs_operation_count: rhs_statistics.operation_count,
            });
        }

        if !matrix_calculator::are_graphs_equivalent(&lhs, &rhs) {
            return Err(RuleBuildError::NonEquivalentGraphs);
        }

        let time_step_count = lhs.time_step_count();
        let lhs_cache = GraphCache::from_graph(&lhs);
        let anchor = Self::extract_anchor(&lhs, &lhs_statistics)?;
        let gate_types = lhs_statistics.gate_types;

        Ok(Self {
            metadata,
            lhs,
            rhs,
            gate_types,
            time_step_count,
            qubit_count,
            lhs_cache,
            anchor,
        })
    }

    fn validate_forbidden_lhs_gates(graph: &Graph) -> Result<(), RuleBuildError> {
        for node in graph.iter_nodes() {
            if node.r#type() == GateType::Measure {
                return Err(RuleBuildError::ContainsForbiddenGate {
                    side: PatternRuleSide::Left,
                    gate_type: node.r#type(),
                });
            }
        }

        Ok(())
    }

    #[expect(clippy::wildcard_enum_match_arm)]
    fn validate_forbidden_rhs_gates(graph: &Graph) -> Result<(), RuleBuildError> {
        use GateType::*;

        for node in graph.iter_nodes() {
            match node.r#type() {
                ID | Measure => {
                    return Err(RuleBuildError::ContainsForbiddenGate {
                        side: PatternRuleSide::Right,
                        gate_type: node.r#type(),
                    });
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn is_contiguous(lhs: &Graph) -> bool {
        let mut current_qubit = None;

        for position in lhs.iter_positions_ordered_by_qubit() {
            let qubit = position.qubit();

            match current_qubit {
                None => current_qubit = Some(qubit),
                Some(prev_qubit) => {
                    if qubit > prev_qubit + 1 {
                        return false;
                    }
                    current_qubit = Some(qubit);
                }
            }
        }

        true
    }

    fn extract_anchor(
        lhs: &Graph,
        lhs_statistics: &GraphStatistics,
    ) -> Result<Anchor, RuleBuildError> {
        let node_frequencies = &lhs_statistics.node_frequencies;
        let mut best = None;

        for node in lhs.iter_nodes_ordered_by_time_step() {
            let position = node.position();
            let gate_type = node.r#type();

            let candidate = Anchor::new(position, gate_type);
            let arity = gate_type.arity();
            let frequency = *node_frequencies
                .get(&gate_type)
                .ok_or(RuleBuildError::InvalidAnchor)?;

            match best {
                None => {
                    best = Some((candidate, arity, frequency));
                }
                Some((_, best_arity, best_frequency)) => {
                    let is_better_candidate =
                        arity > best_arity || (arity == best_arity && frequency < best_frequency);

                    if is_better_candidate {
                        best = Some((candidate, arity, frequency));
                    }
                }
            }
        }

        best.map(|(anchor, _, _)| anchor)
            .ok_or(RuleBuildError::InvalidAnchor)
    }
}
