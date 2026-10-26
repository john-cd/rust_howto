// TODO Reviewed ??-??-2026

// ANCHOR: example
//! An asynchronous WebSocket client using async-tungstenite's Tokio adapter.

use async_tungstenite::tokio::accept_async;
use async_tungstenite::tokio::connect_async;
use async_tungstenite::tungstenite::Message;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::net::TcpListener;

/// Echoes WebSocket messages for one local test client.
async fn echo_server(listener: TcpListener) -> anyhow::Result<()> {
    let (stream, _) = listener.accept().await?;
    let mut socket = accept_async(stream).await?;

    while let Some(message) = socket.next().await {
        let message = message?;
        if message.is_close() {
            socket.flush().await?;
            return Ok(());
        }
        socket.send(message).await?;
    }

    Ok(())
}

/// Sends one text message to a loopback echo server and returns its reply.
async fn echo_round_trip() -> anyhow::Result<String> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let server = tokio::spawn(echo_server(listener));

    let (mut socket, response) =
        connect_async(format!("ws://{address}")).await?;
    println!("Handshake status: {}", response.status());

    let sent = "Hello WebSocket";
    socket.send(Message::Text(sent.into())).await?;
    let echoed = socket.next().await.ok_or_else(|| {
        anyhow::anyhow!("echo server closed before replying")
    })??;
    let Message::Text(echoed) = echoed else {
        anyhow::bail!("echo server returned a non-text WebSocket message");
    };
    let echoed = echoed.to_string();
    if echoed != sent {
        anyhow::bail!("echo server returned {echoed:?}, expected {sent:?}");
    }

    socket.close(None).await?;
    server.await??;
    Ok(echoed)
}

/// Runs one local WebSocket echo round trip and prints the reply.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let echoed = echo_round_trip().await?;
    println!("Received: {echoed}");
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the client exchanges text with a local echo server.
    #[tokio::test]
    async fn round_trips_text_with_local_echo_server() -> anyhow::Result<()> {
        let echoed = echo_round_trip().await?;

        assert_eq!(echoed, "Hello WebSocket");
        Ok(())
    }
}
