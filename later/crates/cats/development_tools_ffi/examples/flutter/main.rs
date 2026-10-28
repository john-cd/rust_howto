//! Runs the Flutter interoperability example.
//!
//! TODO Reviewed ??-??-2026.

/// Provides the generated stream sink type used by the bridge example.
#[cfg(feature = "flutter")]
pub mod frb_generated {
    flutter_rust_bridge::frb_generated_stream_sink!(
        default_stream_sink_codec = SseCodec
    );
}

/// Exposes the Flutter Rust Bridge example API and runner.
pub mod flutter_rust_bridge;

/// Runs the flutter example.
fn main() {
    flutter_rust_bridge::run();
}
