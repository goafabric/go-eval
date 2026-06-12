"""
Equivalent of ExceptionHandler.kt

Global exception handlers registered on the FastAPI app.
- ValueError / AssertionError  -> 412 Precondition Failed  (equiv. IllegalArgumentException / IllegalStateException)
- All other exceptions         -> 500 Internal Server Error
"""

import logging

from fastapi import FastAPI, Request
from fastapi.responses import PlainTextResponse

logger = logging.getLogger(__name__)


def register_exception_handlers(app: FastAPI) -> None:
    @app.exception_handler(ValueError)
    async def value_error_handler(request: Request, exc: ValueError) -> PlainTextResponse:
        logger.error("ValueError: %s", exc, exc_info=exc)
        return PlainTextResponse(
            content=f"An error occured: {exc}",
            status_code=412,
        )

    @app.exception_handler(AssertionError)
    async def assertion_error_handler(request: Request, exc: AssertionError) -> PlainTextResponse:
        logger.error("AssertionError: %s", exc, exc_info=exc)
        return PlainTextResponse(
            content=f"An error occured: {exc}",
            status_code=412,
        )

    @app.exception_handler(Exception)
    async def generic_exception_handler(request: Request, exc: Exception) -> PlainTextResponse:
        logger.error("Unhandled exception: %s", exc, exc_info=exc)
        return PlainTextResponse(
            content=f"An error occured: {exc}",
            status_code=500,
        )
