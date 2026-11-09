use axum::Router;
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use axum::routing::get;

const UP: &str = r#"{"status":"UP","checks":[]}"#;

/// GET /actuator/health, /actuator/health/liveness, /actuator/health/readiness
pub fn routes() -> Router {
    Router::new()
        .route("/actuator/health", get(up))
        .route("/actuator/health/liveness", get(up))
        .route("/actuator/health/readiness", get(up))
}

async fn up() -> impl IntoResponse {
    ([(CONTENT_TYPE, "application/json")], UP)
}

#[cfg(test)]
mod tests {
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    use super::*;

    async fn get(path: &str) -> (StatusCode, String) {
        let res = routes()
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = res.status();
        let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn health_endpoints_are_up() {
        for path in [
            "/actuator/health",
            "/actuator/health/liveness",
            "/actuator/health/readiness",
        ] {
            let (status, body) = get(path).await;
            assert_eq!(status, 200, "{path}");
            assert_eq!(body, UP, "{path}");
        }
    }

    #[tokio::test]
    async fn unknown_health_path_is_not_found() {
        let (status, _) = get("/actuator/health/nope").await;
        assert_eq!(status, 404);
    }
}
