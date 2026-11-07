# AGENTS.md

Rust port of the Quarkus/Kotlin `callee-service` (in the parent directory). Plain `tokio` + `axum`, no heavier frameworks.
Prometheus metrics and MCP are intentionally out of scope for now. OpenAPI/Swagger UI and OpenTelemetry trace export are included.

## Commands
- Run: `cargo run` (listens on `0.0.0.0:50900`, override with `PORT=...`)
- Test: `cargo test`
- Release build: `cargo build --release`
- Docker image: `./build-image.sh` cross-compiles a static musl binary (linker `rust-lld`, see `.cargo/config.toml`) and packs it into a `scratch` image (`Dockerfile`). The build context is only `dist/`. Don't add dependencies that need a C compiler or system libraries (e.g. `openssl`), or the cross build breaks.

- WASI component: `cargo rustc --lib --release --target wasm32-wasip2 --crate-type cdylib`, run with `wasmtime serve -Scli --addr 127.0.0.1:50900 target/wasm32-wasip2/release/callee_service.wasm`.
- Wasm in the local kind cluster: `./deploy-kind-wasm.sh` (native wasm via the containerd runwasi wasmtime shim, see README).

Keep the build warning-free, for the native and the `wasm32-wasip2` target.

## WASI target
`target_os = "wasi"` gets a different entry point and telemetry, everything else is shared.
- `lib.rs`: holds the module tree and `app()` (the router) for both. Its wasi `main` is `#[wstd_axum::http_server]` and returns the router, the host serves it (invoked once per request).
- `main.rs`: native entry only (`TcpListener`), `no_main` on wasi. The wasi component is built from the lib as `cdylib` so it exports only `wasi:http/incoming-handler`; a bin also exports `wasi:cli/run`, and the runwasi shim picks the first of the two it finds, so it would treat the component as a command and fail to link `wasi:http`.
- `telemetry.rs`: wasi only sets up logging (no OTLP/gRPC exporter, no batch threads).
- `Cargo.toml`: `axum` runs with `default-features = false`; `http1`/`tokio`, `tokio` networking and `opentelemetry-otlp` are only in the `cfg(not(target_os = "wasi"))` table. Don't add `axum` or `tokio` features that need sockets or threads to the shared table, and no `hyper`/`tonic`/`reqwest` there.
- `cargo test` only runs natively.
- `deploy-kind-wasm.sh` / `Dockerfile.wasm` / `k8s/callee-service-wasm.yaml`: the image is `scratch` + the `.wasm` and is labelled with the node's platform (not `wasi/wasm`), otherwise the node's containerd refuses to pull it ("no match for platform"). The script installs the shim into the kind node once and edits its `/etc/containerd/config.toml`, a recreated cluster needs the script run again.

## Structure
The layout mirrors the Kotlin service, so keep the same split when adding code:

- `src/main.rs`: entry point (`Application.kt`). Wires logic and controller, serves static files, starts the server.
- `src/controller/`: HTTP layer. `callee_controller.rs` has the handlers and the `routes()` router; `health_controller.rs` has the health endpoints. `dto/` has the request and response types.
- `src/logic/`: business logic (`callee_logic.rs`). It has no HTTP or axum dependencies.
- `src/extensions/`: cross-cutting concerns.
  - `exception_handler.rs`: `ServiceError` and its `IntoResponse` mapping to status codes.
  - `http_interceptor.rs`: axum middleware that builds the user context from headers, logs the call, and runs the rest of the request inside the context and a server span (continues an incoming `traceparent`).
  - `telemetry.rs`: sets up the `tracing` subscriber and the OTLP/gRPC trace export. `main` flushes it on shutdown.
  - `user_context.rs`: per-request context (tenant, organization, user name) kept in a tokio `task_local`.
- `resources/static/`: landing page, CSS and favicon, embedded into the binary with `include_str!` and `include_bytes!`.

Dependencies point one way: `controller` -> `logic`, and `controller` -> `extensions`. `logic` must not import from `controller`, except for the DTO types it already uses.

## Conventions
- Dependencies are injected through constructors and shared as `Arc` (see `CalleeController::new`).
- Every new business endpoint is added to the router in `CalleeController::routes()`, which carries the `http_interceptor::intercept` layer, so the user context and the `tenantId` log field are set.
- Return errors as `ServiceError` by returning `Result<_, ServiceError>` from the handler. Don't build error responses in handlers.
  - `IllegalArgument` and `IllegalState` map to 412.
  - `Internal` maps to 500.
  - The body is `An error occured: <msg>`. The typo is kept to match the Kotlin service.
- Use `tracing` for logging (`info!`, `error!`), not `println!`. Spans created with `tracing` are exported as OpenTelemetry spans.
- Every new endpoint needs a `#[utoipa::path(...)]` attribute and an entry in `ApiDoc` (`callee_controller.rs`), and new DTOs derive `ToSchema`. Query structs derive `IntoParams`.
- JSON field names are camelCase, which matches the Kotlin DTOs. `Option` fields serialize as `null`.
- axum's `Path` extractor percent-decodes path segments, so no manual decoding is needed.
- Unused items that mirror the Kotlin API (such as parts of `UserContext`) are allowed through `#[allow(dead_code)]` on the `extensions` module.

## Testing
- Unit and route tests live in `#[cfg(test)]` modules next to the code. Route tests call the `Router` with `tower::ServiceExt::oneshot`.
- The controller tests are in their own file, `src/controller/callee_controller/tests.rs`, declared with `#[cfg(test)] mod tests;` in `callee_controller.rs`. Small modules (such as `user_context.rs`) keep their tests inline.
- The controller tests build the routes through `CalleeController::routes()` and send requests with `oneshot` (see the `send` and `get` helpers). Do the same for new endpoints.
- There are no separate ArchUnit-style tests or integration tests yet.

## Endpoints
- `GET  /callees/sayMyName?name=...`
- `GET  /callees/sayMyOtherName/{name}`
- `POST /callees/save` (JSON body `{"id": ..., "message": ...}`)
- `GET  /actuator/health`, `/actuator/health/liveness`, `/actuator/health/readiness` (always `{"status":"UP","checks":[]}`, same paths as the Quarkus service; not run through `http_interceptor` so probes need no headers)
- `GET  /q/swagger-ui`, `/q/openapi` (Swagger UI and the generated spec, not run through `http_interceptor`)
- `GET  /`, `/css/bootstrap.min.css`, `/favicon.ico` (static)

## Configuration
Environment variables only, there is no config file (the Quarkus `application.properties` equivalent).
- `PORT`: listen port (default `50900`)
- OpenTelemetry: `telemetry.rs` sets no endpoint in code, the OTLP exporter reads the standard variables.
  - `OTEL_EXPORTER_OTLP_ENDPOINT`: collector, OTLP/gRPC (default `http://localhost:4317`, same as Quarkus). In docker-compose use e.g. `http://host.docker.internal:4317`.
  - `OTEL_EXPORTER_OTLP_HEADERS`, `OTEL_EXPORTER_OTLP_TIMEOUT`: optional (auth header, timeout).
  - Without a collector the exporter fails silently, its errors are filtered from the log.

## Optional headers
- `X-TenantId`: defaults to `0`
- `X-OrganizationId`: defaults to `0`
- `X-Auth-Request-Preferred-Username`: defaults to `anonymous`
- `X-UserInfo`: base64url-encoded JSON. Its `preferred_username` takes precedence over the username header.
