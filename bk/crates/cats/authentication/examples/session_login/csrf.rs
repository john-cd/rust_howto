#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Protect a state-changing form with a synchronizer CSRF token.
//!
//! The token is random, stored in the server-side session, embedded in the
//! form as a hidden field, and compared when the form is posted. A forged
//! cross-site request cannot read the token, so it cannot include it.

use std::fmt::Write as _;

use axum::Form;
use axum::Router;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use serde::Deserialize;
use tower_sessions::MemoryStore;
use tower_sessions::Session;
use tower_sessions::SessionManagerLayer;

const TOKEN_KEY: &str = "csrf_token";

/// Returns the session's CSRF token, creating one on first use.
async fn csrf_token(session: &Session) -> Result<String, StatusCode> {
    fn internal<E>(_: E) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    if let Some(token) =
        session.get::<String>(TOKEN_KEY).await.map_err(internal)?
    {
        return Ok(token);
    }
    let mut bytes = [0_u8; 32];
    getrandom::fill(&mut bytes).map_err(internal)?;
    let token = bytes.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    });
    session.insert(TOKEN_KEY, &token).await.map_err(internal)?;
    Ok(token)
}

/// Compares two strings in time that does not depend on where they differ.
fn constant_time_eq(a: &str, b: &str) -> bool {
    a.len() == b.len()
        && a.bytes()
            .zip(b.bytes())
            .fold(0, |acc, (x, y)| acc | (x ^ y))
            == 0
}

/// Renders the form with the hidden token field.
async fn show_form(session: Session) -> Result<Html<String>, StatusCode> {
    let token = csrf_token(&session).await?;
    Ok(Html(format!(
        r#"<form method="post" action="/change-email">
  <input type="hidden" name="csrf_token" value="{token}">
  <input type="email" name="email">
  <button>Change email</button>
</form>"#
    )))
}

#[derive(Deserialize)]
struct ChangeEmail {
    csrf_token: String,
    email: String,
}

/// Rejects the post unless the submitted token matches the session's.
async fn change_email(
    session: Session,
    Form(form): Form<ChangeEmail>,
) -> Result<String, StatusCode> {
    let expected = csrf_token(&session).await?;
    if !constant_time_eq(&expected, &form.csrf_token) {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(format!("Email changed to {}", form.email))
}

/// Builds the application.
pub fn app() -> Router {
    let session_layer =
        SessionManagerLayer::new(MemoryStore::default()).with_secure(false); // `true` when served over HTTPS.
    Router::new()
        .route("/change-email", get(show_form).post(change_email))
        .layer(session_layer)
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::Response;
    use axum::http::header;
    use http_body_util::BodyExt;
    use tower::ServiceExt as _;

    use super::*;

    async fn body_of(response: Response<Body>) -> anyhow::Result<String> {
        let bytes = response.into_body().collect().await?.to_bytes();
        Ok(String::from_utf8(bytes.to_vec())?)
    }

    /// Fetches the form; returns the session cookie and the embedded token.
    async fn fetch_form(app: &Router) -> anyhow::Result<(String, String)> {
        let response = app
            .clone()
            .oneshot(Request::get("/change-email").body(Body::empty())?)
            .await?;
        let cookie = response
            .headers()
            .get(header::SET_COOKIE)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split(';').next())
            .ok_or_else(|| anyhow::anyhow!("no cookie"))?
            .to_owned();
        let html = body_of(response).await?;
        let token = html
            .split("value=\"")
            .nth(1)
            .and_then(|s| s.split('"').next())
            .ok_or_else(|| anyhow::anyhow!("no token"))?
            .to_owned();
        Ok((cookie, token))
    }

    async fn post(
        app: &Router,
        cookie: &str,
        token: &str,
    ) -> anyhow::Result<Response<Body>> {
        let request = Request::post("/change-email")
            .header(header::COOKIE, cookie)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(format!(
                "csrf_token={token}&email=a%40example.com"
            )))?;
        Ok(app.clone().oneshot(request).await?)
    }

    #[tokio::test]
    async fn valid_token_is_accepted() -> anyhow::Result<()> {
        let app = app();
        let (cookie, token) = fetch_form(&app).await?;
        let response = post(&app, &cookie, &token).await?;
        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn wrong_token_is_rejected() -> anyhow::Result<()> {
        let app = app();
        let (cookie, _) = fetch_form(&app).await?;
        let response = post(&app, &cookie, "forged").await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }

    #[tokio::test]
    async fn token_from_another_session_is_rejected() -> anyhow::Result<()> {
        let app = app();
        let (_, other_token) = fetch_form(&app).await?;
        let (cookie, _) = fetch_form(&app).await?;
        let response = post(&app, &cookie, &other_token).await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }

    #[test]
    fn comparison_checks_length_and_content() {
        assert!(constant_time_eq("abc", "abc"));
        assert!(!constant_time_eq("abc", "abd"));
        assert!(!constant_time_eq("abc", "ab"));
    }
}
