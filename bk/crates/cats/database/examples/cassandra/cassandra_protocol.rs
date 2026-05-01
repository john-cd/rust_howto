#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use cassandra_protocol::consistency::Consistency;
// use cassandra_protocol::error::Error;
// use cassandra_protocol::frame::Opcode;
// use cassandra_protocol::frame::StreamId;
// use cassandra_protocol::protocol::ProtocolVersion;
// use cassandra_protocol::protocol::Request;
// use cassandra_protocol::types::Value;
// use tokio::net::TcpStream;
// use tokio::prelude::*;

// // Cassandra low-level protocol implementation, written in Rust.
// // If you wish to use Cassandra without dealing with protocol-level details,
// // consider a high-level crate like `cdrs-tokio`.

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // Create a TCP stream to the Cassandra node:
//     let stream = tokio::net::TcpStream::connect("127.0.0.1:9042").await?;

//     let protocol_version = ProtocolVersion::V3; // Or your desired version.
//     // 1. STARTUP.
//     let startup_options =
//         vec![("CQL_VERSION".to_string(), "3.0.0".to_string())];
//     let startup_request = Request::Startup {
//         options: startup_options,
//     };
//     let startup_bytes = startup_request.encode(protocol_version)?;
//     stream.write_all(&startup_bytes).await?;
//     // Use helper function:
//     let _startup_response = read_frame(&mut stream).await?;

//     // 2. OPTIONS (Optional, but good practice):
//     let options_request = Request::Options;
//     let options_bytes = options_request.encode(protocol_version)?;
//     stream.write_all(&options_bytes).await?;
//     let _options_response = read_frame(&mut stream).await?;

//     // 3. QUERY:
//     let query = "SELECT * FROM system.local".to_string();
//     let query_request = Request::Query {
//         query,
//         consistency: 0, // Replace with your desired consistency level
//         values: vec![], /* For bound parameters, use Vec<Value>
//                          * ... other query options */
//     };
//     let query_bytes = query_request.encode(protocol_version)?;
//     stream.write_all(&query_bytes).await?;

//     let query_response = read_frame(&mut stream).await?;

//     match query_response.opcode() {
//         Opcode::Result => {
//             println!("Received query result");
//             // Now parse the rows from the `Result` message.
//             let result = query_response.result().expect("Expected Result");
//             match result {
//                 cassandra_protocol::frame::Result::Rows(rows) => {
//                     for row in rows.rows_content {
//                         println!("Row: {row:?}");
//                         // Process each row (Vec<Value>).
//                         for val in row {
//                             match val {
//                                 Value::Text(t) => println!("Text Value: {}",
// t),                                 Value::Int(i) => println!(
//                                     "Int Value: {i}"
//                                 ), // Handle other Value types as needed.
//                                 _ => println!("Other Value: {:?}", val),
//                             }
//                         }
//                     }
//                 }
//                 _ => println!("Other Result type: {result:?}"),
//             }
//         }
//         Opcode::Error => {
//             println!("Received query error: {:?}", query_response.error());
//         }
//         _ => println!("Received other response: {:?}",
// query_response.opcode()),     }

//     Ok(())
// }

// // Helper function to read and decode a frame.
// async fn read_frame(
//     stream: &mut TcpStream,
// ) -> Result<cassandra_protocol::frame::Frame, Error> {
//     let mut buffer = vec![0; 4096]; // Adjust buffer size as needed.
//     let n = stream.read(&mut buffer).await?;
//     cassandra_protocol::frame::Frame::decode(&buffer[..n])
// }

// #[test]
// fn require_external_svc() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [write; see also https://docs.rs/cassandra-protocol/latest/cassandra_protocol/index.html](https://github.com/john-cd/rust_howto/issues/1016)
