// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A blocking WebSocket client using tungstenite.

use tungstenite::Message;
use tungstenite::client::IntoClientRequest;
use tungstenite::connect;
use tungstenite::http::Request;

/// WebSocket echo service endpoint used by this example.
const ECHO_ENDPOINT: &str = "wss://ws.postman-echo.com/raw";

/// Creates a request for Postman's public echo service.
fn echo_request() -> anyhow::Result<Request<()>> {
    Ok(ECHO_ENDPOINT.into_client_request()?)
}

/// Connects, sends one message, prints the reply, and closes the connection.
fn main() -> anyhow::Result<()> {
    let (mut socket, response) = connect(echo_request()?)?;
    println!("Handshake status: {}", response.status());

    socket.send(Message::Text("Hello WebSocket".into()))?;
    let reply = socket.read()?;
    println!("Received: {reply}");

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
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}

// [finish; echo.websocket.org has moved permanently](https://github.com/john-cd/rust_howto/issues/1058)
