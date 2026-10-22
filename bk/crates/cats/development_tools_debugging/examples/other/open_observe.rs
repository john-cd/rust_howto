#![allow(dead_code)]
// ANCHOR: example
//! Log telemetry fields for an OpenObserve integration.
//!
//! To transmit spans, first configure an OpenTelemetry SDK provider with an
//! OTLP exporter. The global tracer is a no-op until a provider is installed.
//!
//! TODO Reviewed ??-??-2026

#[cfg(feature = "opentelemetry")]
use opentelemetry::global;
#[cfg(feature = "opentelemetry")]
use opentelemetry::trace::Span;
#[cfg(feature = "opentelemetry")]
use opentelemetry::trace::Tracer;

/// Creates a span with fields and logs events, for an OpenObserve endpoint.
#[cfg(feature = "opentelemetry")]
fn main() {
    let endpoint = std::env::var("OPENOBSERVE_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:5080/api/default".to_string());
    let _ = tracing_subscriber::fmt().with_target(false).try_init();

    let tracer = global::tracer("openobserve-example");
    let mut span = tracer.start("ingest-demo");

    tracing::info!(%endpoint, stream = "default", "prepared OpenObserve telemetry fields");
    tracing::info!(records = 1, "example event payload");

    span.end();
}

/// Prints a message when the `opentelemetry` feature is not enabled.
#[cfg(not(feature = "opentelemetry"))]
fn main() {
    println!("Enable the `opentelemetry` feature to run this example.");
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/733)
}
