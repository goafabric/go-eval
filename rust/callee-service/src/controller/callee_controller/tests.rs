use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use tower::ServiceExt;

use super::*;
use crate::logic::callee_logic::CalleeLogic;

async fn send(request: Request<Body>) -> (StatusCode, String) {
    let controller = Arc::new(CalleeController::new(Arc::new(CalleeLogic)));
    let res = controller.routes().oneshot(request).await.unwrap();
    let status = res.status();
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    (status, String::from_utf8(body.to_vec()).unwrap())
}

async fn get(path: &str) -> (StatusCode, String) {
    send(Request::get(path).body(Body::empty()).unwrap()).await
}

#[tokio::test]
async fn say_my_name() {
    let (status, body) = get("/callees/sayMyName?name=Heisenberg").await;
    assert_eq!(status, 200);
    assert_eq!(body, r#"{"id":"0","message":"Your name is: Heisenberg"}"#);
}

#[tokio::test]
async fn say_my_name_without_name_is_precondition_failed() {
    let (status, _) = get("/callees/sayMyName").await;
    assert_eq!(status, 412);
}

#[tokio::test]
async fn say_my_other_name() {
    let (status, body) = get("/callees/sayMyOtherName/Andreas").await;
    assert_eq!(status, 200);
    assert_eq!(body, r#"{"id":"0","message":"Your other name is: Andreas"}"#);
}

#[tokio::test]
async fn say_my_other_name_decodes_path() {
    let (status, body) = get("/callees/sayMyOtherName/Dr.%20Dre").await;
    assert_eq!(status, 200);
    assert_eq!(body, r#"{"id":"0","message":"Your other name is: Dr. Dre"}"#);
}

#[tokio::test]
async fn save() {
    let request = Request::post("/callees/save")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"id":null,"message":"hello"}"#))
        .unwrap();
    let (status, body) = send(request).await;
    assert_eq!(status, 200);
    assert_eq!(body, r#"{"id":"0","message":"Storing your message: hello"}"#);
}

#[tokio::test]
async fn save_without_message_is_rejected() {
    let request = Request::post("/callees/save")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"id":null}"#))
        .unwrap();
    let (status, _) = send(request).await;
    assert_eq!(status, 422);
}

#[tokio::test]
async fn invalid_user_info_header_is_precondition_failed() {
    let request = Request::get("/callees/sayMyOtherName/Andreas")
        .header("X-UserInfo", "!!!")
        .body(Body::empty())
        .unwrap();
    let (status, _) = send(request).await;
    assert_eq!(status, 412);
}

#[tokio::test]
async fn unknown_path_is_not_found() {
    let (status, _) = get("/callees/nope").await;
    assert_eq!(status, 404);
}

#[tokio::test]
async fn openapi_spec_is_served() {
    let (status, body) = get("/q/openapi").await;
    assert_eq!(status, 200);
    assert!(body.contains("/callees/sayMyName"), "{body}");
    assert!(body.contains("/callees/save"), "{body}");
}

#[tokio::test]
async fn swagger_ui_is_served() {
    let (status, body) = get("/q/swagger-ui/").await;
    assert_eq!(status, 200);
    assert!(body.contains("swagger-ui"));
}
