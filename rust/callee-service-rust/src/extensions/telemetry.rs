use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, fmt};

const SERVICE_NAME: &str = "callee-service";

/// Sets up logging and exports `tracing` spans as OpenTelemetry traces over OTLP/gRPC.
/// The endpoint comes from `OTEL_EXPORTER_OTLP_ENDPOINT` (default http://localhost:4317).
/// Call `shutdown` on the returned provider to flush pending spans.
pub fn init() -> SdkTracerProvider {
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

    // the exporter's connection errors (no collector running) stay out of the log, like in the Quarkus service
    let log_filter = Targets::new()
        .with_default(LevelFilter::INFO)
        .with_target("opentelemetry", LevelFilter::OFF)
        .with_target("opentelemetry_sdk", LevelFilter::OFF)
        .with_target("opentelemetry_otlp", LevelFilter::OFF);

    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(log_filter))
        .with(tracing_opentelemetry::layer().with_tracer(provider.tracer(SERVICE_NAME)))
        .init();
    provider
}
