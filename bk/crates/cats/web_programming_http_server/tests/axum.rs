// ANCHOR: example
//! This example demonstrates basic usage of the `axum` web framework.
//!
//! `axum` is a web application framework designed to work with `tokio` and
//! `hyper`. It doesn't have its own middleware system but instead uses
//! `tower::Service`. This means `axum` gets timeouts, tracing, compression,
//! authorization, and more, for free.
//!
//! This test file tests `axum` handlers and routing using
//! [`tower::ServiceExt::oneshot`], which calls the [`axum::Router`] directly
//! as a [`tower::Service`], without binding a TCP listener. This avoids
//! the need for time limits that would otherwise be required when testing
//! against a long-running server.
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
/// "extractors" as arguments and returns something that can be converted
/// into a response. Anything that implements `IntoResponse` can be returned
/// from handlers. Here `String` becomes a `200 OK` with
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

/// Creates the application router with all routes configured.
///
/// The `path` is a string of path segments separated by `/`.
/// Each segment can be either static, a capture, or a wildcard,
/// e.g., `/users/123` or `/users/{id}`.
fn create_app() -> Router {
    let state = Arc::new(AppState {
        greeting: "Hello, world!".to_string(),
    });
    Router::new()
        .route("/", get(root))
        .route("/users/{id}", get(get_user))
        .fallback(fallback)
        .with_state(state)
}
// ANCHOR_END: example

#[tokio::test]
async fn test_root() -> anyhow::Result<()> {
    use axum::body::Body;
    use http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt; // for `oneshot`

    let app = create_app();
    // `ServiceExt::oneshot` calls the service with a single request,
    // without starting a TCP listener. No time limit needed.
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await?.to_bytes();
    assert_eq!(&body[..], b"Hello, world!");
    Ok(())
}

#[tokio::test]
async fn test_get_user() -> anyhow::Result<()> {
    use axum::body::Body;
    use http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    let app = create_app();
    let response = app
        .oneshot(Request::builder().uri("/users/42").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await?.to_bytes();
    assert_eq!(&body[..], b"User 42");
    Ok(())
}

#[tokio::test]
async fn test_fallback() -> anyhow::Result<()> {
    use axum::body::Body;
    use http::Request;
    use tower::ServiceExt;

    let app = create_app();
    let response = app
        .oneshot(Request::builder().uri("/nonexistent").body(Body::empty())?)
        .await?;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
