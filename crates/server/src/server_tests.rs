use axum::body::Body;
use axum::body::to_bytes;
use axum::http::Request;
use axum::http::StatusCode;
use serde_json::json;
use tower::ServiceExt;

use super::*;

#[tokio::test]
async fn root_redirects_to_docs() {
    let app = build_router();
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 308);
    assert_eq!(
        response
            .headers()
            .get("location")
            .unwrap()
            .to_str()
            .unwrap(),
        "/docs"
    );
}

#[tokio::test]
async fn health_returns_200() {
    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn features_returns_200() {
    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/features")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn docs_redirect_returns_303() {
    let app = build_router();
    let response = app
        .oneshot(Request::builder().uri("/docs").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 303);
}

#[cfg(all(feature = "converter-json", feature = "converter-xml"))]
#[tokio::test]
async fn convert_returns_json_circuit_inline_when_target_is_json() {
    let app = build_router();
    let request_body = json!({
        "source_format": "xml",
        "target_format": "json",
        "circuit": {
            "encoding": "base64",
            "data": "PGNpcmN1aXQgdmVyc2lvbj0iMSIgcXViaXRfY291bnQ9IjEiPjxnYXRlIHR5cGU9ImgiIHF1Yml0PSIwIi8+PC9jaXJjdWl0Pg=="
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/convert")
                .header("content-type", "application/json")
                .body(Body::from(request_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["source_format"], "xml");
    assert_eq!(body["target_format"], "json");
    assert_eq!(body["circuit"]["qubit_count"], 1);
    assert!(body["circuit"]["operations"].is_array());
}

#[cfg(any(
    feature = "converter-cbor",
    feature = "converter-json",
    feature = "converter-msgpack",
    feature = "converter-xml",
))]
#[tokio::test]
async fn convert_rejects_matching_source_and_target_formats() {
    let app = build_router();
    let request_body = json!({
        "source_format": "json",
        "target_format": "json",
        "circuit": {
            "version": 1,
            "qubit_count": 1,
            "operations": []
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/convert")
                .header("content-type", "application/json")
                .body(Body::from(request_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[cfg(all(feature = "converter-cbor", feature = "converter-json"))]
#[tokio::test]
async fn convert_returns_base64_circuit_envelope_when_target_is_binary() {
    let app = build_router();
    let request_body = json!({
        "source_format": "json",
        "target_format": "cbor",
        "circuit": {
            "version": 1,
            "qubit_count": 1,
            "operations": [
                { "gate": "h", "qubit": 0 }
            ]
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/convert")
                .header("content-type", "application/json")
                .body(Body::from(request_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["source_format"], "json");
    assert_eq!(body["target_format"], "cbor");
    assert_eq!(body["circuit"]["encoding"], "base64");
    assert!(
        body["circuit"]["data"]
            .as_str()
            .is_some_and(|data| !data.is_empty())
    );
}
