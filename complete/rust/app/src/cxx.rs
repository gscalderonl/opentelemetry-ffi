use opentelemetry::{trace::Tracer, KeyValue};
use opentelemetry_otlp::ExporterConfig;
use opentelemetry_otlp::TraceExporter;
use opentelemetry_otlp::WithCompression;
use opentelemetry_otlp::WithTimeout;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_otlp::WithTracesConfig;
use opentelemetry_otlp::WithRetryConfig;
use opentelemetry_otlp::WithBatchConfig;
use opentelemetry_otlp::WithMetadata;
use opentelemetry_jaeger::ThriftCompactProtocolEncoder;
use std::time::Duration;
use std::env;

#[no_mangle]
pub extern "C" fn run_trace() {
    // Configure the OTLP exporter to Jaeger
    let exporter = TraceExporter::new()
        .with_endpoint("http://localhost:4317")
        .with_timeout(Duration::from_secs(3))
        .with_export_config(|c| {
            c.with_traces_config(|tc| {
                tc.with_batch_max_size(512)
                    .with_max_export_batch_size(4096)
                    .with_scheduled_delay(Duration::from_secs(3))
                    .with_scheduled_batch_delay(Duration::from_secs(5))
                    .with_retry_config(|rc| {
                        rc.with_max_retry_delay(Duration::from_secs(30))
                            .with_initial_retry_delay(Duration::from_secs(2))
                            .with_jitter(false)
                            .with_retry_on_status(vec![tonic::Code::Unavailable as u32])
                    })
            })
        })
        .with_exporter_config(|c| {
            c.with_metadata("service_name", "rust_app")
                .with_metadata("service_version", env!("CARGO_PKG_VERSION"))
                .with_metadata("exporter_kind", "jaeger")
                .with_encoding(ThriftCompactProtocolEncoder::default())
        })
        .with_compression();

    // Initialize the OpenTelemetry tracer with the OTLP exporter
    let tracer = opentelemetry::global::tracer("rust_app")
        .with_batch_exporter(exporter)
        .with_batch_buffer_config(Default::default())
        .init();

    // Create and instrument spans as needed
    let span = tracer.start("rust_span");
    span.set_attribute(KeyValue::new("key", "value"));
    span.end();
}
