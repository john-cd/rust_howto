// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Create a manual OpenTelemetry span and pair it with regular tracing output.

#[cfg(feature = "opentelemetry")]
use opentelemetry::KeyValue;
#[cfg(feature = "opentelemetry")]
use opentelemetry::global;
#[cfg(feature = "opentelemetry")]
use opentelemetry::trace::Span as _;
#[cfg(feature = "opentelemetry")]
use opentelemetry::trace::Tracer as _;
#[cfg(feature = "opentelemetry")]
use opentelemetry_sdk::trace::SdkTracerProvider;

/// Creates a span and logs an event.
#[cfg(feature = "opentelemetry")]
fn main() -> anyhow::Result<()> {
    let _ = tracing_subscriber::fmt().with_target(false).try_init();

    let provider = SdkTracerProvider::builder()
        .with_resource(
            opentelemetry_sdk::Resource::builder()
                .with_service_name("rust_howto")
                .build(),
        )
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();
    global::set_tracer_provider(provider.clone());

    let tracer = global::tracer("rust_howto");
    let mut span = tracer.start("example-request");
    span.set_attribute(KeyValue::new("http.route", "/health"));
    span.add_event("request handled", Vec::new());
    tracing::info!(path = "/health", "handling request");
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

    /// Verifies the example installs an exporter and flushes its span.
    #[test]
    fn installs_exporter_and_flushes_span() -> anyhow::Result<()> {
        main()
    }
}
