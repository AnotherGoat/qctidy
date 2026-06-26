#[utoipa::path(
    get,
    path = "/health",
    operation_id = "get_health",
    responses((status = 200, description = "Server is healthy")),
    tag = "health",
)]
pub(crate) async fn handler() -> &'static str {
    "ok"
}
