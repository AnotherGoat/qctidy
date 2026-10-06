use std::sync::Arc;

use getset::{CloneGetters, CopyGetters, Getters};
use newgen::New;
use qctidy::{Circuit, Detection, Graph, RuleConfiguration, RuleLevel, simplifier};
use qctidy_ports::CheckError;

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct CheckRequest {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
}

#[derive(Debug, Clone, Getters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct CheckResponse {
    #[get = "pub"]
    diagnostics: Vec<Detection>,
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
    let diagnostics = simplifier::detect(&graph, &configuration);

    Ok(CheckResponse::new(
        diagnostics,
        graph.height(),
        graph.width(),
        circuit.operations().len(),
    ))
}
