// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use tungstenite::Message;
// use tungstenite::connect;
// use url::Url;

// // Lightweight, flexible WebSockets for Rust

// fn main() -> anyhow::Result<()> {
//     // Connect to the WebSocket server
//     let (mut socket, response) =
//         connect(Url::parse("ws://echo.websocket.org")?)?;

//     println!("Connected to the server");
//     println!("Response HTTP code: {}", response.status());
//     println!("Response contains the following headers:");
//     for (ref header, _value) in response.headers() {
//         println!("* {}", header);
//     }

//     // Send a message to the server
//     socket.send(Message::Text("Hello WebSocket".into()))?;
//     println!("Message sent");

//     // Read a message from the server
//     let msg = socket.read()?;
//     println!("Received: {}", msg);

//     Ok(())
// }

// #[test]
// fn require_network()  -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/874)
