//! This module demonstrates basic usage of axum.
//!
//! `axum` is a web application framework designed to work with `tokio`
//! and `hyper`. It doesn't have its own middleware system but instead uses
//! `tower::Service`. This means `axum` gets timeouts, tracing, compression,
//! authorization, and more, for free.
//! See the current Axum documentation for its `Router`, extractors, and
//! server APIs.
//!
//! TODO Reviewed ??-??-2026.

#![allow(dead_code)]
// ANCHOR: example
use std::collections::HashMap;
use std::sync::Arc;

use axum::Router;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::routing::get;
use axum::routing::post;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

// Application state.
/// The state shared by all the handlers.
#[derive(Clone)]
struct AppState {
    /// The name displayed by the state example route.
    app_name: &'static str,
}

// An Axum "handler" is an async function that accepts zero or more
// "extractors" as arguments and returns something that can be converted into
// a response.
/// Returns a greeting.
async fn root() -> &'static str {
    "Hello, world!"
    // Anything that implements `IntoResponse` can be returned from handlers.
    // `&'static str` becomes a `200 OK` with `content-type: text/plain;
    // charset=utf-8`
}

// The `Path` extractor gives us the path parameters (here `id` in
// `/users/{id}`) and deserializes them.
/// Returns a message that contains the user identifier from the path.
async fn get_user(Path(id): Path<u64>) -> String {
    format!("User {id}")
}

/// Returns a message that confirms the creation of a user.
async fn post_user(Path(id): Path<u64>) -> (StatusCode, String) {
    (StatusCode::CREATED, format!("User {id} created"))
}

// The wildcard e.g. `/assets/{*path}` is passed as a `String`.
/// Returns the path of the requested asset.
async fn serve_asset(Path(path): Path<String>) -> String {
    path
}

// `Query` gives the query parameters and deserializes them.
/// Returns the query parameters of the request.
async fn query(Query(_params): Query<HashMap<String, String>>) -> String {
    format!("Query parameters: {_params:?}")
}

// Buffer the request body and deserialize it as JSON into a
// `serde_json::Value`. `Json` supports any type that implements
// `serde::Deserialize`.
/// Returns the JSON body of the request.
async fn json(
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(payload)
}

// Access the application state via the `State` extractor.
/// Returns the application name from the shared state.
async fn state(State(state): State<Arc<AppState>>) -> String {
    format!("Application: {}", state.app_name)
}

// Fallbacks apply to routes that aren't matched by anything in the router.
/// Returns a `404 Not Found` response for the unmatched routes.
async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}

/// Builds a router with the handlers and shared application state.
fn app() -> Router {
    let shared_state = Arc::new(AppState {
        app_name: "Axum example",
    });

    // The `path` is a string of segments: static, captured, or wildcard.
    // Examples include `/users/123`, `/users/{id}`, and `/assets/{*path}`.
    Router::new()
        .route("/", get(root))
        .route("/users/{id}", get(get_user).post(post_user))
        .route("/assets/{*path}", get(serve_asset))
        .route("/query", get(query))
        .route("/json", post(json))
        .route("/state", get(state))
        .fallback(fallback)
        .with_state(shared_state)
}

/// Builds the router and serves it on a local TCP listener.
// `tokio` macros and rt-multi-thread features should be enabled.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    format!("{}=debug", env!("CARGO_CRATE_NAME")).into()
                }),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!(address = %listener.local_addr()?, "Listening");
    axum::serve(listener, app()).await?;
    Ok(())
}

// ANCHOR_END: example

#[cfg(test)]
mod tests {
    // use super::*;

    /// Runs the example when its network dependency is available.
    #[test]
    fn require_network() -> anyhow::Result<()> {
        // main()?;
        Ok(())
    }
}
// // review time limit
// // <https://docs.rs/axum/latest/axum/index.html>
// // <https://docs.rs/axum/0.8.1/axum/extract/struct.State.html>
// // <https://docs.rs/axum/0.8.1/axum/extract/index.html>
