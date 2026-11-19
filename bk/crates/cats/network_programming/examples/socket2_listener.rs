// TODO Reviewed ??-??-2026.

// ANCHOR: example
use std::io::Read;
use std::io::Write;
use std::io::{self};
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

use socket2::Domain;
use socket2::Protocol;
use socket2::Socket;
use socket2::Type;

/// Creates a loopback listener after setting its socket options.
fn make_listener() -> io::Result<TcpListener> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    let address = SocketAddr::from(([127, 0, 0, 1], 0));
    socket.bind(&address.into())?;
    socket.listen(128)?;
    Ok(socket.into())
}

/// Accepts one local connection and returns the listener's response.
fn round_trip() -> io::Result<String> {
    let listener = make_listener()?;
    let address = listener.local_addr()?;
    let mut client = TcpStream::connect(address)?;
    let (mut server, _) = listener.accept()?;
    server.write_all(b"socket2 listener is ready")?;
    drop(server);

    let mut response = String::new();
    client.read_to_string(&mut response)?;
    Ok(response)
}

fn main() -> io::Result<()> {
    println!("{}", round_trip()?);
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn listener_accepts_a_loopback_connection() -> io::Result<()> {
        assert_eq!(round_trip()?, "socket2 listener is ready");
        Ok(())
    }
}
