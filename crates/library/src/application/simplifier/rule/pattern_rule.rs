use std::{collections::HashMap, fmt};

use getset::{CopyGetters, Getters};

use crate::{
    GateType, Graph, PatternMatch, RuleBuildError, RuleMetadata, SimplificationRule,
    application::simplifier::{
        matrix_calculator,
        pattern::{anchor::Anchor, cache::GraphCache},
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

        match self {
            Left => write!(f, "LHS"),
            Right => write!(f, "RHS"),
        }
    }
}

/// A simplification rule that matches a pattern and replaces it with another pattern.
///
/// The main type of rule used in this library.
/// Pattern rules have a specific set of requirements:
/// - LHS and RHS must be valid graphs.
/// - LHS and RHS must not contain forbidden gates: identity and measurement.
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
    /// The number of qubits in this rule.
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

    fn find_matches(&self, graph: &Graph) -> Vec<PatternMatch> {
        vec![]
    }

    fn apply(&self, graph: &mut Graph) -> bool {
        false
    }
}

impl PatternRule {
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

        Self::validate_forbidden_gates(&lhs, PatternRuleSide::Left)?;
        Self::validate_forbidden_gates(&rhs, PatternRuleSide::Right)?;

        if lhs.is_empty() {
            return Err(RuleBuildError::EmptyPattern);
        }

        let height = lhs.height();
        let rhs_height = rhs.height();

        if rhs_height > height {
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

        if !matrix_calculator::are_graphs_equivalent(&lhs, &rhs) {
            return Err(RuleBuildError::NonEquivalentGraphs);
        }

        let lhs_cache = GraphCache::from_graph(&lhs);
        let anchor = Self::extract_anchor(&lhs)?;

        Ok(Self {
            metadata,
            lhs,
            rhs,
            height,
            lhs_cache,
            anchor,
        })
    }

    fn validate_forbidden_gates(
        graph: &Graph,
        side: PatternRuleSide,
    ) -> Result<(), RuleBuildError> {
        use GateType::*;

        for node in graph.iter_nodes() {
            match node.r#type() {
                ID | Measure => {
                    return Err(RuleBuildError::ContainsForbiddenGate {
                        side,
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
                Some(current_row) => {
                    if row > current_row + 1 {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn extract_anchor(lhs: &Graph) -> Result<Anchor, RuleBuildError> {
        let mut frequencies: HashMap<GateType, usize> = HashMap::new();

        for node in lhs.iter_nodes() {
            *frequencies.entry(node.r#type()).or_insert(0) += 1;
        }

        let mut best = None;

        for node in lhs.iter_nodes_ordered_by_column() {
            let position = node.position();
            let gate_type = node.r#type();

            let candidate = Anchor::new(position.row(), position.column(), gate_type);
            let arity = gate_type.qubit_count();
            let frequency = *frequencies
                .get(&gate_type)
                .ok_or(RuleBuildError::InvalidAnchor)?;

            match &best {
                None => {
                    best = Some((candidate, arity, frequency));
                }
                Some((_, best_arity, best_frequency)) => {
                    let is_better_candidate = arity > *best_arity
                        || (arity == *best_arity && frequency < *best_frequency);

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
