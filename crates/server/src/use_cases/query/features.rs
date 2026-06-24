use axum::extract::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub(crate) struct FeaturesResponse {
    pub features: Vec<&'static str>,
}

#[utoipa::path(
    get,
    path = "/features",
    responses((status = 200, description = "List of enabled features", body = FeaturesResponse)),
    tag = "health",
)]
pub(crate) async fn handler() -> Json<FeaturesResponse> {
    Json(FeaturesResponse {
        features: vec![
            #[cfg(feature = "analyzer")]
            "analyzer",
            #[cfg(feature = "codegen-qiskit")]
            "codegen-qiskit",
            #[cfg(feature = "codegen-openqasm3")]
            "codegen-openqasm3",
            #[cfg(feature = "converter-cbor")]
            "converter-cbor",
            #[cfg(feature = "converter-json")]
            "converter-json",
            #[cfg(feature = "converter-msgpack")]
            "converter-msgpack",
            #[cfg(feature = "converter-xml")]
            "converter-xml",
            #[cfg(feature = "presenter-graphviz")]
            "presenter-graphviz",
            #[cfg(feature = "estimator")]
            "estimator",
        ],
    })
}
