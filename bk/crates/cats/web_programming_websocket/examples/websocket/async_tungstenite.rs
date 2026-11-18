// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! An asynchronous WebSocket client using async-tungstenite's Tokio adapter.
//!
//! A WebSocket is a long-lived, two-way connection that starts as an HTTP
//! request and is then upgraded, so either side can send messages at any time.
//! The async version waits for messages without blocking a thread, so one
//! thread can serve many connections.

use std::time::Duration;

use anyhow::Context;
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

    let message = socket
        .next()
        .await
        .context("client disconnected before sending text")??;
    anyhow::ensure!(message.is_text(), "expected a text message");
    socket.send(message).await?;

    let close = socket
        .next()
        .await
        .context("client disconnected without closing")??;
    anyhow::ensure!(close.is_close(), "expected a close frame");
    socket.flush().await?;
    Ok(())
}

/// Sends one text message to a loopback echo server and returns its reply.
async fn echo_round_trip() -> anyhow::Result<String> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let ((), echoed) = tokio::time::timeout(Duration::from_secs(5), async {
        tokio::try_join!(echo_server(listener), echo_client(address))
    })
    .await
    .context("WebSocket round trip timed out")??;
    Ok(echoed)
}

/// Sends one text frame, checks the reply, and initiates the closing handshake.
async fn echo_client(address: std::net::SocketAddr) -> anyhow::Result<String> {
    let (mut socket, response) = connect_async(format!("ws://{address}")).await?;
    println!("Handshake status: {}", response.status());

    let sent = "Hello WebSocket";
    socket.send(Message::Text(sent.into())).await?;
    let echoed = socket
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("echo server closed before replying"))??;
    let Message::Text(echoed) = echoed else {
        anyhow::bail!("echo server returned a non-text WebSocket message");
    };
    let echoed = echoed.to_string();
    if echoed != sent {
        anyhow::bail!("echo server returned {echoed:?}, expected {sent:?}");
    }

    socket.close(None).await?;
    let close = socket
        .next()
        .await
        .context("server disconnected without replying to close")??;
    anyhow::ensure!(close.is_close(), "expected a close reply");
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

/// Verifies the client exchanges text with a local echo server.
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

    /// Runs the example through its entry point without error.
    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
