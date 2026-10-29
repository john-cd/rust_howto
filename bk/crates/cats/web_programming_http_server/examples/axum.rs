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
    use axum::body::Body;
    use axum::http::Method;
    use axum::http::Request;
    use http_body_util::BodyExt as _;
    use tower::ServiceExt as _;

    use super::*;

    /// Sends a request to the router and returns the status and body.
    async fn send(request: Request<Body>) -> (StatusCode, String) {
        let response = app().oneshot(request).await.expect("infallible");
        let status = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }

    /// Sends a request with no body.
    async fn get_uri(method: Method, uri: &str) -> (StatusCode, String) {
        send(
            Request::builder()
                .method(method)
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
    }

    /// Serves the example until it is stopped.
    #[test]
    #[ignore = "binds 127.0.0.1:3000 and serves until stopped"]
    fn main_serves() -> anyhow::Result<()> {
        main()
    }

    #[tokio::test]
    async fn root_greets() {
        assert_eq!(
            get_uri(Method::GET, "/").await,
            (StatusCode::OK, "Hello, world!".to_owned())
        );
    }

    #[tokio::test]
    async fn path_parameters_are_extracted() {
        assert_eq!(get_uri(Method::GET, "/users/7").await.1, "User 7");
        assert_eq!(
            get_uri(Method::POST, "/users/7").await,
            (StatusCode::CREATED, "User 7 created".to_owned())
        );
        assert_eq!(get_uri(Method::GET, "/assets/a/b.css").await.1, "a/b.css");
    }

    /// A non-numeric id is rejected by the extractor.
    #[tokio::test]
    async fn invalid_user_id_is_rejected() {
        let (status, _) = get_uri(Method::GET, "/users/abc").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn query_and_state_are_read() {
        assert!(
            get_uri(Method::GET, "/query?a=1")
                .await
                .1
                .contains("\"a\": \"1\"")
        );
        assert_eq!(
            get_uri(Method::GET, "/state").await.1,
            "Application: Axum example"
        );
    }

    #[tokio::test]
    async fn json_is_echoed() {
        let request = Request::builder()
            .method(Method::POST)
            .uri("/json")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"k":1}"#))
            .unwrap();
        let (status, body) = send(request).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, r#"{"k":1}"#);
    }

    #[tokio::test]
    async fn unknown_route_uses_the_fallback() {
        assert_eq!(
            get_uri(Method::GET, "/nope").await,
            (StatusCode::NOT_FOUND, "No route for /nope".to_owned())
        );
    }
}
// // review time limit
// // <https://docs.rs/axum/latest/axum/index.html>
// // <https://docs.rs/axum/0.8.1/axum/extract/struct.State.html>
// // <https://docs.rs/axum/0.8.1/axum/extract/index.html>
