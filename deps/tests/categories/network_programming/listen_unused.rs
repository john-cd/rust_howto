// ANCHOR: example
use std::io::Error;
use std::io::Read;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::TcpListener;

fn main() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);
    let (mut tcp_stream, addr) = listener.accept()?; // block  until requested
    println!("Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;
    println!("{:?} says {}", addr, input);
    Ok(())
}
// ANCHOR_END: example

#[ignore]
#[test]
fn test() {
    println!("{:?}", main());
}
// [listen_unused: listens to a connection forever (P1)](https://github.com/john-cd/rust_howto/issues/166)
