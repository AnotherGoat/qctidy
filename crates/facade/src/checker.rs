use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use newgen::New;
use qctidy::{Circuit, Graph, Position, RuleConfiguration, RuleMetadata, RuleSeverity, fixer};

/// A fixable pattern found in a circuit.
#[derive(Debug, Clone, Getters, CopyGetters, New)]
#[new(pub)]
#[must_use]
pub struct Diagnostic {
    /// Metadata of the rule that was detected.
    #[get_copy = "pub"]
    metadata: RuleMetadata,
    /// Severity reported for the detected match.
    #[get_copy = "pub"]
    severity: RuleSeverity,
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
    diagnostics: Vec<Diagnostic>,
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

    /// Find fixable patterns in a circuit, without modifying it.
    pub fn check(&self, circuit: &Circuit) -> CheckResponse {
        let graph = Graph::from(circuit);
        let position_operations = index_operations(circuit);
        let diagnostics = fixer::detect(&graph, &self.configuration)
            .iter()
            .map(|diagnostic| map_diagnostic(diagnostic, &position_operations))
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
    /// Create a session that detects every rule, without fixing any.
    fn default() -> Self {
        Self::new(RuleConfiguration::new(RuleSeverity::Warn))
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

fn map_diagnostic(
    diagnostic: &qctidy::Diagnostic,
    position_operations: &HashMap<Position, usize>,
) -> Diagnostic {
    let mut operation_indices: Vec<usize> = diagnostic
        .positions()
        .iter()
        .filter_map(|position| position_operations.get(position).copied())
        .collect();

    operation_indices.sort_unstable();
    operation_indices.dedup();

    Diagnostic::new(
        *diagnostic.metadata(),
        diagnostic.severity(),
        diagnostic.positions().clone(),
        operation_indices,
    )
}
