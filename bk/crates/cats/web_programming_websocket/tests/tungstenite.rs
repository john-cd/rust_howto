// ANCHOR: example
use tungstenite::Message;
use tungstenite::connect;

// Lightweight, flexible WebSockets for Rust

fn main() -> anyhow::Result<()> {
    // Connect to a test WebSocket server in blocking mode
    let (mut socket, response) = connect("ws://echo.websocket.in")?;
    // To support secure "wss://"" URLs, feature "native-tls" or "rustls-tls"
    // must be turned on. If you want a non-blocking or other custom stream,
    // call `client` instead.

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, value) in response.headers() {
        println!("* {} {:?}", header, value);
    }

    // Send a message to the server:
    // Writes and immediately flushes a message.
    socket.send(Message::Text("Hello WebSocket".into()))?;
    println!("Message sent");

    // Read a message from the server
    let msg = socket.read()?;
    println!("Received: {}", msg);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
