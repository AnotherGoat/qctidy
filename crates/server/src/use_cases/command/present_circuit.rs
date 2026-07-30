use axum::extract::Json;
use axum::http::StatusCode;
use axum::http::header;
use axum::response::IntoResponse;
use serde::Deserialize;
use utoipa::ToSchema;

use qctidy_facade::PresentationRequest;
use qctidy_ports::PresentationFormat;
use qctidy_presenter::GraphvizPresenter;

use crate::circuit;
use crate::error::ApiError;
use crate::schema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct PresentCircuitRequest {
    #[schema(value_type = schema::Circuit, example = schema::example_circuit)]
    circuit: serde_json::Value,
    #[schema(value_type = schema::PresentationFormatName, example = "graphviz_png")]
    format: String,
    #[schema(example = 300)]
    dpi: Option<u32>,
}

#[utoipa::path(
    post,
    path = "/present",
    operation_id = "present_circuit",
    request_body = PresentCircuitRequest,
    responses(
        (status = 200, description = "Circuit presented successfully", content_type = "image/png", body = Vec::<u8>),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "presentation",
)]
pub(crate) async fn handler(
    Json(body): Json<PresentCircuitRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let circ = circuit::from_json(&body.circuit)?;

    let (format, media_type) = match body.format.to_lowercase().as_str() {
        "graphviz_gv" | "gv" => (PresentationFormat::GraphvizGv, "text/plain; charset=utf-8"),
        "graphviz_png" | "png" => (PresentationFormat::GraphvizPng, "image/png"),
        "graphviz_svg" | "svg" => (PresentationFormat::GraphvizSvg, "image/svg+xml"),
        other => {
            return Err(ApiError::BadRequest(format!(
                "Unknown presentation format: {other}"
            )));
        }
    };

    let request = PresentationRequest::new(circ, format, body.dpi);
    let response = qctidy_facade::present(&request, &GraphvizPresenter)
        .map_err(|error| ApiError::Internal(error.to_string()))?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, media_type)],
        response.bytes().to_vec(),
    ))
}
