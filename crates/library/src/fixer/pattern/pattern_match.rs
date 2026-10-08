use getset::Getters;
use newgen::New;

use crate::{Position, RuleCode};

/// An instance of a pattern found inside a graph.
#[derive(Debug, Clone, Getters, New)]
#[new(pub, const)]
pub struct PatternMatch {
    /// The code of the rule that was matched.
    #[get = "pub"]
    rule_code: RuleCode,
    /// Mapping from pattern to target graph.
    #[get = "pub"]
    mapping: QubitMapping,
    /// Every position from the matched graph that was covered by the pattern.
    #[get = "pub"]
    covered_positions: Vec<Position>,
}

/// A mapping from a qubit's index in the pattern to the qubit's index in the graph.
///
/// Essentially, this is the first "winning" permutation that was found.
#[derive(Debug, Clone)]
pub struct QubitMapping {
    pattern_to_graph: Vec<Option<usize>>,
}

impl QubitMapping {
    /// Create a new empty `QubitMapping` for a pattern of the given height.
    ///
    /// Complexity: O(`pattern_height`).
    pub fn new(pattern_height: usize) -> Self {
        Self {
            pattern_to_graph: vec![None; pattern_height],
        }
    }

    /// Add a mapping from a pattern qubit to a graph qubit.
    ///
    /// Returns `false` if the mapping already exists.
    ///
    /// Complexity: O(`pattern_height`).
    pub fn add_mapping(&mut self, pattern_index: usize, graph_index: usize) -> bool {
        match self.pattern_to_graph[pattern_index] {
            None => {
                if self.pattern_to_graph.contains(&Some(graph_index)) {
                    return false;
                }

                self.pattern_to_graph[pattern_index] = Some(graph_index);
                true
            }
            Some(existing) => existing == graph_index,
        }
    }

    /// Remove the mapping of a pattern qubit.
    ///
    /// Complexity: O(1).
    pub(crate) fn remove_mapping(&mut self, pattern_index: usize) {
        self.pattern_to_graph[pattern_index] = None;
    }

    /// Get the graph row corresponding to a pattern row.
    ///
    /// Complexity: O(1).
    pub fn graph_row(&self, pattern_row: usize) -> Option<usize> {
        self.pattern_to_graph[pattern_row]
    }

    /// Get the pattern row corresponding to a graph row.
    ///
    /// Complexity: O(`pattern_height`).
    pub fn pattern_row(&self, graph_row: usize) -> Option<usize> {
        self.pattern_to_graph
            .iter()
            .position(|&mapped| mapped == Some(graph_row))
    }
}
