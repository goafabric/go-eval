import logging
import logging.config
from pathlib import Path

import uvicorn
from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from opentelemetry import trace
# from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
# from opentelemetry.instrumentation.fastapi import FastAPIInstrumentor
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace import TracerProvider

from callee_service.config import settings
from callee_service.controller.callee_controller import router as callee_router
# ---------------------------------------------------------------------------
# Logging — includes tenantId in every log record via a custom filter
# ---------------------------------------------------------------------------
from callee_service.extensions import user_context
from callee_service.extensions.exception_handler import register_exception_handlers
from callee_service.extensions.http_interceptor import register_middleware
from callee_service.extensions.monitoring_extension import register_monitoring


class TenantIdFilter(logging.Filter):
    def filter(self, record: logging.LogRecord) -> bool:
        record.tenant_id = user_context.get_tenant_id()
        return True


logging.basicConfig(
    level=settings.log_level.upper(),
    format="%(asctime)s %(levelname)-5s tenantId=%(tenant_id)s [%(name)s] %(message)s",
)
for handler in logging.root.handlers:
    handler.addFilter(TenantIdFilter())

logger = logging.getLogger(__name__)

# ---------------------------------------------------------------------------
# OpenTelemetry tracing
# ---------------------------------------------------------------------------


def _setup_tracing() -> None:
    if not settings.otel_enabled:
        return
    resource = Resource.create({"service.name": settings.app_name})
    provider = TracerProvider(resource=resource)
    #exporter = OTLPSpanExporter(endpoint=settings.otel_exporter_otlp_endpoint, insecure=True)
    #provider.add_span_processor(BatchSpanProcessor(exporter))
    trace.set_tracer_provider(provider)


# ---------------------------------------------------------------------------
# Application factory
# ---------------------------------------------------------------------------


def create_app() -> FastAPI:
    _setup_tracing()

    app = FastAPI(
        title=settings.app_name,
        version="1.0.0",
        docs_url="/swagger-ui",   # equivalent of quarkus.swagger-ui
        redoc_url="/redoc",
        openapi_url="/openapi",
    )

    # Middleware (request/response filter — equiv. HttpInterceptor)
    register_middleware(app)

    # Exception handlers (equiv. ExceptionHandler)
    register_exception_handlers(app)

    # Routers
    app.include_router(callee_router)

    # Monitoring: health endpoints + Prometheus metrics
    register_monitoring(app)

    # Static files — serves index.html at / (equiv. META-INF/resources)
    static_dir = Path(__file__).parent / "static"
    app.mount("/", StaticFiles(directory=static_dir, html=True), name="static")

    # Auto-instrument FastAPI spans with OTel
    #FastAPIInstrumentor.instrument_app(app)

    return app


app = create_app()

if __name__ == "__main__":
    uvicorn.run(
        "callee_service.main:app",
        host="0.0.0.0",
        port=settings.port,
        reload=False,
    )
