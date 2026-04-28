#![allow(dead_code)]
// ANCHOR: example
// This example demonstrates how to build and serialize Cassandra protocol
// frames using the `cassandra_protocol` crate.
//
// This crate provides a Cassandra low-level protocol implementation, written in
// Rust. If you wish to use Cassandra without dealing with protocol-level
// details, consider a higher-level crate such as `cdrs_tokio`.

use cassandra_protocol::consistency::Consistency;
use cassandra_protocol::error::Error;
use cassandra_protocol::frame::Frame;
use cassandra_protocol::frame::Opcode;
use cassandra_protocol::protocol::ProtocolVersion;
use cassandra_protocol::protocol::Request;
use cassandra_protocol::types::Value;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

async fn read_frame(stream: &mut TcpStream) -> Result<Frame, Error> {
    let mut buffer = vec![0; 4096];
    let n = stream.read(&mut buffer).await?;
    Frame::decode(&buffer[..n])
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut stream = TcpStream::connect("127.0.0.1:9042").await?;
    let protocol_version = ProtocolVersion::V3;

    // 1. STARTUP
    let startup_options =
        vec![("CQL_VERSION".to_string(), "3.0.0".to_string())];
    let startup_request = Request::Startup {
        options: startup_options,
    };
    stream
        .write_all(&startup_request.encode(protocol_version)?)
        .await?;
    let startup_response = read_frame(&mut stream).await?;
    println!("Startup response opcode: {:?}", startup_response.opcode());

    // 2. OPTIONS
    let options_request = Request::Options;
    stream
        .write_all(&options_request.encode(protocol_version)?)
        .await?;
    let _options_response = read_frame(&mut stream).await?;
    println!("Received OPTIONS response");

    // 3. QUERY
    let query = "SELECT key, bootstrapped FROM system.local".to_string();
    let query_request = Request::Query {
        query,
        consistency: Consistency::One as u16,
        values: vec![],
    };
    stream
        .write_all(&query_request.encode(protocol_version)?)
        .await?;
    let query_response = read_frame(&mut stream).await?;

    match query_response.opcode() {
        Opcode::Result => {
            println!("Received query result");
            let result = query_response.result().expect("Expected Result");
            match result {
                cassandra_protocol::frame::Result::Rows(rows) => {
                    for row in rows.rows_content {
                        println!("Row: {row:?}");
                        for value in row {
                            match value {
                                Value::Text(t) => println!("Text value: {t}"),
                                Value::Int(i) => println!("Int value: {i}"),
                                _ => println!("Other value: {value:?}"),
                            }
                        }
                    }
                }
                other => println!("Other result type: {other:?}"),
            }
        }
        Opcode::Error => {
            println!("Received query error: {:?}", query_response.error());
        }
        other => {
            println!("Received other response: {:?}", other);
        }
    }

    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn require_external_svc() -> Result<(), Box<dyn std::error::Error>> {
        main()?;
        Ok(())
    }
}
// [write; see also https://docs.rs/cassandra-protocol/latest/cassandra_protocol/index.html](https://github.com/john-cd/rust_howto/issues/1016)
