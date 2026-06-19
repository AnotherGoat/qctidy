use axum::extract::Json;
use axum::http::StatusCode;
use axum::http::header;
use axum::response::IntoResponse;
use serde::Deserialize;
use utoipa::ToSchema;

use qsimplify::DiracFormat;
use qsimplify::PiFormat;
use qsimplify_facade::DisplayFormat;
use qsimplify_facade::DisplayRequest;

use crate::circuit;
use crate::error::ApiError;

#[derive(Deserialize, ToSchema)]
pub(crate) struct DisplayRequestBody {
    circuit: serde_json::Value,
    format: String,
    pi_format: Option<String>,
    dirac_format: Option<String>,
}

#[utoipa::path(
    post,
    path = "/display",
    request_body = DisplayRequestBody,
    responses(
        (status = 200, description = "Circuit displayed successfully", content_type = "text/plain", body = String),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "circuit",
)]
pub(crate) async fn handler(
    Json(body): Json<DisplayRequestBody>,
) -> Result<impl IntoResponse, ApiError> {
    let circ = circuit::from_json(&body.circuit)?;

    let format = match body.format.to_lowercase().as_str() {
        "graph" => DisplayFormat::Graph,
        "grid" => DisplayFormat::Grid,
        "matrix" => DisplayFormat::Matrix,
        "circuit" => DisplayFormat::Circuit,
        other => {
            return Err(ApiError::BadRequest(format!(
                "Unknown display format: {other}"
            )));
        }
    };

    let pi_format = body
        .pi_format
        .as_deref()
        .map(|value| match value.to_lowercase().as_str() {
            "lowercase" => Ok(PiFormat::Lowercase),
            "uppercase" => Ok(PiFormat::Uppercase),
            "fancy" => Ok(PiFormat::Fancy),
            other => Err(ApiError::BadRequest(format!("Unknown pi format: {other}"))),
        })
        .transpose()?;

    let dirac_format = body
        .dirac_format
        .as_deref()
        .map(|value| match value.to_lowercase().as_str() {
            "ascii" => Ok(DiracFormat::Ascii),
            "fancy" => Ok(DiracFormat::Fancy),
            "none" => Ok(DiracFormat::None),
            other => Err(ApiError::BadRequest(format!(
                "Unknown dirac format: {other}"
            ))),
        })
        .transpose()?;

    let request = DisplayRequest::new(circ, format, pi_format, dirac_format);
    let response = qsimplify_facade::display(&request)
        .map_err(|error| ApiError::Internal(error.to_string()))?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        response.text().to_owned(),
    ))
}
