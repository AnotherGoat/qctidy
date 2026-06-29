use axum::extract::Json;
use qsimplify::Graph;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use qsimplify_estimator::BackendProfile;
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
    let circ = circuit::from_json(&body.circuit)?;
    let graph = Graph::from(circ.as_ref());
    let shots = body.shots.unwrap_or(1024);

    let profile = BackendProfile::default();

    let heuristic_time = qsimplify_estimator::estimate_execution_time(&graph, shots, &profile);

    // false = intentar usar caché
    let pricing = qsimplify_estimator::get_pricing_data(false).await;

    let estimates = qsimplify_estimator::calculate_costs(
        &pricing,
        heuristic_time,
        shots,
        profile.base_time_ns,
    );

    let api_estimates = estimates
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

    Ok(Json(EstimateCircuitResponse { estimates: api_estimates }))
}
