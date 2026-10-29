// TODO Reviewed ??-??-2026.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the snippet declares items to illustrate them, not to call them"
    )
)]
// ANCHOR: example
//! One-time "flash" messages with `axum-messages` and `tower-sessions`.

use axum::Router;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::get;
use axum_messages::Messages;
use axum_messages::MessagesManagerLayer;
use tower_sessions::MemoryStore;
use tower_sessions::SessionManagerLayer;

/// Stores a message, then redirects to the page that displays it.
async fn save(messages: Messages) -> impl IntoResponse {
    messages.success("Settings saved");
    Redirect::to("/show")
}

/// Renders - and consumes - the pending messages.
async fn show(messages: Messages) -> String {
    messages
        .into_iter()
        .map(|message| message.message)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Builds the router. Messages are kept in the session, so the session layer
/// must wrap the messages layer.
pub fn app() -> Router {
    let session_layer =
        SessionManagerLayer::new(MemoryStore::default()).with_secure(false);
    Router::new()
        .route("/save", get(save))
        .route("/show", get(show))
        .layer(MessagesManagerLayer)
        .layer(session_layer)
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use axum::http::header;
    use http_body_util::BodyExt;
    use tower::ServiceExt as _;

    use super::*;

    async fn get_with(
        app: &Router,
        uri: &str,
        cookie: Option<&str>,
    ) -> anyhow::Result<axum::response::Response> {
        let mut request = Request::get(uri);
        if let Some(cookie) = cookie {
            request = request.header(header::COOKIE, cookie);
        }
        Ok(app.clone().oneshot(request.body(Body::empty())?).await?)
    }

    async fn text(
        response: axum::response::Response,
    ) -> anyhow::Result<String> {
        let bytes = response.into_body().collect().await?.to_bytes();
        Ok(String::from_utf8(bytes.to_vec())?)
    }

    #[tokio::test]
    async fn message_is_shown_once() -> anyhow::Result<()> {
        let app = app();

        let response = get_with(&app, "/save", None).await?;
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        // Keep only `name=value` of the session cookie.
        let cookie = response
            .headers()
            .get(header::SET_COOKIE)
            .ok_or_else(|| anyhow::anyhow!("no session cookie"))?
            .to_str()?
            .split(';')
            .next()
            .unwrap_or_default()
            .to_owned();

        let first = text(get_with(&app, "/show", Some(&cookie)).await?).await?;
        assert_eq!(first, "Settings saved");

        let second =
            text(get_with(&app, "/show", Some(&cookie)).await?).await?;
        assert!(second.is_empty());
        Ok(())
    }
}
