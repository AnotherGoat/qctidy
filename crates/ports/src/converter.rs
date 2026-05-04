use qsimplify::GateOperation;
use thiserror::Error;

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
        prettify: Option<bool>,
        indentation: Option<usize>,
    ) -> Result<Vec<u8>, SerializeError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display)]
pub enum ConversionFormat {
    Json,
    MessagePack,
    Binary,
    Base64,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid {format} input: {message}")]
    InvalidInput {
        format: ConversionFormat,
        message: String,
    },
    #[error("Missing required {format} field '{field}' for gate '{gate}'")]
    MissingRequiredField {
        format: ConversionFormat,
        field: String,
        gate: String,
    },
    #[error("Unknown field '{field}' for gate '{gate}'")]
    UnknownField { field: String, gate: String },
    #[error("Unknown gate type: '{gate}'")]
    UnknownGateType { gate: String },
    #[error("Unsupported conversion format")]
    UnsupportedFormat,
}

#[derive(Debug, Error)]
pub enum SerializeError {
    #[error("Failed to serialize to {format}: {message}")]
    SerializationFailure {
        format: ConversionFormat,
        message: String,
    },
    #[error("Unsupported conversion format")]
    UnsupportedFormat,
}
