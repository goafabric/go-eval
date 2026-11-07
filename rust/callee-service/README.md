# callee-service-rust

Rust port of the Quarkus `callee-service`, built on plain `tokio` + `axum`.

Prometheus metrics and MCP from the Quarkus version are intentionally left out. OpenAPI/Swagger UI (`utoipa`) and OpenTelemetry trace export over OTLP/gRPC are included.

## Run
    cargo run            # listens on 0.0.0.0:50900, override with PORT=...
    cargo test

## Docker image
The binary is cross-compiled on the host to a static linux (musl) executable and copied into a `scratch` image, so no Rust toolchain is needed inside Docker. The image is ~3.5 MB.

    ./build-image.sh                 # builds goafabric/callee-service-rust:<version> for the host architecture and pushes it (docker login first)
    NO_PUSH=1 ./build-image.sh       # build only
    ./build-image.sh x86_64          # or for the other architecture (aarch64 | x86_64)
    container run --name callee-service --rm -p 50900:50900 goafabric/callee-service-rust:1.0.0
    docker run --name callee-service --rm -p 50900:50900 goafabric/callee-service-rust:1.0.0


## Configuration
Environment variables only:
- `PORT`: listen port (default `50900`)
- `OTEL_EXPORTER_OTLP_ENDPOINT`: OTLP/gRPC collector for traces (default `http://localhost:4317`). `OTEL_EXPORTER_OTLP_HEADERS` and `OTEL_EXPORTER_OTLP_TIMEOUT` also work.

## Structure
| Kotlin                                  | Rust                                  |
|-----------------------------------------|---------------------------------------|
| `Application.kt`                        | `src/main.rs`                         |
| `controller/CalleeController.kt`        | `src/controller/callee_controller.rs` |
| (health via SmallRye)                   | `src/controller/health_controller.rs` |
| `controller/dto/Callee.kt`              | `src/controller/dto/callee.rs`        |
| `logic/CalleeLogic.kt`                  | `src/logic/callee_logic.rs`           |
| `extensions/ExceptionHandler.kt`        | `src/extensions/exception_handler.rs` |
| `extensions/HttpInterceptor.kt`         | `src/extensions/http_interceptor.rs`  |
| `extensions/UserContext.kt`             | `src/extensions/user_context.rs`      |
| (quarkus-opentelemetry)                 | `src/extensions/telemetry.rs`         |
| `META-INF/resources`                    | `resources/static` (embedded)         |

## Endpoints
- `GET  /callees/sayMyName?name=Heisenberg`
- `GET  /callees/sayMyOtherName/{name}`
- `POST /callees/save`
- `GET  /actuator/health`, `/actuator/health/liveness`, `/actuator/health/readiness`
- `GET  /q/swagger-ui`, `/q/openapi`
- `GET  /` landing page
         
# memory usage
- rust: 4MB
- quarkus native: 17GB
- go: 17 GB