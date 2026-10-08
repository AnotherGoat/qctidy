use std::sync::Arc;

use getset::{CloneGetters, CopyGetters};
use newgen::New;
use qctidy::{Circuit, Graph, fixer};

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct FixRequest {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
    #[get_copy = "pub"]
    iterations: u32,
}

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct FixResponse {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
}

pub fn fix(request: &FixRequest) -> FixResponse {
    let graph: Graph = request.circuit().as_ref().into();
    let fixed = fixer::fix(graph, request.iterations());

    let circuit: Circuit = (&fixed).into();
    FixResponse::new(circuit.into())
}
