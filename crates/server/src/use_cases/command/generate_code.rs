use axum::extract::Json;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use qctidy_codegen::CodegenAdapter;
use qctidy_facade::CodeGenerationRequest;
use qctidy_ports::CodeGenerationTarget;

use crate::circuit;
use crate::error::ApiError;
use crate::schema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct GenerateCodeRequest {
    #[schema(value_type = schema::Circuit, example = schema::example_circuit)]
    circuit: serde_json::Value,
    #[schema(value_type = schema::CodeGenerationTargetName, example = "qiskit")]
    target: String,
    #[schema(example = "circuit")]
    circuit_name: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct GenerateCodeResponse {
    code: String,
}

#[utoipa::path(
    post,
    path = "/codegen",
    operation_id = "generate_code",
    request_body = GenerateCodeRequest,
    responses(
        (status = 200, description = "Code generated successfully", body = GenerateCodeResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "codegen",
)]
pub(crate) async fn handler(
    Json(body): Json<GenerateCodeRequest>,
) -> Result<Json<GenerateCodeResponse>, ApiError> {
    let circ = circuit::from_json(&body.circuit)?;

    let target = match body.target.to_lowercase().as_str() {
        "qiskit" => CodeGenerationTarget::Qiskit,
        "openqasm3" => CodeGenerationTarget::OpenQasm3,
        other => return Err(ApiError::BadRequest(format!("Unknown target: {other}"))),
    };

    let request = CodeGenerationRequest::new(circ, target, body.circuit_name);
    let response = qctidy_facade::generate_code(&request, &CodegenAdapter)
        .map_err(|error| ApiError::Internal(error.to_string()))?;

    Ok(Json(GenerateCodeResponse {
        code: response.code().clone(),
    }))
}
