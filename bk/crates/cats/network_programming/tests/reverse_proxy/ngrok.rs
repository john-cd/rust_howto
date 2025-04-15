// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates a simple HTTP server that can be exposed to the
// //! internet using `ngrok`.
// //!
// //! "ngrok is your app's front door. ngrok is a globally distributed reverse
// //! proxy that secures, protects and accelerates your applications and
// //! network  services, no matter where you run them. ngrok supports delivering
// //! HTTP, TLS or TCP-based applications".
// //!
// //! Add to your `Cargo.toml`:
// //! ```toml
// //! ngrok = { version = "0.15.0", features = [ "axum" ] } # or latest
// //! axum = { version = "0.8.1", features = ["tokio"] }
// //! anyhow = "1.0.95"
// //! ```
// //!
// //! ## Use Cases
// //!
// //! - Unified ingress-as-a-service.
// //! - Serve HTTP apps and APIs.
// //! - Receive webhooks.
// //! - Test with ephemeral/random domains.
// //! - Share your website / app from your localhost.
// //! - Connect to IoT devices behind NAT/firewalls.
// //! - Debug HTTP requests.
// //!
// //! ## Usage
// //!
// //! 1. Install ngrok e.g. `brew install ngrok`
// //! 2. Run the server: `cargo test --test ngrok`
// //! 3. Start ngrok: `ngrok http 3000`
// //! 4. Access the server from the internet using the ngrok URL.

// use std::net::SocketAddr;

// use axum::Router;
// use axum::routing::get;
// use ngrok::config::ForwarderBuilder;
// use url::Url;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // Create an Axum app.
//     let app = Router::new().route("/", get(|| async { "Hello from Axum!" }));

//     // Spawn Axum server.
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     tokio::spawn(async move {
//         axum::serve(listener, app).await.unwrap();
//     });

//     // Set up a  ngrok tunnel.
//     let sess1 = ngrok::Session::builder()
//         //.authtoken(authtoken)
//         // Call `authtoken` with the value of the NGROK_AUTHTOKEN environment
// variable.         .authtoken_from_env()
//         // Begin a new ngrok Session by connecting to the ngrok service.
//         .connect()
//         .await?;

//     let _listener = sess1
//         // Build a tunnel for an HTTP endpoint.
//         .http_endpoint()
//         .domain("http://rust.ngrok-free.dev")
//         .pooling_enabled(true)
//         .listen_and_forward(Url::parse("http://localhost:3000").unwrap())
//         .await?;

//     // Wait indefinitely.
//     tokio::signal::ctrl_c().await?;
//     Ok(())
// }

// #[test]
// fn test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     main()?;
//     Ok(())
// }
// // [finish NOW](https://github.com/john-cd/rust_howto/issues/811)

// // https://ngrok.com/blog-post/ngrok-rs
// // https://ngrok.com/docs/getting-started/rust/
// // https://github.com/ngrok/ngrok-rust/tree/main/ngrok/examples
// // https://github.com/ngrok/ngrok-rust/blob/main/ngrok/src/online_tests.rs

// // https://pinggy.io/blog/best_ngrok_alternatives/
// // https://dev.to/ghoshbishakh/top-3-ngrok-alternatives-499e
