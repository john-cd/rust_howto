#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example builds a Basic-authenticated request without sending it.

use reqwest::blocking::Client;
use reqwest::blocking::RequestBuilder;

/// Creates a GET request that uses HTTP Basic authentication.
fn basic_auth_request(
    client: &Client,
    endpoint: &str,
    user_name: &str,
    password: &str,
) -> RequestBuilder {
    client.get(endpoint).basic_auth(user_name, Some(password))
}

/// Builds a request locally so the example can run without a network service.
///
/// The credentials are placeholders. Real credentials must be kept secret and
/// sent only to an HTTPS endpoint.
///
/// # Errors
/// Returns an error if the request cannot be constructed.
fn main() -> anyhow::Result<()> {
    let client = Client::new();
    let request = basic_auth_request(
        &client,
        "https://example.test/protected",
        "demo-user",
        "demo-password",
    )
    .build()?;
    println!(
        "Built a Basic-authenticated request for {} (not sent).",
        request.url()
    );
    Ok(())
}

// ANCHOR_END: example

/// # Errors
/// Returns an error if the request cannot be constructed.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use anyhow::Context as _;
    use axum::Router;
    use axum::body::Body;
    use axum::http::HeaderMap;
    use axum::http::HeaderValue;
    use axum::http::Request;
    use axum::http::StatusCode;
    use axum::routing::get;
    use reqwest::header::AUTHORIZATION;
    use tower::ServiceExt;

    use super::*;

    /// Returns unauthorized unless the request has the example credentials.
    async fn protected(headers: HeaderMap) -> StatusCode {
        let expected = HeaderValue::from_static("Basic YWxpY2U6c2VjcmV0");
        if headers.get(AUTHORIZATION) == Some(&expected) {
            StatusCode::OK
        } else {
            StatusCode::UNAUTHORIZED
        }
    }

    /// Verifies valid credentials receive 200 and invalid ones receive 401
    /// without a socket.
    #[test]
    fn basic_auth_route_accepts_valid_and_rejects_invalid_credentials()
    -> anyhow::Result<()> {
        let request = basic_auth_request(
            &Client::new(),
            "https://example.test/protected",
            "alice",
            "secret",
        )
        .build()?;
        let authorization = request
            .headers()
            .get(AUTHORIZATION)
            .context("request should contain an Authorization header")?;
        let valid_request = Request::builder()
            .uri("/protected")
            .header(AUTHORIZATION, authorization.clone())
            .body(Body::empty())?;
        let invalid_request = Request::builder()
            .uri("/protected")
            .header(AUTHORIZATION, "Basic Ym9iOndyb25n")
            .body(Body::empty())?;
        let app = Router::new().route("/protected", get(protected));
        let runtime = tokio::runtime::Builder::new_current_thread().build()?;
        let valid_response = runtime
            .block_on(app.clone().oneshot(valid_request))
            .expect("Axum's in-memory router service is infallible");

        assert_eq!(valid_response.status(), StatusCode::OK);

        let invalid_response = runtime
            .block_on(app.oneshot(invalid_request))
            .expect("Axum's in-memory router service is infallible");

        assert_eq!(invalid_response.status(), StatusCode::UNAUTHORIZED);
        Ok(())
    }
}
