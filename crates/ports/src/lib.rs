use std::io;

use qsimplify::{Graph, dto::GateOperation};
use thiserror::Error;

pub trait PresenterPort {
    fn present(
        &self,
        graph: &Graph,
        format: PresentationFormat,
        dpi: Option<u32>,
    ) -> Result<Vec<u8>, io::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationFormat {
    GraphvizGv,
    GraphvizPng,
    GraphvizSvg,
}

pub trait AnalyzerPort {
    fn analyze(&self, graph: &Graph) -> ();
}

pub trait CodegenPort {
    fn generate(&self, graph: &Graph, target: CodeGenerationTarget) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeGenerationTarget {
    Qiskit,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid JSON input: {message}")]
    InvalidJson { message: String },
    #[error("Missing required field '{field}' for gate '{gate}'")]
    MissingRequiredJsonField { field: String, gate: String },
    #[error("Unknown field '{field}' for gate '{gate}'")]
    UnknownJsonField { field: String, gate: String },
    #[error("Unknown gate type: '{gate}'")]
    UnknownGateType { gate: String },
    #[error("Unsupported conversion format")]
    UnsupportedFormat,
}

#[derive(Debug, Error)]
pub enum SerializeError {
    #[error("Failed to serialize to JSON: {message}")]
    JsonSerializationFail { message: String },
    #[error("Unsupported conversion format")]
    UnsupportedFormat,
}

pub trait ConverterPort {
    fn parse(
        &self,
        input: &[u8],
        format: ConversionFormat,
    ) -> Result<Vec<GateOperation>, ParseError>;

    fn serialize(
        &self,
        operations: &[GateOperation],
        format: ConversionFormat,
    ) -> Result<Vec<u8>, SerializeError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionFormat {
    Json,
    Binary,
    Base64,
}

pub trait EstimatorPort {
    fn estimate(&self, graph: &Graph) -> (f64, f64);
}
