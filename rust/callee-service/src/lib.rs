pub mod controller;
#[allow(dead_code)]
pub mod extensions;
pub mod logic;

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

pub fn app() -> Router {
    let callee_logic = Arc::new(CalleeLogic);
    let callee_controller = Arc::new(CalleeController::new(callee_logic));

    callee_controller
        .routes()
        .merge(health_controller::routes())
        .merge(static_resources())
}

/// wasi entry point, built as a `cdylib` (see `deploy-kind-wasm.sh`) so the component only exports
/// `wasi:http/incoming-handler`. The host owns the socket and calls this once per request.
#[cfg(target_os = "wasi")]
#[wstd_axum::http_server]
fn main() -> Router {
    extensions::telemetry::init();
    app()
}
