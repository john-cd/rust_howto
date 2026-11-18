// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Echoes text over a blocking WebSocket on loopback.

use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use anyhow::bail;
use anyhow::ensure;
use tungstenite::Error;
use tungstenite::Message;
use tungstenite::accept;
use tungstenite::client;

/// Bounds blocking I/O for the local example.
const TIMEOUT: Duration = Duration::from_secs(5);

/// Completes a single text echo and responds to the client's close frame.
fn serve_one_message(stream: TcpStream) -> anyhow::Result<()> {
    let mut socket = accept(stream)?;
    let Message::Text(text) = socket.read()? else {
        bail!("expected a text message from the client");
    };
    socket.send(Message::Text(text))?;

    ensure!(socket.read()?.is_close(), "expected a close frame");
    // Reading Close queues the reply. The server flushes it before dropping TCP.
    match socket.flush() {
        Ok(()) | Err(Error::ConnectionClosed) => Ok(()),
        Err(error) => Err(error.into()),
    }
}

/// Sends one text message and waits for the server's close response.
fn exchange(url: &str, stream: TcpStream) -> anyhow::Result<String> {
    let (mut socket, _) = client(url, stream)?;
    let sent = "Hello WebSocket";
    socket.send(Message::text(sent))?;
    let Message::Text(reply) = socket.read()? else {
        bail!("expected a text reply from the server");
    };
    ensure!(reply.as_str() == sent, "unexpected echo: {reply}");

    socket.close(None)?;
    ensure!(socket.read()?.is_close(), "expected a close reply");
    Ok(reply.to_string())
}

/// Connects the loopback TCP pair and joins the blocking server after the exchange.
fn echo_round_trip() -> anyhow::Result<String> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    let client = TcpStream::connect_timeout(&address, TIMEOUT)?;
    let (server, _) = listener.accept()?;
    for stream in [&client, &server] {
        stream.set_read_timeout(Some(TIMEOUT))?;
        stream.set_write_timeout(Some(TIMEOUT))?;
    }

    let worker = thread::spawn(move || serve_one_message(server));
    let reply = exchange(&format!("ws://{address}"), client);
    let server = worker
        .join()
        .map_err(|_| anyhow::anyhow!("echo server panicked"))?;
    let reply = reply?;
    server?;
    Ok(reply)
}

/// Prints the reply after the client and server complete the echo.
fn main() -> anyhow::Result<()> {
    println!("Received: {}", echo_round_trip()?);
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::echo_round_trip;

    /// A blocking client receives its text back and completes the close handshake.
    #[test]
    fn blocking_echo_finishes() -> anyhow::Result<()> {
        assert_eq!(echo_round_trip()?, "Hello WebSocket");
        Ok(())
    }
}
