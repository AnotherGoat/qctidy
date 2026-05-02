use std::sync::Arc;

use getset::{CloneGetters, CopyGetters, Getters};
use inew::New;
use qsimplify::{DiracFormat, PiFormat, dto::GateOperation};
use qsimplify_ports::{CodeGenerationTarget, ConversionFormat, PresentationFormat};

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

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct CodeGenerationRequest {
    #[get_clone = "pub"]
    operations: Arc<[GateOperation]>,
    #[get_copy = "pub"]
    target: CodeGenerationTarget,
}

#[derive(Debug, Clone, Getters, New)]
#[new(pub, const)]
#[must_use]
pub struct CodeGenerationResponse {
    #[get = "pub"]
    code: String,
}

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct ParseRequest {
    #[get_clone = "pub"]
    input: Arc<[u8]>,
    #[get_copy = "pub"]
    format: ConversionFormat,
}

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct ParseResponse {
    #[get_clone = "pub"]
    operations: Arc<[GateOperation]>,
}

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct SerializeRequest {
    #[get_clone = "pub"]
    operations: Arc<[GateOperation]>,
    #[get_copy = "pub"]
    format: ConversionFormat,
    #[get_copy = "pub"]
    prettify: Option<bool>,
    #[get_copy = "pub"]
    indentation: Option<usize>,
}

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct SerializeResponse {
    #[get_clone = "pub"]
    bytes: Arc<[u8]>,
}

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct EstimationRequest {
    #[get_clone = "pub"]
    operations: Arc<[GateOperation]>,
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
