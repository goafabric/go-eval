mod controller;
#[allow(dead_code)]
mod extensions;
mod logic;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::http::header::CONTENT_TYPE;
use axum::routing::get;
use tracing::info;

use controller::callee_controller::CalleeController;
use extensions::health_controller;
use extensions::telemetry;
use logic::callee_logic::CalleeLogic;

const DEFAULT_PORT: u16 = 50900;

const INDEX_HTML: &str = include_str!("../resources/static/index.html");
const BOOTSTRAP_CSS: &str = include_str!("../resources/static/css/bootstrap.min.css");
const FAVICON: &[u8] = include_bytes!("../resources/static/favicon.ico");

fn static_resources() -> Router {
    Router::new()
        .route("/", get(|| async { ([(CONTENT_TYPE, "text/html; charset=utf-8")], INDEX_HTML) }))
        .route("/css/bootstrap.min.css", get(|| async { ([(CONTENT_TYPE, "text/css")], BOOTSTRAP_CSS) }))
        .route("/favicon.ico", get(|| async { ([(CONTENT_TYPE, "image/x-icon")], FAVICON) }))
}

#[tokio::main]
async fn main() {
    let tracer_provider = telemetry::init();

    let callee_logic = Arc::new(CalleeLogic);
    let callee_controller = Arc::new(CalleeController::new(callee_logic));

    let app = callee_controller
        .routes()
        .merge(health_controller::routes())
        .merge(static_resources());

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
