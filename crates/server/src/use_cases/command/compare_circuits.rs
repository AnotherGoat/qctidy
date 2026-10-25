use axum::extract::Json;
use qctidy_analyzer::AnalyzerAdapter;
use qctidy_facade::ComparisonRequest;
use qctidy_ports::DeltaAnalysisResult;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

use crate::circuit;
use crate::error::ApiError;
use crate::schema;

#[derive(Deserialize, ToSchema)]
#[schema(example = example_compare_circuits_request)]
pub(crate) struct CompareCircuitsRequest {
    #[schema(value_type = schema::Circuit, example = schema::example_circuit)]
    old_circuit: Value,
    #[schema(value_type = schema::Circuit, example = schema::example_comparison_circuit)]
    new_circuit: Value,
    #[schema(example = "simple")]
    mode: Option<String>,
}

fn example_compare_circuits_request() -> Value {
    serde_json::json!({
        "old_circuit": schema::example_circuit(),
        "new_circuit": schema::example_comparison_circuit(),
        "mode": "simple"
    })
}

#[derive(Serialize, ToSchema)]
pub(crate) struct CompareCircuitsResponse {
    metrics: Value,
}

#[utoipa::path(
    post,
    path = "/compare",
    operation_id = "compare_circuits",
    request_body = CompareCircuitsRequest,
    responses(
        (status = 200, description = "Circuits compared successfully", body = CompareCircuitsResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "analysis",
)]
pub(crate) async fn handler(
    Json(body): Json<CompareCircuitsRequest>,
) -> Result<Json<CompareCircuitsResponse>, ApiError> {
    use DeltaAnalysisResult::*;

    let request = ComparisonRequest::new(
        circuit::from_json(&body.old_circuit)?,
        circuit::from_json(&body.new_circuit)?,
        super::analyze_circuit::parse_mode(body.mode.as_deref())?,
    );
    let response = qctidy_facade::compare(&request, &AnalyzerAdapter);
    let metrics = match response.result() {
        Simple(metrics) => serde_json::to_value(metrics),
        Detailed(metrics) => serde_json::to_value(metrics),
    }
    .expect("metrics should serialize");

    Ok(Json(CompareCircuitsResponse { metrics }))
}
