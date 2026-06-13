"""
Equivalent of HttpInterceptor.kt

FastAPI middleware that:
1. Extracts multi-tenant user context from inbound request headers.
2. Injects tenantId into the logging context (via contextvars, async-safe).
3. Annotates the active OpenTelemetry span with tenant.id.
4. Logs "<module.function> http call for user <userName>" per request.
5. Resets context after response (cleanup).

In Quarkus this was a JAX-RS ContainerRequestFilter + ContainerResponseFilter.
In FastAPI the natural equivalent is Starlette BaseHTTPMiddleware.
"""

import logging

from fastapi import FastAPI
from opentelemetry import trace
from starlette.middleware.base import BaseHTTPMiddleware, RequestResponseEndpoint
from starlette.requests import Request
from starlette.responses import Response

from callee_service.extensions import user_context

logger = logging.getLogger("HttpInterceptor")


class HttpInterceptorMiddleware(BaseHTTPMiddleware):
    async def dispatch(self, request: Request, call_next: RequestResponseEndpoint) -> Response:
        user_context.set_context_from_request(request)
        _configure_tracing()

        route = request.scope.get("route")
        route_name = route.name if route else request.url.path
        logger.info("%s http call for user %s", route_name, user_context.get_user_name())

        try:
            response = await call_next(request)
        finally:
            user_context.reset_context()

        return response


def _configure_tracing() -> None:
    span = trace.get_current_span()
    if span.is_recording():
        span.set_attribute("tenant.id", user_context.get_tenant_id())


def register_http_interceptor(app: FastAPI) -> None:
    app.add_middleware(HttpInterceptorMiddleware)
