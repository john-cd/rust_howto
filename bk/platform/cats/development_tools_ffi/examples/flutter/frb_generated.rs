//! Generated Flutter Rust Bridge support used by the example.

flutter_rust_bridge::frb_generated_sse_codec!();
flutter_rust_bridge::frb_generated_stream_sink!(default_stream_sink_codec = SseCodec);

impl SseEncode for String {
    /// Encodes a string into the bridge's server-sent event buffer.
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        use std::io::Write as _;

        use byteorder::WriteBytesExt as _;

        let bytes = self.into_bytes();
        let length = i32::try_from(bytes.len())
            .expect("SSE strings must fit in the generated i32 length field");
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
