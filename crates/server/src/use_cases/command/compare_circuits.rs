use axum::extract::Json;
use qctidy_analyzer::AnalyzerAdapter;
use qctidy_facade::ComparisonRequest;
use qctidy_ports::DeltaAnalysisMetrics;
use qctidy_ports::DeltaAnalysisResult;
use qctidy_ports::DeltaDetailedAnalysisMetrics;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
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
    let response = qctidy_facade::compare(&request, &AnalyzerAdapter)
        .map_err(|error| ApiError::Internal(error.to_string()))?;
    let metrics = match response.result() {
        Simple(metrics) => delta_to_json(&metrics),
        Detailed(metrics) => detailed_delta_to_json(&metrics),
    };

    Ok(Json(CompareCircuitsResponse { metrics }))
}

fn delta_to_json(metrics: &DeltaAnalysisMetrics) -> Value {
    let mut json = Map::new();
    insert_delta(&mut json, "qubit_count", metrics.qubit_count);
    insert_delta(&mut json, "depth", metrics.depth);
    insert_delta(&mut json, "x_count", metrics.x_count);
    insert_delta(&mut json, "y_count", metrics.y_count);
    insert_delta(&mut json, "z_count", metrics.z_count);
    insert_delta(&mut json, "pauli_count", metrics.pauli_count);
    insert_delta(&mut json, "hadamard_count", metrics.hadamard_count);
    insert_delta(&mut json, "rotation_count", metrics.rotation_count);
    insert_delta(&mut json, "square_root_count", metrics.square_root_count);
    insert_delta(&mut json, "measure_count", metrics.measure_count);
    insert_delta(&mut json, "swap_count", metrics.swap_count);
    insert_delta(&mut json, "cx_count", metrics.cx_count);
    insert_delta(&mut json, "gate_count", metrics.gate_count);
    insert_delta(&mut json, "single_gate_count", metrics.single_gate_count);
    insert_delta(
        &mut json,
        "controlled_gate_count",
        metrics.controlled_gate_count,
    );
    insert_delta(
        &mut json,
        "auxiliary_qubit_count",
        metrics.auxiliary_qubit_count,
    );
    insert_delta(&mut json, "gate_types_count", metrics.gate_types_count);
    Value::Object(json)
}

fn detailed_delta_to_json(metrics: &DeltaDetailedAnalysisMetrics) -> Value {
    let mut json = Map::new();
    insert_delta(&mut json, "width", metrics.width);
    insert_delta(&mut json, "depth", metrics.depth);
    insert_delta(&mut json, "max_density", metrics.max_density);
    insert_delta(&mut json, "average_density", metrics.average_density);
    insert_delta(&mut json, "x_count", metrics.x_count);
    insert_delta(&mut json, "y_count", metrics.y_count);
    insert_delta(&mut json, "z_count", metrics.z_count);
    insert_delta(&mut json, "pauli_count", metrics.pauli_count);
    insert_delta(&mut json, "hadamard_count", metrics.hadamard_count);
    insert_delta(
        &mut json,
        "initial_superposition_percent",
        metrics.initial_superposition_percent,
    );
    insert_delta(
        &mut json,
        "other_single_qubit_count",
        metrics.other_single_qubit_count,
    );
    insert_delta(&mut json, "single_qubit_count", metrics.single_qubit_count);
    insert_delta(
        &mut json,
        "single_controlled_qubit_count",
        metrics.single_controlled_qubit_count,
    );
    insert_delta(&mut json, "swap_count", metrics.swap_count);
    insert_delta(&mut json, "cnot_count", metrics.cnot_count);
    insert_delta(&mut json, "cnot_qubit_percent", metrics.cnot_qubit_percent);
    insert_delta(&mut json, "average_cnot", metrics.average_cnot);
    insert_delta(&mut json, "max_cnot", metrics.max_cnot);
    insert_delta(&mut json, "toffoli_count", metrics.toffoli_count);
    insert_delta(
        &mut json,
        "toffoli_qubit_percent",
        metrics.toffoli_qubit_percent,
    );
    insert_delta(&mut json, "average_toffoli", metrics.average_toffoli);
    insert_delta(&mut json, "max_toffoli", metrics.max_toffoli);
    insert_delta(&mut json, "gate_count", metrics.gate_count);
    insert_delta(
        &mut json,
        "controlled_gate_count",
        metrics.controlled_gate_count,
    );
    insert_delta(
        &mut json,
        "single_qubit_percent",
        metrics.single_qubit_percent,
    );
    insert_delta(&mut json, "measure_count", metrics.measure_count);
    insert_delta(&mut json, "measure_percent", metrics.measure_percent);
    insert_delta(&mut json, "auxiliary_percent", metrics.auxiliary_percent);
    Value::Object(json)
}

fn insert_delta<T: Serialize>(json: &mut Map<String, Value>, key: &str, value: Option<T>) {
    if let Some(value) = value {
        json.insert(key.to_owned(), serde_json::to_value(value).unwrap());
    }
}
