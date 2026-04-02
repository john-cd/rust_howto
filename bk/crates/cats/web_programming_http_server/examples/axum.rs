// ANCHOR: example
//! This example demonstrates how to start an `axum` HTTP server.
//!
//! `axum` is a web application framework designed to work with `tokio` and
//! `hyper`. It doesn't have its own middleware system but instead uses
//! `tower::Service`. This means `axum` gets timeouts, tracing, compression,
//! authorization, and more, for free.
//!
//! See also:
//! - <https://docs.rs/axum/latest/axum/index.html>
//! - <https://docs.rs/axum/latest/axum/extract/struct.State.html>
//! - <https://docs.rs/axum/latest/axum/extract/index.html>

use std::sync::Arc;

use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::routing::get;

/// Application state shared across all handlers.
#[derive(Clone)]
struct AppState {
    greeting: String,
}

/// Root handler: returns a greeting from the application state.
///
/// An Axum "handler" is an async function that accepts zero or more
/// "extractors" as arguments and returns something that can be converted into
/// a response. Anything that implements `IntoResponse` can be returned from
/// handlers. `String` becomes a `200 OK` with
/// `content-type: text/plain; charset=utf-8`.
async fn root(State(state): State<Arc<AppState>>) -> String {
    state.greeting.clone()
}

/// Returns a user by ID.
///
/// The [`Path`] extractor gives us the path parameters (here `id` in
/// `/users/{id}`) and deserializes them.
async fn get_user(Path(id): Path<u64>) -> String {
    format!("User {id}")
}

/// Fallback handler for unmatched routes.
///
/// Fallbacks apply to routes that aren't matched by anything in the router.
async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}

// `tokio` macros and rt-multi-thread features must be enabled.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = Arc::new(AppState {
        greeting: "Hello, world!".to_string(),
    });

    // Build the application by routing requests to handlers.
    // The `path` is a string of path segments separated by `/`.
    // Each segment can be either static, a capture, or a wildcard,
    // e.g. `/users/123` or `/users/{id}` or `/assets/{*path}`.
    let router = Router::new()
        .route("/", get(root)) // Could be `get`, `post`, or `delete`...
        .route("/users/{id}", get(get_user))
        .fallback(fallback)
        .with_state(state);

    // Run it; tests for this server are in `tests/axum.rs`.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
// ANCHOR_END: example
