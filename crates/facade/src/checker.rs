use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use newgen::New;
use qctidy::{
    Circuit, Detection, Graph, Position, RuleConfiguration, RuleLevel, RuleMetadata, simplifier,
};

/// A simplification opportunity found in a circuit.
#[derive(Debug, Clone, Getters, CopyGetters, New)]
#[new(pub)]
#[must_use]
pub struct CheckDiagnostic {
    /// Metadata of the rule that was detected.
    #[get_copy = "pub"]
    metadata: RuleMetadata,
    /// Positions affected by the detected match, sorted by row and then column.
    #[get = "pub"]
    positions: Vec<Position>,
    /// Indices of the operations that occupy the affected positions, sorted and deduplicated.
    #[get = "pub"]
    operation_indices: Vec<usize>,
}

#[derive(Debug, Clone, Getters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct CheckResponse {
    #[get = "pub"]
    diagnostics: Vec<CheckDiagnostic>,
    #[get_copy = "pub"]
    qubit_count: usize,
    #[get_copy = "pub"]
    time_step_count: usize,
    #[get_copy = "pub"]
    gate_count: usize,
}

/// A checking session that reuses a single rule configuration across circuits.
#[derive(Debug, Clone)]
pub struct Session {
    configuration: RuleConfiguration,
}

impl Session {
    /// Create a session that checks circuits with the given rule configuration.
    #[must_use]
    pub const fn new(configuration: RuleConfiguration) -> Self {
        Self { configuration }
    }

    /// Find simplification opportunities in a circuit, without modifying it.
    pub fn check(&self, circuit: &Circuit) -> CheckResponse {
        let graph = Graph::from(circuit);
        let position_operations = index_operations(circuit);
        let diagnostics = simplifier::detect(&graph, &self.configuration)
            .iter()
            .map(|detection| diagnostic(detection, &position_operations))
            .collect();

        CheckResponse::new(
            diagnostics,
            graph.height(),
            graph.width(),
            circuit.operations().len(),
        )
    }
}

impl Default for Session {
    /// Create a session that detects every rule, without applying any.
    fn default() -> Self {
        Self::new(RuleConfiguration::new(RuleLevel::Detect))
    }
}

fn index_operations(circuit: &Circuit) -> HashMap<Position, usize> {
    circuit
        .operation_positions()
        .into_iter()
        .enumerate()
        .flat_map(|(index, positions)| positions.into_iter().map(move |position| (position, index)))
        .collect()
}

fn diagnostic(
    detection: &Detection,
    position_operations: &HashMap<Position, usize>,
) -> CheckDiagnostic {
    let mut operation_indices: Vec<usize> = detection
        .positions()
        .iter()
        .filter_map(|position| position_operations.get(position).copied())
        .collect();

    operation_indices.sort_unstable();
    operation_indices.dedup();

    CheckDiagnostic::new(
        *detection.metadata(),
        detection.positions().clone(),
        operation_indices,
    )
}
