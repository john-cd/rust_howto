// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A Tokio WebSocket client and one-connection echo server.

use anyhow::Context;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::Utf8Bytes;

/// Text sent by the client and returned by the local echo server.
const ECHO_TEXT: &str = "Hello WebSocket";

/// Accepts one client and echoes its first text message.
async fn serve_one_message(listener: TcpListener) -> anyhow::Result<()> {
    let (stream, _) = listener.accept().await?;
    let mut socket = accept_async(stream).await?;
    let message = socket
        .next()
        .await
        .context("client disconnected before sending a message")??;

    let Message::Text(text) = message else {
        anyhow::bail!("expected a text message from the WebSocket client");
    };
    socket.send(Message::Text(text)).await?;
    Ok(())
}

/// Starts a loopback echo server, connects a client, and returns the reply.
async fn echo_round_trip() -> anyhow::Result<String> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let server = tokio::spawn(serve_one_message(listener));

    let (mut client, _) = connect_async(format!("ws://{address}")).await?;
    client
        .send(Message::Text(Utf8Bytes::from_static(ECHO_TEXT)))
        .await?;
    let reply = client
        .next()
        .await
        .context("echo server closed before sending a reply")??;
    let Message::Text(reply) = reply else {
        anyhow::bail!("expected a text reply from the WebSocket server");
    };

    server.await.context("wait for the echo server task")??;
    Ok(reply.to_string())
}

/// Runs the local WebSocket echo client and server and prints the reply.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let reply = echo_round_trip().await?;
    println!("Received: {reply}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the WebSocket echo example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::ECHO_TEXT;
    use super::echo_round_trip;

    /// Verifies that the loopback echo server returns the client's text.
    #[tokio::test]
    async fn loopback_server_echoes_text() -> anyhow::Result<()> {
        let reply = echo_round_trip().await?;

        assert_eq!(reply, ECHO_TEXT);
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
