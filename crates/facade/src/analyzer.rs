use std::sync::Arc;

use getset::{CloneGetters, CopyGetters};
use inew::New;
use qsimplify::GateOperation;
use qsimplify_ports::{AnalysisError, AnalyzerPort};

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct AnalysisRequest {
    #[get_clone = "pub"]
    operations: Arc<[GateOperation]>,
}

#[derive(Debug, Clone, Copy, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct AnalysisResponse {
    #[get_copy = "pub"]
    gate_count: usize,
}

#[expect(clippy::unimplemented)]
pub fn analyze<A: AnalyzerPort>(
    _request: &AnalysisRequest,
    _analyzer: &A,
) -> Result<AnalysisResponse, AnalysisError> {
    unimplemented!()
}
