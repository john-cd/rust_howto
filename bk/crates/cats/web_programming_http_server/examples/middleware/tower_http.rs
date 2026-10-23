#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Serves an Axum route with common `tower-http` middleware.

use std::time::Duration;

use axum::Router;
use axum::routing::get;
use http::StatusCode;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

/// Returns a response body large enough to demonstrate compression.
async fn handler() -> &'static str {
    "tower-http applies tracing, compression, cross-origin headers, and a request timeout to this response."
}

/// Builds the example router with one middleware layer for each concept.
fn app() -> Router {
    Router::new().route("/", get(handler)).layer(
        ServiceBuilder::new()
            // Emit high-level request and response tracing events.
            .layer(TraceLayer::new_for_http())
            // Compress eligible response bodies when the client supports it.
            .layer(CompressionLayer::new())
            // Allow cross-origin requests for this demonstration.
            .layer(CorsLayer::permissive())
            // Return HTTP 408 when a request takes longer than five seconds.
            .layer(TimeoutLayer::with_status_code(
                StatusCode::REQUEST_TIMEOUT,
                Duration::from_secs(5),
            )),
    )
}

/// Binds the example server and serves requests until the process is stopped.
///
/// # Errors
///
/// Returns an error if the listener cannot be bound or the server fails.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("tower-http example listening on http://127.0.0.1:3000");
    axum::serve(listener, app()).await?;
    Ok(())
}

// ANCHOR_END: example

/// Runs the example server.
///
/// # Errors
///
/// Returns an error if the listener cannot be bound or the server fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;
    use tokio::sync::oneshot;

    use super::*;

    /// Verifies a live server responds to a CORS request and exits on shutdown.
    #[tokio::test]
    async fn serves_a_cors_request_and_shuts_down_cleanly() -> anyhow::Result<()>
    {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();
        let server = tokio::spawn(async move {
            axum::serve(listener, app())
                .with_graceful_shutdown(async move {
                    let _ = shutdown_receiver.await;
                })
                .await
        });

        let mut client = tokio::net::TcpStream::connect(address).await?;
        client
            .write_all(
                b"GET / HTTP/1.1\r\nHost: localhost\r\nOrigin: https://client.example\r\nConnection: close\r\n\r\n",
            )
            .await?;
        let mut response = Vec::new();
        client.read_to_end(&mut response).await?;
        let response = String::from_utf8(response)?;
        shutdown_sender.send(()).map_err(|()| {
            anyhow::anyhow!("server shutdown receiver dropped")
        })?;
        tokio::time::timeout(Duration::from_secs(2), server).await???;

        let response_headers = response
            .split_once("\r\n\r\n")
            .map_or(&response, |(headers, _)| headers)
            .to_ascii_lowercase();

        assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
        assert!(
            response_headers.contains("access-control-allow-origin: *"),
            "{response_headers}"
        );
        Ok(())
    }
}
