pub(crate) mod dto;

/// Orchestrator for common use cases related to the `QSimplify` library.
///
/// Provides a stable interface meant to be used by client implementations.
pub(crate) mod use_case;

pub use dto::{
    AnalysisRequest, AnalysisResponse, CodeGenerationRequest, CodeGenerationResponse,
    DisplayFormat, DisplayRequest, DisplayResponse, EstimationRequest, EstimationResponse,
    ParseRequest, ParseResponse, PresentationRequest, PresentationResponse, SerializeRequest,
    SerializeResponse, SimplificationRequest, SimplificationResponse,
};
pub use use_case::{
    analyze, display, estimate, generate_code, parse, present, serialize, simplify,
};
