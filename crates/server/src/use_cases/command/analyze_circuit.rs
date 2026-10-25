use axum::extract::Json;
use qctidy_analyzer::AnalyzerAdapter;
use qctidy_facade::AnalysisRequest;
use qctidy_ports::{AnalysisMode, AnalysisResult};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

use crate::circuit;
use crate::error::ApiError;
use crate::schema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct AnalyzeCircuitRequest {
    #[schema(value_type = schema::Circuit, example = schema::example_circuit)]
    circuit: Value,
    #[schema(example = "simple")]
    mode: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct AnalyzeCircuitResponse {
    metrics: Value,
}

#[utoipa::path(
    post,
    path = "/analyze",
    operation_id = "analyze_circuit",
    request_body = AnalyzeCircuitRequest,
    responses(
        (status = 200, description = "Circuit analyzed successfully", body = AnalyzeCircuitResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "analysis",
)]
pub(crate) async fn handler(
    Json(body): Json<AnalyzeCircuitRequest>,
) -> Result<Json<AnalyzeCircuitResponse>, ApiError> {
    use AnalysisResult::*;

    let request = AnalysisRequest::new(
        circuit::from_json(&body.circuit)?,
        parse_mode(body.mode.as_deref())?,
    );
    let response = qctidy_facade::analyze(&request, &AnalyzerAdapter);
    let metrics = match response.result() {
        Simple(metrics) => serde_json::to_value(metrics),
        Detailed(metrics) => serde_json::to_value(metrics),
    }
    .expect("metrics should serialize");

    Ok(Json(AnalyzeCircuitResponse { metrics }))
}

pub(crate) fn parse_mode(mode: Option<&str>) -> Result<AnalysisMode, ApiError> {
    match mode.unwrap_or("simple").to_lowercase().as_str() {
        "simple" => Ok(AnalysisMode::Simple),
        "detailed" => Ok(AnalysisMode::Detailed),
        other => Err(ApiError::BadRequest(format!(
            "Unknown analysis mode: {other}"
        ))),
    }
}
