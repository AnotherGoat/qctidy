use std::sync::Arc;

use getset::{CloneGetters, CopyGetters};
use inew::New;
use qctidy::{Circuit, Graph};
use qctidy_ports::{Estimation, EstimationError, EstimatorPort};

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct EstimationRequest {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
    #[get_copy = "pub"]
    shots: usize,
}

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct EstimationResponse {
    #[get_clone = "pub"]
    estimation: Estimation,
}

pub fn estimate<E: EstimatorPort>(
    request: &EstimationRequest,
    estimator: &E,
) -> Result<EstimationResponse, EstimationError> {
    let graph = Graph::from(request.circuit().as_ref());

    estimator
        .estimate(&graph, request.shots())
        .map(EstimationResponse::new)
}
