use std::sync::Arc;

use getset::{CloneGetters, CopyGetters};
use inew::New;
use qsimplify::Circuit;
use qsimplify_ports::{EstimationError, EstimatorPort};

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct EstimationRequest {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
}

#[derive(Debug, Clone, Copy, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct EstimationResponse {
    #[get_copy = "pub"]
    execution_time: f64,
    #[get_copy = "pub"]
    cost: f64,
}

#[expect(clippy::unimplemented)]
pub fn estimate<E: EstimatorPort>(
    _request: &EstimationRequest,
    _estimator: &E,
) -> Result<EstimationResponse, EstimationError> {
    unimplemented!()
}
