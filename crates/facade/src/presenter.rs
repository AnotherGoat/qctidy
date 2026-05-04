use std::sync::Arc;

use getset::{CloneGetters, CopyGetters};
use inew::New;
use qsimplify::{GateOperation, mapper};
use qsimplify_ports::{PresentationError, PresentationFormat, PresenterPort};

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct PresentationRequest {
    #[get_clone = "pub"]
    operations: Arc<[GateOperation]>,
    #[get_copy = "pub"]
    format: PresentationFormat,
    #[get_copy = "pub"]
    dpi: Option<u32>,
}

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct PresentationResponse {
    #[get_clone = "pub"]
    bytes: Arc<[u8]>,
}

pub fn present<P: PresenterPort>(
    request: &PresentationRequest,
    presenter: &P,
) -> Result<PresentationResponse, PresentationError> {
    let graph = mapper::operations_to_graph(&request.operations());

    let result = presenter.present(&graph, request.format(), request.dpi())?;
    Ok(PresentationResponse::new(result.into()))
}
