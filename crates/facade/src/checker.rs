use std::collections::HashMap;
use std::sync::Arc;

use getset::{CloneGetters, CopyGetters, Getters};
use newgen::New;
use qctidy::{
    Circuit, Detection, Graph, Position, RuleConfiguration, RuleLevel, RuleMetadata, simplifier,
};
use qctidy_ports::CheckError;

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct CheckRequest {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
}

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
    operations: Vec<usize>,
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

/// Find simplification opportunities in a circuit, without modifying it.
pub fn check(request: &CheckRequest) -> Result<CheckResponse, CheckError> {
    let circuit = request.circuit();
    let graph = Graph::from(circuit.as_ref());
    let configuration = RuleConfiguration::new(RuleLevel::Detect);
    let position_operations = index_operations(&circuit);
    let diagnostics = simplifier::detect(&graph, &configuration)
        .iter()
        .map(|detection| diagnostic(detection, &position_operations))
        .collect();

    Ok(CheckResponse::new(
        diagnostics,
        graph.height(),
        graph.width(),
        circuit.operations().len(),
    ))
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
    let mut operations: Vec<usize> = detection
        .positions()
        .iter()
        .filter_map(|position| position_operations.get(position).copied())
        .collect();

    operations.sort_unstable();
    operations.dedup();

    CheckDiagnostic::new(
        *detection.metadata(),
        detection.positions().clone(),
        operations,
    )
}
