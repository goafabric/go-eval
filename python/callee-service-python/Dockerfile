# Equivalent of Quarkus Jib container build
# Multi-stage: build with uv, run on slim Python image

FROM python:3.12-slim AS builder

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/local/bin/uv

WORKDIR /app

# Copy dependency manifests first (layer cache)
COPY pyproject.toml .
COPY src/ src/

# Install dependencies into a virtual env inside the image
RUN uv sync --no-dev

# ---------------------------------------------------------------------------
FROM python:3.12-slim AS runtime

WORKDIR /app

# Copy the venv and source from builder
COPY --from=builder /app/.venv /app/.venv
COPY --from=builder /app/src /app/src

ENV PATH="/app/.venv/bin:$PATH"
ENV PYTHONPATH="/app/src"

EXPOSE 50900

CMD ["uvicorn", "callee_service.main:app", "--host", "0.0.0.0", "--port", "50900", "--workers", "4"]