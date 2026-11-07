use std::future::Future;

use axum::extract::{MatchedPath, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use opentelemetry::global;
use opentelemetry_http::HeaderExtractor;
use tracing::field::Empty;
use tracing::{Instrument, Span, info, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use super::user_context::{UserContext, UserContextRecord};

/// Middleware: creates the user context from the headers, logs the call and runs the rest of the
/// request with the context bound to it, inside a server span that continues an incoming trace.
pub async fn intercept(request: Request, next: Next) -> Response {
    let context = match UserContext::create_context(request.headers()) {
        Ok(context) => context,
        Err(e) => return e.into_response(),
    };
    info!(
        tenantId = %context.tenant_id,
        "{} {} http call for user {} ",
        request.method(),
        request.uri().path(),
        context.user_name
    );
    let span = request_span(&context, &request);
    let response = run(context, span.clone(), next.run(request)).await;
    let status = response.status();
    span.record("http.response.status_code", status.as_u16());
    if status.is_server_error() {
        span.record("otel.status_code", "ERROR");
    }
    response
}

/// Runs the handler with the context bound to it and the span (carrying tenantId) attached to the logs,
/// the context is removed again once the handler completes.
pub async fn run<F: Future>(context: UserContextRecord, span: Span, handler: F) -> F::Output {
    UserContext::scope(context, handler.instrument(span)).await
}

fn request_span(context: &UserContextRecord, request: &Request) -> Span {
    let method = request.method().as_str();
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map_or_else(|| request.uri().path(), MatchedPath::as_str);
    let span = info_span!(
        "request",
        otel.name = %format!("{method} {route}"),
        otel.kind = "server",
        otel.status_code = Empty,
        tenantId = %context.tenant_id,
        http.request.method = %method,
        http.route = %route,
        http.response.status_code = Empty,
    );
    let parent = global::get_text_map_propagator(|p| p.extract(&HeaderExtractor(request.headers())));
    let _ = span.set_parent(parent);
    span
}
