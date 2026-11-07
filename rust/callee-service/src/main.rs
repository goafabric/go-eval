// native entry point only, the wasi component is the lib (src/lib.rs)
#![cfg_attr(target_os = "wasi", no_main)]

#[cfg(not(target_os = "wasi"))]
const DEFAULT_PORT: u16 = 50900;

#[cfg(not(target_os = "wasi"))]
#[tokio::main]
async fn main() {
    use std::net::SocketAddr;

    use tracing::info;

    let tracer_provider = callee_service::extensions::telemetry::init();
    let app = callee_service::app();

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
