use axum::extract::Json;
use qctidy_estimator::EstimatorAdapter;
use qctidy_facade::EstimationRequest;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::circuit;
use crate::error::ApiError;
use crate::schema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct EstimateCircuitRequest {
    #[schema(value_type = schema::Circuit, example = schema::example_circuit)]
    circuit: serde_json::Value,
    #[schema(example = 1000)]
    shots: Option<usize>,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct ApiEstimatedCost {
    #[schema(example = "IBM")]
    provider: String,
    #[schema(example = "Pay-As-You-Go")]
    plan_name: String,
    #[schema(example = "Per Second")]
    price_label: String,
    #[schema(example = 2.15)]
    cost_usd: Option<f64>,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct ApiProviderCostEstimates {
    #[schema(example = "IBM Quantum")]
    provider: String,
    #[schema(example = "success")]
    status: String,
    estimates: Vec<ApiEstimatedCost>,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct EstimateCircuitResponse {
    estimates: Vec<ApiProviderCostEstimates>,
}

#[utoipa::path(
    post,
    path = "/estimate",
    operation_id = "estimate_circuit",
    request_body = EstimateCircuitRequest,
    responses(
        (status = 200, description = "Circuit estimated successfully", body = EstimateCircuitResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "estimator",
)]
pub(crate) async fn handler(
    Json(body): Json<EstimateCircuitRequest>,
) -> Result<Json<EstimateCircuitResponse>, ApiError> {
    let circuit = circuit::from_json(&body.circuit)?;
    let shots = body.shots.unwrap_or(1024);
    let request = EstimationRequest::new(circuit, shots);

    let response =
        tokio::task::spawn_blocking(move || qctidy_facade::estimate(&request, &EstimatorAdapter))
            .await
            .map_err(|error| ApiError::Internal(error.to_string()))?
            .map_err(|error| ApiError::Internal(error.to_string()))?;

    let api_estimates = response
        .estimation()
        .costs
        .into_iter()
        .map(|p| ApiProviderCostEstimates {
            provider: p.provider,
            status: p.status,
            estimates: p
                .estimates
                .into_iter()
                .map(|e| ApiEstimatedCost {
                    provider: e.provider,
                    plan_name: e.plan_name,
                    price_label: e.price_label,
                    cost_usd: e.cost_usd,
                })
                .collect(),
        })
        .collect();

    Ok(Json(EstimateCircuitResponse {
        estimates: api_estimates,
    }))
}
