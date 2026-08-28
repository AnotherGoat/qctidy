use getset::Getters;
use newgen::New;

use crate::{Position, RuleId};

/// An instance of a pattern found inside a graph.
#[derive(Debug, Clone, Getters, New)]
#[new(pub, const)]
pub struct PatternMatch {
    /// The ID of the rule that was matched.
    #[get = "pub"]
    rule_id: RuleId,
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
    graph_to_pattern: Vec<Option<usize>>,
}

impl QubitMapping {
    /// Create a new empty `QubitMapping` with the specified dimensions.
    pub fn new(pattern_height: usize, graph_height: usize) -> Self {
        Self {
            pattern_to_graph: vec![None; pattern_height],
            graph_to_pattern: vec![None; graph_height],
        }
    }

    /// Add a mapping from a pattern qubit to a graph qubit.
    ///
    /// Returns `false` if the mapping already exists.
    pub fn add_mapping(&mut self, pattern_index: usize, graph_index: usize) -> bool {
        match (
            self.pattern_to_graph[pattern_index],
            self.graph_to_pattern[graph_index],
        ) {
            (None, None) => {
                self.pattern_to_graph[pattern_index] = Some(graph_index);
                self.graph_to_pattern[graph_index] = Some(pattern_index);
                true
            }
            (Some(existing), _) => existing == graph_index,
            (_, Some(existing)) => existing == pattern_index,
        }
    }

    /// Get the graph row corresponding to a pattern row.
    pub fn graph_row(&self, pattern_row: usize) -> Option<usize> {
        self.pattern_to_graph[pattern_row]
    }

    /// Get the pattern row corresponding to a graph row.
    pub fn pattern_row(&self, graph_row: usize) -> Option<usize> {
        self.graph_to_pattern[graph_row]
    }
}
