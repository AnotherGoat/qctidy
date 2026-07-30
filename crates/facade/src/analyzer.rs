use std::sync::Arc;

use getset::{CloneGetters, CopyGetters};
use inew::New;
use qctidy::{Circuit, Graph};
use qctidy_ports::{
    AnalysisError, AnalysisMode, AnalysisResult, AnalyzerPort, DeltaAnalysisResult,
};

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct AnalysisRequest {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
    #[get_copy = "pub"]
    mode: AnalysisMode,
}

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct ComparisonRequest {
    #[get_clone = "pub"]
    old_circuit: Arc<Circuit>,
    #[get_clone = "pub"]
    new_circuit: Arc<Circuit>,
    #[get_copy = "pub"]
    mode: AnalysisMode,
}

#[derive(Debug, Clone, Copy, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct AnalysisResponse {
    #[get_copy = "pub"]
    result: AnalysisResult,
}

#[derive(Debug, Clone, Copy, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct ComparisonResponse {
    #[get_copy = "pub"]
    result: DeltaAnalysisResult,
}

pub fn analyze<A: AnalyzerPort>(
    request: &AnalysisRequest,
    analyzer: &A,
) -> Result<AnalysisResponse, AnalysisError> {
    let graph = Graph::from(request.circuit().as_ref());

    Ok(AnalysisResponse::new(
        analyzer.analyze(&graph, request.mode()),
    ))
}

pub fn compare<A: AnalyzerPort>(
    request: &ComparisonRequest,
    analyzer: &A,
) -> Result<ComparisonResponse, AnalysisError> {
    let old_graph = Graph::from(request.old_circuit().as_ref());
    let new_graph = Graph::from(request.new_circuit().as_ref());

    Ok(ComparisonResponse::new(analyzer.compare(
        &old_graph,
        &new_graph,
        request.mode(),
    )))
}
