#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Log telemetry fields for an `OpenObserve` integration.
//!
//! Set `OTEL_EXPORTER_OTLP_ENDPOINT` to an OpenObserve OTLP base URL such as
//! `http://localhost:5080/api/default`. Set
//! `OTEL_EXPORTER_OTLP_HEADERS` to the required authorization header.

#[cfg(feature = "opentelemetry")]
use opentelemetry::KeyValue;
#[cfg(feature = "opentelemetry")]
use opentelemetry::global;
#[cfg(feature = "opentelemetry")]
use opentelemetry::trace::Span as _;
#[cfg(feature = "opentelemetry")]
use opentelemetry::trace::Tracer as _;
#[cfg(feature = "opentelemetry")]
use opentelemetry_otlp::SpanExporter;
#[cfg(feature = "opentelemetry")]
use opentelemetry_otlp::WithExportConfig;
#[cfg(feature = "opentelemetry")]
use opentelemetry_sdk::trace::SdkTracerProvider;

/// Creates a span with fields and logs events, for an OpenObserve endpoint.
#[cfg(feature = "opentelemetry")]
fn main() -> anyhow::Result<()> {
    let _ = tracing_subscriber::fmt().with_target(false).try_init();
    let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .or_else(|_| std::env::var("OPENOBSERVE_OTLP_ENDPOINT"));
    run_with_endpoint(endpoint.ok().as_deref())
}

/// Exports a sample span when an endpoint is configured, or prints setup help.
#[cfg(feature = "opentelemetry")]
fn run_with_endpoint(endpoint: Option<&str>) -> anyhow::Result<()> {
    let Some(endpoint) = endpoint else {
        println!(
            "Set OTEL_EXPORTER_OTLP_ENDPOINT to send a trace to OpenObserve."
        );
        return Ok(());
    };

    let exporter = SpanExporter::builder()
        .with_http()
        .with_endpoint(endpoint.to_owned())
        .build()?;
    let provider = SdkTracerProvider::builder()
        .with_resource(
            opentelemetry_sdk::Resource::builder()
                .with_service_name("openobserve-example")
                .build(),
        )
        .with_batch_exporter(exporter)
        .build();
    global::set_tracer_provider(provider.clone());

    let tracer = global::tracer("openobserve-example");
    let mut span = tracer.start("ingest-demo");
    span.set_attribute(KeyValue::new("openobserve.stream", "default"));
    span.add_event("example event payload", vec![KeyValue::new("records", 1)]);

    tracing::info!(%endpoint, stream = "default", "sending trace to OpenObserve");
    tracing::info!(records = 1, "example event payload");

    span.end();
    provider.shutdown()?;
    Ok(())
}

/// Prints a message when the `opentelemetry` feature is not enabled.
#[cfg(not(feature = "opentelemetry"))]
fn main() -> anyhow::Result<()> {
    println!("Enable the `opentelemetry` feature to run this example.");
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example stays offline when no endpoint is configured.
    #[test]
    fn missing_endpoint_does_not_send_a_request() -> anyhow::Result<()> {
        #[cfg(feature = "opentelemetry")]
        run_with_endpoint(None)?;
        #[cfg(not(feature = "opentelemetry"))]
        main()?;
        Ok(())
    }
}
