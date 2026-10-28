// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Talking to Cassandra over its native protocol with `cassandra-protocol`.

// This example demonstrates how to build and serialize Cassandra protocol
// frames using the `cassandra_protocol` crate.
//
// This crate provides a Cassandra low-level protocol implementation, written in
// Rust. If you wish to use Cassandra without dealing with protocol-level
// details, consider a higher-level crate such as `cdrs_tokio`.

use cassandra_protocol::compression::Compression;
use cassandra_protocol::frame::Envelope;
use cassandra_protocol::frame::Version;

/// Encodes and decodes a Cassandra startup request without a server.
///
/// # Errors
///
/// Returns an error if serialization or frame decoding fails.
fn round_trip_startup_request() -> anyhow::Result<Envelope> {
    let request = Envelope::new_req_startup(None, Version::V4);
    let bytes = request.encode_with(Compression::None)?;
    let parsed = Envelope::from_buffer(&bytes, Compression::None)?;
    anyhow::ensure!(
        parsed.envelope_len == bytes.len(),
        "decoder did not consume the complete frame"
    );
    anyhow::ensure!(
        parsed.envelope == request,
        "decoded frame differs from the original request"
    );

    Ok(parsed.envelope)
}

/// Runs the frame encoding and decoding example without a Cassandra server.
///
/// # Errors
///
/// Returns an error if frame serialization or decoding fails.
fn main() -> anyhow::Result<()> {
    let decoded = round_trip_startup_request()?;
    println!(
        "Round-tripped {:?} request using {:?} (stream {})",
        decoded.opcode, decoded.version, decoded.stream_id
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the offline frame encoding and decoding example.
///
/// # Errors
///
/// Returns an error if frame serialization or decoding fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that a STARTUP frame encodes and decodes without a server.
    #[test]
    fn startup_request_round_trips_through_a_protocol_frame()
    -> anyhow::Result<()> {
        let decoded = round_trip_startup_request()?;

        assert_eq!(decoded.opcode, cassandra_protocol::frame::Opcode::Startup);
        assert_eq!(decoded.version, Version::V4);

        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
