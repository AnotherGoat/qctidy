use axum::extract::Json;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use qsimplify_codegen::CodegenAdapter;
use qsimplify_facade::CodeGenerationRequest;
use qsimplify_ports::CodeGenerationTarget;

use crate::circuit;
use crate::error::ApiError;

#[derive(Deserialize, ToSchema)]
pub(crate) struct CodegenRequestBody {
    circuit: serde_json::Value,
    target: String,
    circuit_name: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct CodegenResponseBody {
    code: String,
}

#[utoipa::path(
    post,
    path = "/codegen",
    request_body = CodegenRequestBody,
    responses(
        (status = 200, description = "Code generated successfully", body = CodegenResponseBody),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "codegen",
)]
pub(crate) async fn handler(
    Json(body): Json<CodegenRequestBody>,
) -> Result<Json<CodegenResponseBody>, ApiError> {
    let circ = circuit::from_json(&body.circuit)?;

    let target = match body.target.to_lowercase().as_str() {
        "qiskit" => CodeGenerationTarget::Qiskit,
        "openqasm3" => CodeGenerationTarget::OpenQasm3,
        other => return Err(ApiError::BadRequest(format!("Unknown target: {other}"))),
    };

    let request = CodeGenerationRequest::new(circ, target, body.circuit_name);
    let response = qsimplify_facade::generate_code(&request, &CodegenAdapter)
        .map_err(|error| ApiError::Internal(error.to_string()))?;

    Ok(Json(CodegenResponseBody {
        code: response.code().clone(),
    }))
}
