mod docs;
mod error;
mod schema;
mod use_cases;

#[cfg(any(
    feature = "converter-cbor",
    feature = "converter-json",
    feature = "converter-msgpack",
    feature = "converter-xml",
))]
mod circuit;

use axum::Router;
use axum::response::Redirect;
use axum::routing::get;
use std::env;
use tokio::net::TcpListener;
use tokio::signal;
use utoipa_swagger_ui::SwaggerUi;

fn build_router() -> Router {
    #[cfg_attr(
        not(any(
            feature = "converter-cbor",
            feature = "converter-json",
            feature = "converter-msgpack",
            feature = "converter-xml",
            any(feature = "codegen-qiskit", feature = "codegen-openqasm3"),
            feature = "presenter-graphviz",
        )),
        allow(unused_mut)
    )]
    let mut app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/docs") }))
        .route("/health", get(use_cases::query::health::handler))
        .route("/features", get(use_cases::query::features::handler))
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", docs::build()));

    #[cfg(any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ))]
    {
        use axum::routing::post;

        app = app
            .route(
                "/display",
                post(use_cases::command::display_circuit::handler),
            )
            .route(
                "/simplify",
                post(use_cases::command::simplify_circuit::handler),
            )
            .route(
                "/convert",
                post(use_cases::command::convert_circuit::handler),
            );
    }

    #[cfg(all(
        any(feature = "codegen-qiskit", feature = "codegen-openqasm3"),
        any(
            feature = "converter-cbor",
            feature = "converter-json",
            feature = "converter-msgpack",
            feature = "converter-xml",
        ),
    ))]
    {
        use axum::routing::post;

        app = app.route("/codegen", post(use_cases::command::generate_code::handler));
    }

    #[cfg(all(
        feature = "presenter-graphviz",
        any(
            feature = "converter-cbor",
            feature = "converter-json",
            feature = "converter-msgpack",
            feature = "converter-xml",
        ),
    ))]
    {
        use axum::routing::post;

        app = app.route(
            "/present",
            post(use_cases::command::present_circuit::handler),
        );
    }

    app
}

#[expect(clippy::print_stderr)]
#[tokio::main]
async fn main() {
    let app = build_router();

    let host = env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = env::var("API_PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3000);
    let addr = format!("{host}:{port}");

    eprintln!("server listening on {addr}");

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .expect("Server error");
}

#[cfg(test)]
mod server_tests;
