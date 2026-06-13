
from pathlib import Path

from fastapi import APIRouter, FastAPI
from fastapi.responses import JSONResponse
from fastapi.staticfiles import StaticFiles
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


def register_monitoring(app: FastAPI) -> None:
    """Wire up the monitoring router, Prometheus instrumentation, and static files."""
    app.include_router(router)
    Instrumentator().instrument(app).expose(app, endpoint=settings.prometheus_path)

    static_dir = Path(__file__).parent.parent / "static"
    app.mount("/", StaticFiles(directory=static_dir, html=True), name="static")
