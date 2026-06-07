use std::{
    any::Any,
    collections::{HashMap, HashSet},
    fmt,
};

use getset::{CopyGetters, Getters};

use crate::{
    GateType, Graph, PatternMatch, Position, RuleBuildError, RuleMetadata, SimplificationRule,
    simplifier::{
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
    gate_count: usize,
    gate_types: HashSet<GateType>,
    node_frequencies: HashMap<GateType, usize>,
}

impl GraphStatistics {
    fn from_graph(graph: &Graph) -> Self {
        let mut node_frequencies: HashMap<GateType, usize> = HashMap::new();

        for node in graph.iter_nodes() {
            *node_frequencies.entry(node.r#type()).or_insert(0) += 1;
        }

        let gate_count = node_frequencies
            .iter()
            .map(|(gate_type, count)| count / gate_type.qubit_count())
            .sum();
        let gate_types = node_frequencies.keys().copied().collect();

        Self {
            gate_count,
            gate_types,
            node_frequencies,
        }
    }
}

/// A simplification rule that matches a pattern and replaces it with another pattern.
///
/// The main type of rule used in this library.
/// Pattern rules have a specific set of requirements:
/// - LHS and RHS must be valid graphs.
/// - LHS and RHS must not contain forbidden gates: measurement.
/// - Additionally, identity is forbidden from the RHS.
/// - LHS cannot be empty.
/// - RHS cannot have more qubits (height) than LHS.
/// - LHS must not have gaps between rows.
/// - LHS and RHS cannot be identical graphs.
/// - LHS and RHS must be equivalent in global phase.
///
/// Anchors are determined by heuristics in this order:
/// - Highest arity (qubit count).
/// - Lowest frequency (appears the least).
/// - Earliest time (column).
/// - Lowest qubit index (row).
#[derive(Debug, Clone, Getters, CopyGetters)]
pub struct PatternRule {
    /// Metadata associated with the rule.
    #[get = "pub"]
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
    /// The width (number of time steps) of the left-hand side of the rule.
    #[get_copy = "pub"]
    width: usize,
    /// The number of qubits covered by this rule.
    #[get_copy = "pub"]
    height: usize,
    /// Cached rows and columns of the left-hand side of the rule.
    #[get = "pub"]
    lhs_cache: GraphCache,
    /// Anchor point for the pattern to match.
    #[get_copy = "pub"]
    anchor: Anchor,
}

impl SimplificationRule for PatternRule {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn required_gate_types(&self) -> &HashSet<GateType> {
        &self.gate_types
    }

    fn minimum_width(&self) -> usize {
        self.width
    }

    fn minimum_height(&self) -> usize {
        self.height
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

        let height = lhs.height();
        let rhs_height = rhs.height();

        if height != rhs_height {
            return Err(RuleBuildError::InvalidHeight {
                lhs_height: height,
                rhs_height,
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

        if rhs_statistics.gate_count >= lhs_statistics.gate_count {
            return Err(RuleBuildError::NonCompactingRule {
                lhs_gate_count: lhs_statistics.gate_count,
                rhs_gate_count: rhs_statistics.gate_count,
            });
        }

        if !matrix_calculator::are_graphs_equivalent(&lhs, &rhs) {
            return Err(RuleBuildError::NonEquivalentGraphs);
        }

        let width = lhs.width();
        let lhs_cache = GraphCache::from_graph(&lhs);
        let anchor = Self::extract_anchor(&lhs, &lhs_statistics)?;
        let gate_types = lhs_statistics.gate_types;

        Ok(Self {
            metadata,
            lhs,
            rhs,
            gate_types,
            width,
            height,
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
        let mut current_row = None;

        for position in lhs.iter_positions_ordered_by_row() {
            let row = position.row();

            match current_row {
                None => current_row = Some(row),
                Some(prev_row) => {
                    if row > prev_row + 1 {
                        return false;
                    }
                    current_row = Some(row);
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

        for node in lhs.iter_nodes_ordered_by_column() {
            let position = node.position();
            let gate_type = node.r#type();

            let candidate = Anchor::new(position, gate_type);
            let arity = gate_type.qubit_count();
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
