mod controller;
#[allow(dead_code)]
mod extensions;
mod logic;

use std::sync::Arc;

use axum::Router;
use axum::http::header::CONTENT_TYPE;
use axum::routing::get;

use controller::callee_controller::CalleeController;
use controller::health_controller;
use logic::callee_logic::CalleeLogic;

const INDEX_HTML: &str = include_str!("../resources/static/index.html");
const BOOTSTRAP_CSS: &str = include_str!("../resources/static/css/bootstrap.min.css");
const FAVICON: &[u8] = include_bytes!("../resources/static/favicon.ico");

fn static_resources() -> Router {
    Router::new()
        .route("/", get(|| async { ([(CONTENT_TYPE, "text/html; charset=utf-8")], INDEX_HTML) }))
        .route("/css/bootstrap.min.css", get(|| async { ([(CONTENT_TYPE, "text/css")], BOOTSTRAP_CSS) }))
        .route("/favicon.ico", get(|| async { ([(CONTENT_TYPE, "image/x-icon")], FAVICON) }))
}

fn app() -> Router {
    let callee_logic = Arc::new(CalleeLogic);
    let callee_controller = Arc::new(CalleeController::new(callee_logic));

    callee_controller
        .routes()
        .merge(health_controller::routes())
        .merge(static_resources())
}

#[cfg(not(target_os = "wasi"))]
const DEFAULT_PORT: u16 = 50900;

#[cfg(not(target_os = "wasi"))]
#[tokio::main]
async fn main() {
    use std::net::SocketAddr;

    use tracing::info;

    let tracer_provider = extensions::telemetry::init();
    let app = app();

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_PORT);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("callee-service listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.expect("failed to bind");
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .expect("server error");

    let _ = tracer_provider.shutdown();
}

/// The wasi host (e.g. `wasmtime serve`) owns the socket and calls this component once per request.
#[cfg(target_os = "wasi")]
#[wstd_axum::http_server]
fn main() -> Router {
    extensions::telemetry::init();
    app()
}
