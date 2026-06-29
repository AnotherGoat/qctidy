use axum::extract::Json;
use qsimplify_analyzer::AnalyzerAdapter;
use qsimplify_facade::AnalysisRequest;
use qsimplify_ports::DetailedAnalysisMetrics;
use qsimplify_ports::{AnalysisMode, AnalysisResult};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
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
    let response = qsimplify_facade::analyze(&request, &AnalyzerAdapter)
        .map_err(|error| ApiError::Internal(error.to_string()))?;
    let metrics = match response.result() {
        Simple(metrics) => metrics_to_json(metrics),
        Detailed(metrics) => detailed_metrics_to_json(metrics),
    };

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

pub(crate) fn metrics_to_json(metrics: qsimplify_ports::AnalysisMetrics) -> Value {
    json!({
        "qubit_count": metrics.qubit_count,
        "depth": metrics.depth,
        "x_count": metrics.x_count,
        "y_count": metrics.y_count,
        "z_count": metrics.z_count,
        "pauli_count": metrics.pauli_count,
        "hadamard_count": metrics.hadamard_count,
        "rotation_count": metrics.rotation_count,
        "square_root_count": metrics.square_root_count,
        "measure_count": metrics.measure_count,
        "swap_count": metrics.swap_count,
        "cx_count": metrics.cx_count,
        "gate_count": metrics.gate_count,
        "single_gate_count": metrics.single_gate_count,
        "controlled_gate_count": metrics.controlled_gate_count,
        "auxiliary_qubit_count": metrics.auxiliary_qubit_count,
        "gate_types_count": metrics.gate_types_count,
    })
}

pub(crate) fn detailed_metrics_to_json(metrics: DetailedAnalysisMetrics) -> Value {
    json!({
        "width": metrics.width,
        "depth": metrics.depth,
        "max_density": metrics.max_density,
        "average_density": metrics.average_density,
        "x_count": metrics.x_count,
        "y_count": metrics.y_count,
        "z_count": metrics.z_count,
        "pauli_count": metrics.pauli_count,
        "hadamard_count": metrics.hadamard_count,
        "initial_superposition_percent": metrics.initial_superposition_percent,
        "other_single_qubit_count": metrics.other_single_qubit_count,
        "single_qubit_count": metrics.single_qubit_count,
        "single_controlled_qubit_count": metrics.single_controlled_qubit_count,
        "swap_count": metrics.swap_count,
        "cnot_count": metrics.cnot_count,
        "cnot_qubit_percent": metrics.cnot_qubit_percent,
        "average_cnot": metrics.average_cnot,
        "max_cnot": metrics.max_cnot,
        "toffoli_count": metrics.toffoli_count,
        "toffoli_qubit_percent": metrics.toffoli_qubit_percent,
        "average_toffoli": metrics.average_toffoli,
        "max_toffoli": metrics.max_toffoli,
        "gate_count": metrics.gate_count,
        "controlled_gate_count": metrics.controlled_gate_count,
        "single_qubit_percent": metrics.single_qubit_percent,
        "measure_count": metrics.measure_count,
        "measure_percent": metrics.measure_percent,
        "auxiliary_percent": metrics.auxiliary_percent,
    })
}
