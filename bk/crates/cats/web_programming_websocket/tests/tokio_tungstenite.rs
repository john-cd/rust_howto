// ANCHOR: example
// Extension trait for Sinks; defines `send()`
use futures_util::SinkExt;
// Extension trait for Streams; defines `next()`
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
// An enum representing the various forms of a WebSocket message: text,
// binary...
use tokio_tungstenite::tungstenite::protocol::Message;

// Create an asynchronous WebSocket client.
// The WebSocket protocol provides a simultaneous two-way communication channel
// over a single TCP connection, typically between a web browser and a web
// server.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to a test WebSocket server, which will echo the message you sent.
    let request = "ws://echo.websocket.in".into_client_request()?;
    // Add headers if needed:
    // request.headers_mut().insert("api-key", "42".parse()?);
    let (ws_stream, response) = connect_async(request).await?;
    println!("Connected to the server");

    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    let (mut write, mut read) = ws_stream.split();

    // Send a message to the server
    write.send(Message::Text("Hello WebSocket".into())).await?;
    println!("Message sent!");

    // Read a message from the server
    if let Some(msg) = read.next().await {
        let msg = msg?;
        println!("Received: {}", msg);
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// TODO WIP NOW review https://github.com/snapview/tokio-tungstenite/tree/master/examples
