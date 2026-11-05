// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A blocking WebSocket client using tungstenite.
//!
//! A WebSocket is a long-lived, two-way connection between a client and a
//! server. It starts as an ordinary HTTP request that is "upgraded" (the
//! handshake), after which either side can send messages at any time. That
//! suits chat, live dashboards, games and notifications, where polling with
//! repeated HTTP requests would be slow and wasteful. A blocking client waits
//! for each operation to finish, which keeps the code simple.

use tungstenite::Message;
use tungstenite::client::IntoClientRequest;
use tungstenite::connect;
use tungstenite::http::Request;

/// WebSocket echo service endpoint used by this example.
// `wss://` is WebSocket over TLS (encrypted), like `https://`; `ws://` is
// unencrypted. An echo service sends back whatever it receives.
const ECHO_ENDPOINT: &str = "wss://ws.postman-echo.com/raw";

/// Creates a request for Postman's public echo service.
fn echo_request() -> anyhow::Result<Request<()>> {
    Ok(ECHO_ENDPOINT.into_client_request()?)
}

/// Connects, sends one message, prints the reply, and closes the connection.
fn main() -> anyhow::Result<()> {
    let (mut socket, response) = connect(echo_request()?)?;
    // Status 101 (Switching Protocols) means the server accepted the upgrade.
    println!("Handshake status: {}", response.status());

    socket.send(Message::Text("Hello WebSocket".into()))?;
    let reply = socket.read()?;
    println!("Received: {reply}");

    // Close politely: this sends a close frame so the server can clean up.
    socket.close(None)?;
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

    #[test]
    #[ignore = "needs network access to the public echo service"]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}

// [finish; echo.websocket.org has moved permanently](https://github.com/john-cd/rust_howto/issues/1058)
