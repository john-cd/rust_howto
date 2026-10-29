// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Get readable compiler errors for handlers with `#[debug_handler]`.
//!
//! Without the attribute, a handler with an unsupported signature fails with
//! a long error about trait bounds on `Handler`. With it, the error points at
//! the offending argument or return type.

use axum::Router;
use axum::debug_handler;
use axum::extract::State;
use axum::routing::get;

/// Shared state.
#[derive(Clone)]
struct AppState {
    greeting: &'static str,
}

/// A correct handler. Try changing `State<AppState>` to an argument that is
/// not an extractor, or putting `State` *before* a body extractor such as
/// `String`: the error message names the argument.
#[debug_handler]
async fn greet(State(state): State<AppState>) -> String {
    state.greeting.to_owned()
}

/// Builds the router.
pub fn app() -> Router {
    Router::new()
        .route("/", get(greet))
        .with_state(AppState { greeting: "hello" })
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt as _;

    use super::*;

    #[tokio::test]
    async fn greets() -> anyhow::Result<()> {
        let response = app()
            .oneshot(Request::get("/").body(Body::empty())?)
            .await?;
        let bytes = response.into_body().collect().await?.to_bytes();
        assert_eq!(&*bytes, b"hello");
        Ok(())
    }
}
