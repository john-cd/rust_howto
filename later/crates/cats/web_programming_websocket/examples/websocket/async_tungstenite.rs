// TODO Reviewed ??-??-2026

// ANCHOR: example
//! An asynchronous WebSocket client using async-tungstenite's Tokio adapter.

use async_tungstenite::tokio::connect_async;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::tungstenite::client::IntoClientRequest;
use async_tungstenite::tungstenite::http::Request;
use futures_util::StreamExt;

/// WebSocket echo service endpoint used by this example.
const ECHO_ENDPOINT: &str = "wss://ws.postman-echo.com/raw";

/// Creates a request for Postman's public echo service.
fn echo_request() -> anyhow::Result<Request<()>> {
    Ok(ECHO_ENDPOINT.into_client_request()?)
}

/// Connects, sends one message, prints the reply, and closes the connection.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (mut socket, response) = connect_async(echo_request()?).await?;
    println!("Handshake status: {}", response.status());

    socket.send(Message::Text("Hello WebSocket".into())).await?;
    while let Some(message) = socket.next().await {
        let message = message?;
        match message {
            Message::Text(text) => {
                println!("Received: {text}");
                break;
            }
            Message::Close(frame) => {
                println!("Server closed the connection: {frame:?}");
                break;
            }
            message => println!("Received another message: {message}"),
        }
    }

    socket.close(None).await?;
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a secure WebSocket request without network access.
    #[test]
    fn builds_secure_echo_request() -> anyhow::Result<()> {
        let request = echo_request()?;
        assert_eq!(request.uri().scheme_str(), Some("wss"));
        Ok(())
    }
}
