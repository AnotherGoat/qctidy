mod analyzer;
mod codegen;
mod converter;
mod estimator;
mod presenter;

pub use analyzer::{
    AnalysisError, AnalysisMetrics, AnalysisMode, AnalysisResult, AnalyzerPort,
    DeltaAnalysisMetrics, DeltaAnalysisResult, DeltaDetailedAnalysisMetrics,
    DetailedAnalysisMetrics,
};
pub use codegen::{CodeGenerationError, CodeGenerationTarget, CodegenPort};
pub use converter::{ConversionFormat, ConverterPort, ParseError, SerializeError};
pub use estimator::{EstimationError, EstimatorPort};
pub use presenter::{PresentationError, PresentationFormat, PresenterPort};

use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum DisplayError {}

#[derive(Debug, Clone, Copy, Error)]
pub enum SimplificationError {}

#[derive(thiserror::Error, Debug)]
pub enum UseCaseError {
    #[error(transparent)]
    Display(#[from] DisplayError),
    #[error(transparent)]
    Simplification(#[from] SimplificationError),
    #[error(transparent)]
    Presentation(#[from] PresentationError),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Serialize(#[from] SerializeError),
    #[error(transparent)]
    Codegen(#[from] CodeGenerationError),
    #[error(transparent)]
    Analyzer(#[from] AnalysisError),
    #[error(transparent)]
    Estimator(#[from] EstimationError),
}
