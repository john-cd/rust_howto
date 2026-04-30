#![allow(dead_code)]
//! This example demonstrates a simple HTTP server that can be exposed to the
//! internet using `ngrok`.
//!
//! "`ngrok` is your app's front door. `ngrok` is a globally distributed
//! reverse proxy that secures, protects and accelerates your applications
//! and network services, no matter where we run them. `ngrok` supports
//! delivering HTTP, TLS or TCP-based applications".
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! ngrok = { version = "0.15.0", features = [ "axum" ] } # or latest
//! axum = { version = "0.8.4", features = ["tokio"] }
//! anyhow = "1.0.100"
//! ```
//!
//! ## Use Cases
//!
//! - Unified ingress-as-a-service.
//! - Serve HTTP apps and APIs.
//! - Receive webhooks.
//! - Test with ephemeral/random domains.
//! - Share your website / app from your localhost.
//! - Connect to IoT devices behind NAT/firewalls.
//! - Debug HTTP requests.
//!
//! ## Usage
//!
//! 1. Install ngrok e.g. `brew install ngrok`
//! 2. Run the server: `cargo test --package network_programming --example reverse_proxy`
//! 3. Start ngrok: `ngrok http 3000`
//! 4. Access the server from the internet using the ngrok URL.

// ANCHOR: example
use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use ngrok::config::ForwarderBuilder;
use url::Url;

async fn run() -> anyhow::Result<()> {
    // Create an Axum app.
    let app = Router::new().route("/", get(|| async { "Hello from Axum!" }));

    // Spawn Axum server.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Set up a `ngrok` tunnel.
    let sess = ngrok::Session::builder()
        //.authtoken(authtoken)
        // Call `authtoken` with the value of the NGROK_AUTHTOKEN environment variable.
        .authtoken_from_env()
        // Begin a new ngrok Session by connecting to the ngrok service.
        .connect()
        .await?;

    let _tunnel = sess
        // Build a tunnel for an HTTP endpoint.
        .http_endpoint()
        .domain("rust.ngrok-free.dev")
        .listen_and_forward(Url::parse("http://localhost:3000").unwrap())
        .await?;

    // Wait indefinitely.
    tokio::signal::ctrl_c().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}

#[ignore = "Requires network access and NGROK_AUTHTOKEN"]
#[test]
fn require_network() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(run())
        .unwrap();
}

// [finish](https://github.com/john-cd/rust_howto/issues/811)

// https://ngrok.com/blog-post/ngrok-rs
// https://ngrok.com/docs/getting-started/rust/
// https://github.com/ngrok/ngrok-rust/tree/main/ngrok/examples
// https://github.com/ngrok/ngrok-rust/blob/main/ngrok/src/online_tests.rs

// https://pinggy.io/blog/best_ngrok_alternatives/
// https://dev.to/ghoshbishakh/top-3-ngrok-alternatives-499e
// ANCHOR_END: example
