#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates a simple HTTP server that can be exposed to the
//! internet using `ngrok`.
//!
//! Add these dependencies to your `Cargo.toml`:
//! ```toml
//! ngrok = { version = "0.19.0", default-features = false, features = ["axum", "ring"] }
//! axum = { version = "0.8.9", features = ["tokio"] }
//! anyhow = "1.0.104"
//! tokio = { version = "1.54.1", features = ["macros", "rt-multi-thread", "signal"] }
//! url = "2.5.8"
//! ```
//!
//! ## Usage
//!
//! 1. Set `NGROK_AUTHTOKEN` in the environment.
//! 2. Run `cargo run -p network_programming --example reverse_proxy -- ngrok`.
//! 3. Open the public URL printed by the example and stop it with Ctrl-C.

use anyhow::Context as _;
use axum::Router;
use axum::routing::get;
use ngrok::config::ForwarderBuilder as _;
use ngrok::prelude::EndpointInfo as _;
use url::Url;

/// Builds the local Axum app that ngrok forwards requests to.
fn app() -> Router {
    Router::new().route("/", get(|| async { "Hello from Axum!" }))
}

/// Serves a web app and exposes it with an `ngrok` tunnel.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Bind an ephemeral loopback port so the example can run alongside other
    // local services without a fixed-port conflict.
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .context("could not bind the local Axum server")?;
    let local_addr = listener
        .local_addr()
        .context("could not determine the local Axum address")?;
    let upstream = Url::parse(&format!("http://{local_addr}"))
        .context("could not build the local Axum URL")?;
    let app = app();
    let mut server =
        tokio::spawn(async move { axum::serve(listener, app).await });

    // Set up a tunnel using the token from NGROK_AUTHTOKEN.
    let sess = ngrok::Session::builder()
        .authtoken_from_env()
        .connect()
        .await
        .context("could not connect to ngrok; check NGROK_AUTHTOKEN")?;
    let tunnel = sess
        .http_endpoint()
        .listen_and_forward(upstream)
        .await
        .context("could not forward the ngrok endpoint to Axum")?;
    println!("Serving the Axum app at {}", tunnel.url());

    tokio::select! {
        result = &mut server => {
            let result = result.context("Axum server task failed")?;
            result.context("Axum server stopped unexpectedly")?;
            anyhow::bail!("Axum server stopped before shutdown was requested");
        }
        result = tokio::signal::ctrl_c() => {
            result.context("could not listen for Ctrl-C")?;
            server.abort();
            match server.await {
                Err(error) if error.is_cancelled() => {}
                Err(error) => return Err(error.into()),
                Ok(Err(error)) => return Err(error.into()),
                Ok(Ok(())) => {}
            }
        }
    }
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

/// Tests the local route without requiring ngrok or an authentication token.
#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use tower::ServiceExt as _;

    use super::*;

    /// Returns the greeting and an OK status from the root route.
    #[tokio::test]
    async fn root_route_returns_greeting() -> anyhow::Result<()> {
        let response = app()
            .oneshot(Request::builder().uri("/").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await?
                .as_ref(),
            b"Hello from Axum!"
        );
        Ok(())
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/811)
// https://ngrok.com/download/rust
