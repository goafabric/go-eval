
from pathlib import Path

from fastapi import APIRouter, FastAPI
from fastapi.responses import JSONResponse
from fastapi.staticfiles import StaticFiles
from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.instrumentation.fastapi import FastAPIInstrumentor
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from prometheus_fastapi_instrumentator import Instrumentator

from callee_service.config import settings

router = APIRouter(tags=["monitoring"], include_in_schema=False)


@router.get(settings.health_path)
async def health() -> JSONResponse:
    return JSONResponse({"status": "UP"})


@router.get(f"{settings.health_path}/liveness")
async def liveness() -> JSONResponse:
    return JSONResponse({"status": "UP"})


@router.get(f"{settings.health_path}/readiness")
async def readiness() -> JSONResponse:
    return JSONResponse({"status": "UP"})


def setup_tracing() -> None:
    if not settings.otel_enabled:
        return
    resource = Resource.create({"service.name": settings.app_name})
    provider = TracerProvider(resource=resource)
    exporter = OTLPSpanExporter(endpoint=settings.otel_exporter_otlp_endpoint, insecure=True)
    provider.add_span_processor(BatchSpanProcessor(exporter))
    trace.set_tracer_provider(provider)

def register_monitoring(app: FastAPI) -> None:
    """Wire up the monitoring router, Prometheus instrumentation, and static files."""
    app.include_router(router)
    Instrumentator().instrument(app).expose(app, endpoint=settings.prometheus_path)

    setup_tracing()
    FastAPIInstrumentor.instrument_app(app)

    static_dir = Path(__file__).parent.parent / "static"
    app.mount("/", StaticFiles(directory=static_dir, html=True), name="static")
