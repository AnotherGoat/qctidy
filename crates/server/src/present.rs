use axum::extract::Json;
use axum::http::StatusCode;
use axum::http::header;
use axum::response::IntoResponse;
use serde::Deserialize;
use utoipa::ToSchema;

use qsimplify_facade::PresentationRequest;
use qsimplify_ports::PresentationFormat;
use qsimplify_presenter::GraphvizPresenter;

use crate::circuit;
use crate::error::ApiError;

#[derive(Deserialize, ToSchema)]
pub(crate) struct PresentRequestBody {
    circuit: serde_json::Value,
    format: String,
    dpi: Option<u32>,
}

#[utoipa::path(
    post,
    path = "/present",
    request_body = PresentRequestBody,
    responses(
        (status = 200, description = "Circuit presented successfully", content_type = "image/png", body = Vec::<u8>),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "presentation",
)]
pub(crate) async fn handler(
    Json(body): Json<PresentRequestBody>,
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
    let response = qsimplify_facade::present(&request, &GraphvizPresenter)
        .map_err(|error| ApiError::Internal(error.to_string()))?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, media_type)],
        response.bytes().to_vec(),
    ))
}
