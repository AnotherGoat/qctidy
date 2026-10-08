use axum::extract::Json;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use qctidy_facade::FixRequest;

use crate::circuit;
use crate::error::ApiError;
use crate::schema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct FixCircuitRequest {
    #[schema(value_type = schema::Circuit, example = schema::example_circuit)]
    circuit: serde_json::Value,
    #[schema(example = 1)]
    iterations: u32,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct FixCircuitResponse {
    #[schema(value_type = schema::Circuit, example = schema::example_circuit)]
    circuit: serde_json::Value,
}

#[utoipa::path(
    post,
    path = "/fix",
    operation_id = "fix_circuit",
    request_body = FixCircuitRequest,
    responses(
        (status = 200, description = "Circuit fixed successfully", body = FixCircuitResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "circuit",
)]
pub(crate) async fn handler(
    Json(body): Json<FixCircuitRequest>,
) -> Result<Json<FixCircuitResponse>, ApiError> {
    let circ = circuit::from_json(&body.circuit)?;

    let request = FixRequest::new(circ, body.iterations);
    let response = qctidy_facade::fix(&request);

    let circuit_json = circuit::to_json(response.circuit())?;
    Ok(Json(FixCircuitResponse {
        circuit: circuit_json,
    }))
}
