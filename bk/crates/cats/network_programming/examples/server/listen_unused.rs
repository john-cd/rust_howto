#![allow(dead_code)]
// TODO Reviewed ??-??-2026

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
use std::io::Read as _;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::TcpListener;

/// Accepts one connection on `listener` and returns the text it sends.
fn read_one_connection(listener: TcpListener) -> Result<String, Error> {
    let port = listener.local_addr()?;
    println!(
        "Listening on {port}; send one line and close the connection to finish"
    );
    let (mut tcp_stream, addr) = listener.accept()?;
    println!("Connection received! {addr:?} is sending data.");
    let mut input = String::new();
    tcp_stream.read_to_string(&mut input)?;
    Ok(input)
}

/// Listens on an unused port of the loopback interface and prints what one
/// client sends.
fn main() -> Result<(), Error> {
    let loopback = Ipv4Addr::LOCALHOST;
    let socket = SocketAddrV4::new(loopback, 0); // 0 means any available port.
    let listener = TcpListener::bind(socket)?; // Bind to the socket.
    let input = read_one_connection(listener)?;
    println!("Received: {input}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that reads one local connection and returns.
    #[test]
    fn reads_one_local_connection_and_returns() -> Result<(), Error> {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
        let address = listener.local_addr()?;
        let client = std::thread::spawn(move || -> Result<(), Error> {
            use std::io::Write as _;

            let mut stream = std::net::TcpStream::connect(address)?;
            stream.write_all(b"hello")?;
            stream.shutdown(std::net::Shutdown::Write)?;
            Ok(())
        });

        assert_eq!(read_one_connection(listener)?, "hello");
        client.join().expect("client thread should complete")?;
        Ok(())
    }
}
// finish; listens to a connection forever
