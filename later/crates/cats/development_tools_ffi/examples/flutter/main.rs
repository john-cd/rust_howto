//! Runs the Flutter interoperability example.
//!
//! TODO Reviewed ??-??-2026.

/// Provides the generated stream sink type used by the bridge example.
#[cfg(feature = "flutter")]
pub mod frb_generated {
    flutter_rust_bridge::frb_generated_sse_codec!();
    flutter_rust_bridge::frb_generated_stream_sink!(
        default_stream_sink_codec = SseCodec
    );

    impl SseEncode for String {
        fn sse_encode(
            self,
            serializer: &mut flutter_rust_bridge::for_generated::SseSerializer,
        ) {
            use std::io::Write as _;

            use byteorder::WriteBytesExt as _;

            let bytes = self.into_bytes();
            let length = i32::try_from(bytes.len()).expect(
                "SSE strings must fit in the generated i32 length field",
            );
            serializer
                .cursor
                .write_i32::<byteorder::NativeEndian>(length)
                .expect("writing to the in-memory SSE buffer should succeed");
            serializer
                .cursor
                .write_all(&bytes)
                .expect("writing to the in-memory SSE buffer should succeed");
        }
    }
}

/// Exposes the Flutter Rust Bridge example API and runner.
pub mod flutter_rust_bridge;

/// Runs the flutter example.
fn main() {
    flutter_rust_bridge::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
