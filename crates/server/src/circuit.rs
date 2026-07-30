use std::sync::Arc;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use qctidy::Circuit;
use qctidy_converter::ConverterAdapter;
use qctidy_facade::ParseRequest;
use qctidy_facade::SerializeRequest;
use qctidy_ports::ConversionFormat;

use crate::error::ApiError;

pub(crate) fn from_json(json: &serde_json::Value) -> Result<Arc<Circuit>, ApiError> {
    let bytes = serde_json::to_vec(json)
        .map_err(|error| ApiError::BadRequest(format!("Invalid circuit JSON: {error}")))?;
    let request = ParseRequest::new(bytes.into(), ConversionFormat::Json);
    let response = qctidy_facade::parse(&request, &ConverterAdapter)
        .map_err(|error| ApiError::BadRequest(format!("Failed to parse circuit: {error}")))?;
    Ok(response.circuit())
}

pub(crate) fn to_json(circuit: Arc<Circuit>) -> Result<serde_json::Value, ApiError> {
    let request = SerializeRequest::new(circuit, ConversionFormat::Json, Some(true), Some(2));
    let response = qctidy_facade::serialize(&request, &ConverterAdapter)
        .map_err(|error| ApiError::Internal(format!("Failed to serialize circuit: {error}")))?;
    serde_json::from_slice(&response.bytes())
        .map_err(|error| ApiError::Internal(format!("Failed to parse serialized circuit: {error}")))
}

pub(crate) fn parse_conversion_format(s: &str) -> Option<ConversionFormat> {
    match s.to_lowercase().as_str() {
        "json" => Some(ConversionFormat::Json),
        "xml" => Some(ConversionFormat::Xml),
        "msgpack" | "message_pack" => Some(ConversionFormat::MessagePack),
        "cbor" => Some(ConversionFormat::Cbor),
        _ => None,
    }
}

pub(crate) fn encode_b64(bytes: &[u8]) -> String {
    STANDARD.encode(bytes)
}

pub(crate) fn decode_b64(s: &str) -> Result<Vec<u8>, ApiError> {
    STANDARD
        .decode(s)
        .map_err(|error| ApiError::BadRequest(format!("Invalid base64: {error}")))
}
