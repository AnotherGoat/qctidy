use qctidy_ports::SimplificationError;
use std::sync::Arc;

use getset::{CloneGetters, CopyGetters};
use newgen::New;
use qctidy::{Circuit, Graph, simplifier};

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct SimplificationRequest {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
    #[get_copy = "pub"]
    iterations: u32,
}

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct SimplificationResponse {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
}

pub fn simplify(
    request: &SimplificationRequest,
) -> Result<SimplificationResponse, SimplificationError> {
    let graph: Graph = request.circuit().as_ref().into();
    let simplified = simplifier::simplify(graph, request.iterations());

    let circuit: Circuit = (&simplified).into();
    Ok(SimplificationResponse::new(circuit.into()))
}
