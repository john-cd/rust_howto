#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to use `async-tungstenite` to connect to a
// //! WebSocket server, send a message, and receive a response.
// //!
// //! `async-tungstenite` provides asynchronous WebSockets for `async-std`,
// //! `tokio`, `gio` and any standard Futures runtime.
// //!
// //! The following uses the `tokio` runtime and `tokio-native-tls` for TLS
// //! support.
// //!
// //! In your `Cargo.toml`, add:
// //! ```toml
// //! async-tungstenite = { version = "0.28.2", features = ["tokio-runtime",
// //! "tokio-native-tls"] }
// //! ```

// use async_tungstenite::tokio::connect_async;
// use futures_util::SinkExt;
// use futures_util::StreamExt;
// use tungstenite::client::IntoClientRequest;

// #[tokio::main]
// async fn main() {
//     // Define the WebSocket server URL.
//     let url = "ws://echo.websocket.org";

//     let mut request = url.into_client_request().unwrap();
//     request
//         .headers_mut()
//         .insert("api-key", "42".parse().unwrap());

//     // Connect to the WebSocket server.
//     let (ws_stream, _) =
//         connect_async(request).await.expect("Failed to connect");

//     println!("Connected to {url}");

//     // Split the WebSocket stream into a sender and receiver.
//     let (mut write, mut read) = ws_stream.split();

//     // Send a message to the WebSocket server.
//     let message = "Hello, WebSocket!";
//     write
//         .send(message.into())
//         .await
//         .expect("Failed to send message");

//     println!("Sent: {message}");

//     // Receive a message from the WebSocket server.
//     if let Some(Ok(msg)) = read.next().await {
//         println!("Received: {msg}");
//     }
// }

// #[test]
// fn require_network() {
//     main();
// }
// // [finish; echo.websocket.org has moved permanently](https://github.com/john-cd/rust_howto/issues/1058)

fn main() {}
