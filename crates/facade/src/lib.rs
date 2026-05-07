#[cfg(feature = "analyzer")]
mod analyzer;
#[cfg(feature = "codegen")]
mod codegen;
#[cfg(feature = "converter")]
mod converter;
#[cfg(feature = "estimator")]
mod estimator;
#[cfg(feature = "presenter")]
mod presenter;

#[cfg(feature = "analyzer")]
pub use analyzer::{AnalysisRequest, AnalysisResponse, analyze};
#[cfg(feature = "codegen")]
pub use codegen::{CodeGenerationRequest, CodeGenerationResponse, generate_code};
#[cfg(feature = "converter")]
pub use converter::{
    ParseRequest, ParseResponse, SerializeRequest, SerializeResponse, parse, serialize,
};
#[cfg(feature = "estimator")]
pub use estimator::{EstimationRequest, EstimationResponse, estimate};
#[cfg(feature = "presenter")]
pub use presenter::{PresentationRequest, PresentationResponse, present};
use qsimplify_ports::{DisplayError, SimplificationError};
use std::sync::Arc;

use getset::{CloneGetters, CopyGetters, Getters};
use inew::New;
use qsimplify::{DiracFormat, GateOperation, PiFormat, mapper, simplifier};

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct DisplayRequest {
    #[get_clone = "pub"]
    operations: Arc<[GateOperation]>,
    #[get_copy = "pub"]
    format: DisplayFormat,
    #[get_copy = "pub"]
    pi_format: Option<PiFormat>,
    #[get_copy = "pub"]
    dirac_format: Option<DiracFormat>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayFormat {
    Graph,
    Grid,
    Matrix,
}

#[derive(Debug, Clone, Getters, New)]
#[new(pub, const)]
#[must_use]
pub struct DisplayResponse {
    #[get = "pub"]
    text: String,
}

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct SimplificationRequest {
    #[get_clone = "pub"]
    operations: Arc<[GateOperation]>,
    #[get_copy = "pub"]
    iterations: u32,
}

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct SimplificationResponse {
    #[get_clone = "pub"]
    operations: Arc<[GateOperation]>,
}

pub fn display(request: &DisplayRequest) -> Result<DisplayResponse, DisplayError> {
    use DisplayFormat::*;

    let graph = mapper::operations_to_graph(&request.operations());
    let pi_format = request.pi_format().unwrap_or(PiFormat::Fancy);
    let dirac_format = request.dirac_format().unwrap_or(DiracFormat::Fancy);

    let text = match request.format() {
        Graph => graph.display_nodes_and_edges(pi_format),
        Grid => graph.display_grid(pi_format),
        Matrix => graph.display_matrix(dirac_format),
    };

    Ok(DisplayResponse::new(text))
}

pub fn simplify(
    request: &SimplificationRequest,
) -> Result<SimplificationResponse, SimplificationError> {
    let graph = mapper::operations_to_graph(&request.operations());
    let simplified = simplifier::simplify(graph, request.iterations());

    let operations = mapper::graph_to_operations(&simplified);
    Ok(SimplificationResponse::new(operations.into()))
}
