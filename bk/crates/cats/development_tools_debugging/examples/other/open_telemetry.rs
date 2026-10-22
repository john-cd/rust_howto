#![allow(dead_code)]
// ANCHOR: example
//! Create a manual OpenTelemetry span and pair it with regular tracing output.
//!
//! TODO Reviewed ??-??-2026

#[cfg(feature = "opentelemetry")]
use opentelemetry::global;
#[cfg(feature = "opentelemetry")]
use opentelemetry::trace::Span as _;
#[cfg(feature = "opentelemetry")]
use opentelemetry::trace::Tracer as _;

/// Creates a span and logs an event.
#[cfg(feature = "opentelemetry")]
fn main() {
    let _ = tracing_subscriber::fmt().with_target(false).try_init();

    let tracer = global::tracer("rust_howto");
    let mut span = tracer.start("example-request");
    tracing::info!(path = "/health", "handling request");
    span.end();

    println!("Created an OpenTelemetry span named `example-request`.");
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
}
// [finish](https://github.com/john-cd/rust_howto/issues/734)
