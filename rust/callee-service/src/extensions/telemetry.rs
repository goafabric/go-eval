use opentelemetry::global;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, fmt};

#[cfg(not(target_os = "wasi"))]
const SERVICE_NAME: &str = "callee-service";

/// The exporter's connection errors (no collector running) stay out of the log, like in the Quarkus service.
fn log_filter() -> Targets {
    Targets::new()
        .with_default(LevelFilter::INFO)
        .with_target("opentelemetry", LevelFilter::OFF)
        .with_target("opentelemetry_sdk", LevelFilter::OFF)
        .with_target("opentelemetry_otlp", LevelFilter::OFF)
}

/// Sets up logging and exports `tracing` spans as OpenTelemetry traces over OTLP/gRPC.
/// The endpoint comes from `OTEL_EXPORTER_OTLP_ENDPOINT` (default http://localhost:4317).
/// Call `shutdown` on the returned provider to flush pending spans.
#[cfg(not(target_os = "wasi"))]
pub fn init() -> opentelemetry_sdk::trace::SdkTracerProvider {
    use opentelemetry::trace::TracerProvider as _;
    use opentelemetry_otlp::SpanExporter;
    use opentelemetry_sdk::Resource;
    use opentelemetry_sdk::trace::SdkTracerProvider;

    global::set_text_map_propagator(TraceContextPropagator::new());
    let exporter = SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("failed to build OTLP exporter");
    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(Resource::builder().with_service_name(SERVICE_NAME).build())
        .build();
    global::set_tracer_provider(provider.clone());

    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(log_filter()))
        .with(tracing_opentelemetry::layer().with_tracer(provider.tracer(SERVICE_NAME)))
        .init();
    provider
}

/// Sets up logging only. A wasi component has no raw sockets and no background threads, so there is
/// no OTLP/gRPC export; an incoming `traceparent` is still read by the interceptor but not exported.
/// Safe to call once per request: a second call finds the subscriber already set and leaves it.
#[cfg(target_os = "wasi")]
pub fn init() {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let _ = tracing_subscriber::registry()
        .with(fmt::layer().with_filter(log_filter()))
        .try_init();
}
