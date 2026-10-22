#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Talking to Cassandra over its native protocol with `cassandra-protocol`.

// This example demonstrates how to build and serialize Cassandra protocol
// frames using the `cassandra_protocol` crate.
//
// This crate provides a Cassandra low-level protocol implementation, written in
// Rust. If you wish to use Cassandra without dealing with protocol-level
// details, consider a higher-level crate such as `cdrs_tokio`.

use std::convert::TryInto as _;

use cassandra_protocol::compression::Compression;
use cassandra_protocol::consistency::Consistency;
use cassandra_protocol::error::Error;
use cassandra_protocol::error::Result;
use cassandra_protocol::frame::Envelope;
use cassandra_protocol::frame::Flags;
use cassandra_protocol::frame::Version;
use cassandra_protocol::frame::message_response::ResponseBody;
use cassandra_protocol::types::ByName as _;
use tokio::io::AsyncReadExt as _;
use tokio::io::AsyncWriteExt as _;
use tokio::net::TcpStream;

/// Reads one protocol envelope from `stream`.
#[allow(
    clippy::result_large_err,
    reason = "Keep the protocol crate's Result type, as in the calling example."
)]
async fn read_envelope(stream: &mut TcpStream) -> Result<Envelope> {
    let mut header = [0u8; 9];
    stream.read_exact(&mut header).await?;

    let body_len =
        i32::from_be_bytes(header[5..9].try_into().unwrap()) as usize;
    let mut buffer = Vec::with_capacity(9 + body_len);
    buffer.extend_from_slice(&header);
    buffer.resize(9 + body_len, 0);
    stream.read_exact(&mut buffer[9..]).await?;

    Ok(Envelope::from_buffer(&buffer, Compression::None)
        .map_err(|err| Error::General(err.to_string()))?
        .envelope)
}

/// Sends startup, options and query requests to Cassandra and prints the
/// responses.
#[allow(clippy::result_large_err)]
#[tokio::main]
async fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9042").await?;
    let protocol_version = Version::V4;

    // 1. STARTUP
    let startup_request = Envelope::new_req_startup(None, protocol_version);
    stream
        .write_all(&startup_request.encode_with(Compression::None)?)
        .await?;
    let startup_response = read_envelope(&mut stream).await?;
    println!("Startup response opcode: {:?}", startup_response.opcode);

    // 2. OPTIONS
    let options_request = Envelope::new_req_options(protocol_version);
    stream
        .write_all(&options_request.encode_with(Compression::None)?)
        .await?;
    let _options_response = read_envelope(&mut stream).await?;
    println!("Received OPTIONS response");

    // 3. QUERY
    let query = "SELECT key, bootstrapped FROM system.local".to_owned();
    let query_request = Envelope::new_req_query(
        query,
        Consistency::One,
        None,
        true,
        None,
        None,
        None,
        None,
        None,
        None,
        Flags::empty(),
        protocol_version,
    );
    stream
        .write_all(&query_request.encode_with(Compression::None)?)
        .await?;
    let query_response = read_envelope(&mut stream).await?;

    match query_response.response_body()? {
        ResponseBody::Result(result) => {
            println!("Received query result");
            let rows = result.into_rows().unwrap_or_default();
            for row in rows {
                let key: Option<String> = row.by_name("key")?;
                let bootstrapped: Option<bool> = row.by_name("bootstrapped")?;
                println!("Row: key={key:?}, bootstrapped={bootstrapped:?}");
            }
        }
        ResponseBody::Error(error_body) => {
            println!("Received query error: {error_body:?}");
        }
        other => {
            println!("Unexpected response body: {other:?}");
        }
    }

    Ok(())
}
// ANCHOR_END: example

/// Runs the example; needs a running Cassandra server.
pub fn run() -> anyhow::Result<()> {
    main()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn require_external_svc() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
// [write; see also https://docs.rs/cassandra-protocol/latest/cassandra_protocol/index.html](https://github.com/john-cd/rust_howto/issues/1016)
