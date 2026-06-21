use axum::extract::Json;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use qsimplify_facade::SimplificationRequest;

use crate::circuit;
use crate::error::ApiError;

#[derive(Deserialize, ToSchema)]
pub(crate) struct SimplifyRequestBody {
    circuit: serde_json::Value,
    iterations: u32,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct SimplifyResponseBody {
    circuit: serde_json::Value,
}

#[utoipa::path(
    post,
    path = "/simplify",
    request_body = SimplifyRequestBody,
    responses(
        (status = 200, description = "Circuit simplified successfully", body = SimplifyResponseBody),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "circuit",
)]
pub(crate) async fn handler(
    Json(body): Json<SimplifyRequestBody>,
) -> Result<Json<SimplifyResponseBody>, ApiError> {
    let circ = circuit::from_json(&body.circuit)?;

    let request = SimplificationRequest::new(circ, body.iterations);
    let response = qsimplify_facade::simplify(&request)
        .map_err(|error| ApiError::Internal(error.to_string()))?;

    let circuit_json = circuit::to_json(response.circuit())?;
    Ok(Json(SimplifyResponseBody {
        circuit: circuit_json,
    }))
}
