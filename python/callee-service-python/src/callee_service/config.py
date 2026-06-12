"""
Equivalent of application.properties

Uses pydantic-settings so values can be overridden via environment variables
or a .env file (useful for Docker / Kubernetes deployments).
"""

from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_file=".env", env_file_encoding="utf-8", extra="ignore")

    app_name: str = "callee-service"
    port: int = 50900

    # Health paths (Spring Boot Actuator-compatible)
    health_path: str = "/actuator/health"

    # Prometheus metrics path
    prometheus_path: str = "/actuator/prometheus"

    # OpenTelemetry OTLP endpoint
    otel_exporter_otlp_endpoint: str = "http://localhost:4317"
    otel_enabled: bool = True

    # Logging
    log_level: str = "INFO"


settings = Settings()
