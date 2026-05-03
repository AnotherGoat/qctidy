use std::io;

use qsimplify::{DiracFormat, PiFormat, dto::mapper, simplifier};
use qsimplify_ports::{
    AnalyzerPort, CodegenPort, ConverterPort, EstimatorPort, ParseError, PresenterPort,
    SerializeError,
};

use crate::{
    AnalysisRequest, AnalysisResponse, CodeGenerationRequest, CodeGenerationResponse,
    DisplayFormat, DisplayRequest, DisplayResponse, EstimationRequest, EstimationResponse,
    ParseRequest, ParseResponse, PresentationRequest, PresentationResponse, SerializeRequest,
    SerializeResponse, SimplificationRequest, SimplificationResponse,
};

pub fn display(request: &DisplayRequest) -> DisplayResponse {
    use DisplayFormat::*;

    let graph = mapper::operations_to_graph(&request.operations());
    let pi_format = request.pi_format().unwrap_or(PiFormat::Fancy);
    let dirac_format = request.dirac_format().unwrap_or(DiracFormat::Fancy);

    let text = match request.format() {
        Graph => graph.display_nodes_and_edges(pi_format),
        Grid => graph.display_grid(pi_format),
        Matrix => graph.display_matrix(dirac_format),
    };

    DisplayResponse::new(text)
}

pub fn simplify(request: &SimplificationRequest) -> SimplificationResponse {
    let graph = mapper::operations_to_graph(&request.operations());
    let simplified = simplifier::simplify(graph, request.iterations());

    let operations = mapper::graph_to_operations(&simplified);
    SimplificationResponse::new(operations.into())
}

pub fn present<P: PresenterPort>(
    request: &PresentationRequest,
    presenter: &P,
) -> Result<PresentationResponse, io::Error> {
    let graph = mapper::operations_to_graph(&request.operations());

    let result = presenter.present(&graph, request.format(), request.dpi())?;
    Ok(PresentationResponse::new(result.into()))
}

pub fn analyze<A: AnalyzerPort>(_request: &AnalysisRequest, _analyzer: &A) -> AnalysisResponse {
    todo!()
}

pub fn generate_code<C: CodegenPort>(
    _request: &CodeGenerationRequest,
    _codegen: &C,
) -> CodeGenerationResponse {
    todo!()
}

pub fn parse<C: ConverterPort>(
    request: &ParseRequest,
    converter: &C,
) -> Result<ParseResponse, ParseError> {
    converter
        .parse(&request.input(), request.format())
        .map(|operations| ParseResponse::new(operations.into()))
}

pub fn serialize<C: ConverterPort>(
    request: &SerializeRequest,
    converter: &C,
) -> Result<SerializeResponse, SerializeError> {
    converter
        .serialize(
            &request.operations(),
            request.format(),
            request.prettify(),
            request.indentation(),
        )
        .map(|bytes| SerializeResponse::new(bytes.into()))
}

pub fn estimate<E: EstimatorPort>(
    _request: &EstimationRequest,
    _estimator: &E,
) -> EstimationResponse {
    todo!()
}
