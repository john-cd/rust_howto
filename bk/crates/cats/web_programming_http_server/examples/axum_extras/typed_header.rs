// TODO Reviewed ??-??-2026.

#![allow(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Extract strongly typed headers with [`axum_extra::TypedHeader`].

use axum::Router;
use axum::routing::get;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::UserAgent;
use axum_extra::headers::authorization::Bearer;

/// Rejects the request with `400 Bad Request` if either header is missing
/// or malformed.
async fn whoami(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    TypedHeader(agent): TypedHeader<UserAgent>,
) -> String {
    format!("token={} agent={}", auth.token(), agent.as_str())
}

/// Builds the router.
pub fn app() -> Router {
    Router::new().route("/whoami", get(whoami))
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use tower::ServiceExt as _;

    use super::*;

    async fn call(
        headers: &[(&str, &str)],
    ) -> anyhow::Result<(StatusCode, String)> {
        let mut builder = Request::get("/whoami");
        for (k, v) in headers {
            builder = builder.header(*k, *v);
        }
        let response = app().oneshot(builder.body(Body::empty())?).await?;
        let status = response.status();
        let body = response.into_body().collect().await?.to_bytes();
        Ok((status, String::from_utf8_lossy(&body).into_owned()))
    }

    #[tokio::test]
    async fn valid_headers() -> anyhow::Result<()> {
        let (status, body) = call(&[
            ("authorization", "Bearer abc123"),
            ("user-agent", "tests/1.0"),
        ])
        .await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "token=abc123 agent=tests/1.0");
        Ok(())
    }

    #[tokio::test]
    async fn missing_header() -> anyhow::Result<()> {
        let (status, _) = call(&[("user-agent", "tests/1.0")]).await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[tokio::test]
    async fn malformed_authorization() -> anyhow::Result<()> {
        let (status, _) = call(&[
            ("authorization", "Basic abc"),
            ("user-agent", "tests/1.0"),
        ])
        .await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        Ok(())
    }
}
