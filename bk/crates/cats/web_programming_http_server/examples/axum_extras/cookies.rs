#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Set and read cookies with `tower-cookies`.

use axum::Router;
use axum::routing::get;
use tower_cookies::Cookie;
use tower_cookies::CookieManagerLayer;
use tower_cookies::Cookies;

const COOKIE_NAME: &str = "lang";

/// Remembers the language choice in a cookie.
async fn set_language(cookies: Cookies) -> &'static str {
    let cookie = Cookie::build((COOKIE_NAME, "fr"))
        .path("/")
        .http_only(true)
        .build();
    cookies.add(cookie);
    "language saved"
}

/// Reads the cookie back, with a default when it is absent.
async fn get_language(cookies: Cookies) -> String {
    cookies
        .get(COOKIE_NAME)
        .map_or_else(|| "en".to_owned(), |c| c.value().to_owned())
}

/// Builds the router.
pub fn app() -> Router {
    Router::new()
        .route("/set", get(set_language))
        .route("/get", get(get_language))
        .layer(CookieManagerLayer::new())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::header;
    use http_body_util::BodyExt;
    use tower::ServiceExt as _;

    use super::*;

    async fn body(
        response: axum::response::Response,
    ) -> anyhow::Result<String> {
        let bytes = response.into_body().collect().await?.to_bytes();
        Ok(String::from_utf8(bytes.to_vec())?)
    }

    #[tokio::test]
    async fn default_without_cookie() -> anyhow::Result<()> {
        let response = app()
            .oneshot(Request::get("/get").body(Body::empty())?)
            .await?;
        assert_eq!(body(response).await?, "en");
        Ok(())
    }

    #[tokio::test]
    async fn cookie_round_trip() -> anyhow::Result<()> {
        let response = app()
            .oneshot(Request::get("/set").body(Body::empty())?)
            .await?;
        let set_cookie = response
            .headers()
            .get(header::SET_COOKIE)
            .ok_or_else(|| anyhow::anyhow!("no Set-Cookie header"))?
            .to_str()?
            .to_owned();
        assert!(set_cookie.starts_with("lang=fr"));
        assert!(set_cookie.contains("HttpOnly"));

        let response = app()
            .oneshot(
                Request::get("/get")
                    .header(header::COOKIE, "lang=fr")
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(body(response).await?, "fr");
        Ok(())
    }
}
