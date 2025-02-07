// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use cassandra_protocol::consistency::Consistency;
// use cassandra_protocol::frame::Opcode;
// use tokio::net::TcpStream;

// // Cassandra low-level protocol implementation, written in Rust.
// // If you wish to use Cassandra without dealing with protocol-level details,
// // consider a high-level crate like `cdrs-tokio`.

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // Create a TCP stream to the Cassandra node
//     let stream = TcpStream::connect("127.0.0.1:9042").await?;

//     // Configure the transport
//     let config = TransportConfig::default();
//     let mut transport = Transport::new(stream, config)?;

//     // Send the startup message
//     let startup = Startup::new(vec![("CQL_VERSION", "3.0.0")]);
//     transport.send(Frame::new(Opcode::Startup, startup)).await?;

//     // Receive the options message
//     let frame = transport.receive().await?;
//     let options = frame.get_body::<Options>()?;

//     // Send the options message
//     transport.send(Frame::new(Opcode::Options, options)).await?;

//     // Send a query message
//     let query =
//         Query::new("SELECT * FROM system.local", Consistency::One, None,
// None);     transport.send(Frame::new(Opcode::Query, query)).await?;

//     // Receive the result message
//     let frame = transport.receive().await?;
//     let rows = frame.get_body::<Rows>()?;

//     // Print the results
//     println!("{:?}", rows);

//     Ok(())
// }

// #[test]
// fn require_external_svc() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// [ P2 write; see also https://docs.rs/cassandra-protocol/latest/cassandra_protocol/index.html](https://github.com/john-cd/rust_howto/issues/1016)
