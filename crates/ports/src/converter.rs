use qsimplify::Circuit;
use thiserror::Error;

pub trait ConverterPort {
    fn parse(&self, input: &[u8], format: ConversionFormat) -> Result<Circuit, ParseError>;

    fn serialize(
        &self,
        circuit: &Circuit,
        format: ConversionFormat,
        prettify: Option<bool>,
        indentation: Option<usize>,
    ) -> Result<Vec<u8>, SerializeError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display)]
pub enum ConversionFormat {
    Json,
    Xml,
    MessagePack,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid {format} input: {message}")]
    InvalidInput {
        format: ConversionFormat,
        message: String,
    },
    #[error("Missing required field '{field}' for gate '{gate}'")]
    MissingRequiredField { field: String, gate: String },
    #[error("Unknown field '{field}' for gate '{gate}'")]
    UnknownField { field: String, gate: String },
    #[error("Unknown gate type: '{gate}'")]
    UnknownGateType { gate: String },
    #[error("Unsupported input format version: {version}")]
    UnsupportedVersion { version: u16 },
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
