// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use futures_util::StreamExt;
// use futures_util::SinkExt;
// //use futures_util::future;
// //use futures_util::pin_mut;
// //use tokio::io::AsyncReadExt;
// //use tokio::io::AsyncWriteExt;
// use tokio_tungstenite::connect_async;
// use tokio_tungstenite::tungstenite::protocol::Message;
// use tokio_tungstenite::tungstenite::client::IntoClientRequest;

// // Create an asynchronous WebSocket client.
// //
// // Connect to the WebSocket server, send the message "Hello WebSocket," and
// // print the response from the server, which will be an echo of the message
// // you sent.

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // Connect to the WebSocket server
//     let mut request = "ws://echo.websocket.org/".into_client_request()?;
//     request.headers_mut().insert("api-key", "42".parse()?);
//     let (ws_stream, response) = connect_async(request).await?;
//     println!("Connected to the server");

//     println!("Response HTTP code: {}", response.status());
//     println!("Response contains the following headers:");
//     for (ref header, _value) in response.headers() {
//         println!("* {}", header);
//     }

//     let (mut write, mut read) = ws_stream.split();

//     // Send a message to the server
//     write.send(Message::Text("Hello WebSocket".into())).await?;
//     println!("Message sent");

//     // Read a message from the server
//     if let Some(msg) = read.next().await {
//         let msg = msg?;
//         println!("Received: {}", msg);
//     }

//     Ok(())
// }

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/873)
// // review https://github.com/snapview/tokio-tungstenite/tree/master/examples
