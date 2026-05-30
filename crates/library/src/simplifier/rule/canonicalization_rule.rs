use std::fmt;
use std::{any::Any, collections::HashSet};

use crate::{GateType, Graph, Position, RuleMetadata, SimplificationRule};

/// A simplification rule that applies a canonicalization transformation to the entire graph.
///
/// Unlike `PatternRule` which matches and replaces specific subgraph patterns, canonicalization rules perform global transformations on the graph structure.
pub struct CanonicalizationRule {
    metadata: RuleMetadata,
    gate_types: HashSet<GateType>,
    width: usize,
    height: usize,
    detect_function: Box<dyn Fn(&Graph) -> Vec<HashSet<Position>> + Send + Sync>,
    apply_function: Box<dyn Fn(&mut Graph) -> bool + Send + Sync>,
}

impl fmt::Debug for CanonicalizationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CanonicalizationRule")
            .field("metadata", &self.metadata)
            .field("gate_types", &self.gate_types)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl CanonicalizationRule {
    #[must_use]
    pub fn new(
        metadata: RuleMetadata,
        gate_types: HashSet<GateType>,
        width: usize,
        height: usize,
        detect_function: Box<dyn Fn(&Graph) -> Vec<HashSet<Position>> + Send + Sync>,
        apply_function: Box<dyn Fn(&mut Graph) -> bool + Send + Sync>,
    ) -> Self {
        Self {
            metadata,
            gate_types,
            width,
            height,
            detect_function,
            apply_function,
        }
    }
}

impl SimplificationRule for CanonicalizationRule {
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
        (self.detect_function)(graph)
    }

    fn apply(&self, graph: &mut Graph) -> bool {
        (self.apply_function)(graph)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
