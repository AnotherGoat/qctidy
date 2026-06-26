use axum::extract::Json;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use qsimplify_facade::SimplificationRequest;

use crate::circuit;
use crate::error::ApiError;
use crate::schema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct SimplifyCircuitRequest {
    #[schema(value_type = schema::Circuit, example = schema::example_circuit)]
    circuit: serde_json::Value,
    #[schema(example = 1)]
    iterations: u32,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct SimplifyCircuitResponse {
    #[schema(value_type = schema::Circuit, example = schema::example_circuit)]
    circuit: serde_json::Value,
}

#[utoipa::path(
    post,
    path = "/simplify",
    operation_id = "simplify_circuit",
    request_body = SimplifyCircuitRequest,
    responses(
        (status = 200, description = "Circuit simplified successfully", body = SimplifyCircuitResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "circuit",
)]
pub(crate) async fn handler(
    Json(body): Json<SimplifyCircuitRequest>,
) -> Result<Json<SimplifyCircuitResponse>, ApiError> {
    let circ = circuit::from_json(&body.circuit)?;

    let request = SimplificationRequest::new(circ, body.iterations);
    let response = qsimplify_facade::simplify(&request)
        .map_err(|error| ApiError::Internal(error.to_string()))?;

    let circuit_json = circuit::to_json(response.circuit())?;
    Ok(Json(SimplifyCircuitResponse {
        circuit: circuit_json,
    }))
}
