// ANCHOR: example
//! This example demonstrates a simple TCP server that listens for a single
//! connection, reads data from the connection, and then prints the received
//! data.
//!
//! The server binds to a dynamically assigned port on the loopback interface
//! (127.0.0.1). It then waits for an incoming connection. Once a connection is
//! established, it reads data from the client and prints the received message.
//!
//! The server will continue to listen until a client connects and sends data.
//!
//! To test this, run the server and then use a tool like `telnet` or `netcat`
//! to connect to the printed port. For example: `telnet 127.0.0.1 <port>` or
//! `nc 127.0.0.1 <port>`.
use std::io::Error;
use std::io::Read;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::TcpListener;

fn main() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0); // 0 means any available port
    let listener = TcpListener::bind(socket)?; // Bind to the socket
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);
    let (mut tcp_stream, addr) = listener.accept()?; // Block until requested
    println!("Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;
    println!("{:?} says {}", addr, input);
    Ok(())
}
// ANCHOR_END: example

#[ignore = "Needs review"]
#[test]
fn test() {
    println!("{:?}", main());
}
// [finish; listens to a connection forever](https://github.com/john-cd/rust_howto/issues/166)
