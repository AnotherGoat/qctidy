use axum::extract::Json;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;

use qsimplify_converter::ConverterAdapter;
use qsimplify_facade::ParseRequest;
use qsimplify_facade::SerializeRequest;
use qsimplify_ports::ConversionFormat;

use crate::circuit;
use crate::error::ApiError;
use crate::schema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct ConvertCircuitRequest {
    #[schema(value_type = schema::ConversionFormatName, example = "json")]
    source_format: String,
    #[schema(value_type = schema::ConversionFormatName, example = "cbor")]
    target_format: String,
    #[schema(value_type = schema::CircuitPayload, example = schema::example_circuit_payload)]
    circuit: serde_json::Value,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct ConvertCircuitResponse {
    #[schema(value_type = schema::ConversionFormatName, example = "json")]
    source_format: String,
    #[schema(value_type = schema::ConversionFormatName, example = "cbor")]
    target_format: String,
    #[schema(value_type = schema::CircuitPayload, example = schema::example_circuit_payload)]
    circuit: serde_json::Value,
}

#[derive(Deserialize)]
struct Base64Circuit {
    encoding: String,
    data: String,
}

#[utoipa::path(
    post,
    path = "/convert",
    request_body = ConvertCircuitRequest,
    responses(
        (status = 200, description = "Circuit converted successfully", body = ConvertCircuitResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "conversion",
)]
pub(crate) async fn handler(
    Json(body): Json<ConvertCircuitRequest>,
) -> Result<Json<ConvertCircuitResponse>, ApiError> {
    let source_format = circuit::parse_conversion_format(&body.source_format).ok_or_else(|| {
        ApiError::BadRequest(format!("Unknown source_format: {}", body.source_format))
    })?;
    let target_format = circuit::parse_conversion_format(&body.target_format).ok_or_else(|| {
        ApiError::BadRequest(format!("Unknown target_format: {}", body.target_format))
    })?;

    if source_format == target_format {
        return Err(ApiError::BadRequest(format!(
            "source_format and target_format must be different: {}",
            body.source_format
        )));
    }

    let input_bytes = decode_circuit(&body.circuit, source_format)?;

    let parsed = qsimplify_facade::parse(
        &ParseRequest::new(input_bytes.into(), source_format),
        &ConverterAdapter,
    )
    .map_err(|error| ApiError::BadRequest(format!("Parse error: {error}")))?;

    let serialized = qsimplify_facade::serialize(
        &SerializeRequest::new(parsed.circuit(), target_format, Some(false), None),
        &ConverterAdapter,
    )
    .map_err(|error| ApiError::Internal(format!("Serialize error: {error}")))?;

    Ok(Json(ConvertCircuitResponse {
        source_format: body.source_format,
        target_format: body.target_format,
        circuit: encode_circuit(serialized.bytes().as_ref(), target_format)?,
    }))
}

fn decode_circuit(
    circuit: &serde_json::Value,
    source_format: ConversionFormat,
) -> Result<Vec<u8>, ApiError> {
    if matches!(source_format, ConversionFormat::Json) {
        serde_json::to_vec(circuit)
            .map_err(|error| ApiError::BadRequest(format!("Invalid circuit JSON: {error}")))
    } else {
        let circuit: Base64Circuit = serde_json::from_value(circuit.clone()).map_err(|error| {
            ApiError::BadRequest(format!("Invalid base64 circuit envelope: {error}"))
        })?;

        if circuit.encoding != "base64" {
            return Err(ApiError::BadRequest(format!(
                "Unsupported circuit encoding: {}",
                circuit.encoding
            )));
        }

        circuit::decode_b64(&circuit.data)
    }
}

fn encode_circuit(
    bytes: &[u8],
    target_format: ConversionFormat,
) -> Result<serde_json::Value, ApiError> {
    if matches!(target_format, ConversionFormat::Json) {
        serde_json::from_slice(bytes)
            .map_err(|error| ApiError::Internal(format!("Invalid serialized JSON: {error}")))
    } else {
        Ok(json!({
            "encoding": "base64",
            "data": circuit::encode_b64(bytes),
        }))
    }
}
