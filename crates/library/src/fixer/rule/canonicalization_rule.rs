use std::fmt;
use std::{any::Any, collections::HashSet};

use crate::{GateType, Graph, Position, Rule, RuleMetadata};

/// A function that detects the positions a canonicalization rule applies to.
type DetectFunction = Box<dyn Fn(&Graph) -> Vec<HashSet<Position>> + Send + Sync>;

/// A fix rule that applies a canonicalization transformation to the entire graph.
///
/// Unlike `PatternRule` which matches and replaces specific subgraph patterns, canonicalization rules perform global transformations on the graph structure.
pub struct CanonicalizationRule {
    metadata: RuleMetadata,
    gate_types: HashSet<GateType>,
    time_step_count: usize,
    qubit_count: usize,
    detect_function: DetectFunction,
    apply_function: Box<dyn Fn(&mut Graph) -> bool + Send + Sync>,
}

impl fmt::Debug for CanonicalizationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CanonicalizationRule")
            .field("metadata", &self.metadata)
            .field("gate_types", &self.gate_types)
            .field("time_step_count", &self.time_step_count)
            .field("qubit_count", &self.qubit_count)
            .finish()
    }
}

impl CanonicalizationRule {
    #[must_use]
    pub fn new(
        metadata: RuleMetadata,
        gate_types: HashSet<GateType>,
        time_step_count: usize,
        qubit_count: usize,
        detect_function: DetectFunction,
        apply_function: Box<dyn Fn(&mut Graph) -> bool + Send + Sync>,
    ) -> Self {
        Self {
            metadata,
            gate_types,
            time_step_count,
            qubit_count,
            detect_function,
            apply_function,
        }
    }
}

impl Rule for CanonicalizationRule {
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
        (self.detect_function)(graph)
    }

    fn apply(&self, graph: &mut Graph) -> bool {
        (self.apply_function)(graph)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
