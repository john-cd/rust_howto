#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates the basic usage of the `http` crate for creating
//! and manipulating HTTP requests and responses.
//!
//! The `http` crate provides fundamental types for working with HTTP, such as
//! `Request`, `Response`, `Uri` for what a `Request` is requesting, `Method`
//! for how it's being requested, `StatusCode` for what sort of response came
//! back, `Version` for how this was communicated, `HeaderName`, and
//! `HeaderValue`.
//!
//! Implementations of the HTTP protocol are elsewhere - see e.g. `hyper` or
//! `reqwest`.

use std::str::FromStr as _;

use http::HeaderMap;
use http::HeaderName;
use http::HeaderValue;
use http::Method;
use http::Request;
use http::Response;
use http::StatusCode;
use http::header::ACCEPT;
use http::header::AUTHORIZATION;
use http::header::CONTENT_LENGTH;
use http::header::CONTENT_TYPE;
use http::header::SET_COOKIE;

/// Builds a header map with standard and application-defined headers.
///
/// # Errors
///
/// Returns an error if any configured header value contains invalid bytes.
fn build_headers() -> anyhow::Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_str("application/json")?);
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str("Bearer example-token")?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("5"));
    headers.insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str("request-42")?,
    );
    headers.append(SET_COOKIE, HeaderValue::from_static("session=abc"));
    headers.append(SET_COOKIE, HeaderValue::from_static("theme=dark"));

    Ok(headers)
}

/// Builds a GET request with typed standard headers and an in-memory body.
///
/// # Errors
///
/// Returns an error if a header is invalid or the request cannot be built.
fn build_request() -> anyhow::Result<Request<&'static str>> {
    let request = Request::builder()
        .method(Method::GET)
        .uri("https://example.com/items")
        .headers(build_headers()?)
        .body("hello")?;

    Ok(request)
}

/// Builds an HTTP request and response, then parses and inspects their headers.
///
/// # Errors
///
/// Returns an error if a header value is invalid or a response header cannot
/// be represented as text.
fn main() -> anyhow::Result<()> {
    let request = build_request()?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_str("text/plain; charset=utf-8")?,
        )
        .header(SET_COOKIE, HeaderValue::from_static("response=ok"))
        .body("Hello, HTTP World!")?;

    println!("Request URI: {}", request.uri());
    println!("Response Status: {}", response.status());
    println!(
        "Request Content-Type: {}",
        request
            .headers()
            .get(CONTENT_TYPE)
            .ok_or_else(|| anyhow::anyhow!(
                "request has no Content-Type header"
            ))?
            .to_str()?
    );
    println!("Request Accept: {:?}", request.headers().get(ACCEPT));
    println!(
        "Request Content-Length: {}",
        request
            .headers()
            .get(CONTENT_LENGTH)
            .ok_or_else(|| anyhow::anyhow!(
                "request has no Content-Length header"
            ))?
            .to_str()?
            .parse::<usize>()?
    );
    println!("Request Authorization: [redacted]");
    println!("Request ID: {:?}", request.headers().get("x-request-id"));
    for cookie in request.headers().get_all(SET_COOKIE) {
        println!("Request Set-Cookie: {}", cookie.to_str()?);
    }
    println!(
        "Response Content-Type: {}",
        response
            .headers()
            .get(CONTENT_TYPE)
            .ok_or_else(|| anyhow::anyhow!(
                "response has no Content-Type header"
            ))?
            .to_str()?
    );

    Ok(())
}

// ANCHOR_END: example

/// Runs the HTTP types and interfaces example.
///
/// # Errors
///
/// Returns an error if header parsing or message construction fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Preserves appended values for a repeated header in insertion order.
    #[test]
    fn appends_and_reads_repeated_header_values_in_order() -> anyhow::Result<()>
    {
        let headers = build_headers()?;
        let cookies = headers
            .get_all(SET_COOKIE)
            .iter()
            .map(HeaderValue::to_str)
            .collect::<Result<Vec<_>, _>>()?;

        assert_eq!(cookies, ["session=abc", "theme=dark"]);

        Ok(())
    }

    /// Rejects header values that contain invalid bytes.
    #[test]
    fn rejects_invalid_header_values() {
        assert!(HeaderValue::from_str("invalid\nvalue").is_err());
    }

    /// Builds a request and response with typed and parsed header values.
    #[test]
    fn builds_request_and_response_messages() -> anyhow::Result<()> {
        let request = build_request()?;
        let headers = request.headers();
        assert_eq!(
            headers.get(ACCEPT).map(HeaderValue::to_str).transpose()?,
            Some("application/json")
        );
        assert_eq!(
            headers
                .get(AUTHORIZATION)
                .map(HeaderValue::to_str)
                .transpose()?,
            Some("Bearer example-token")
        );
        assert_eq!(
            headers
                .get(CONTENT_TYPE)
                .map(HeaderValue::to_str)
                .transpose()?,
            Some("application/json")
        );
        assert_eq!(
            headers
                .get(CONTENT_LENGTH)
                .ok_or_else(|| anyhow::anyhow!(
                    "request has no Content-Length"
                ))?
                .to_str()?
                .parse::<usize>()?,
            5
        );
        assert_eq!(request.method(), Method::GET);
        assert_eq!(request.uri().path(), "/items");
        assert_eq!(
            request
                .headers()
                .get(CONTENT_TYPE)
                .map(HeaderValue::to_str)
                .transpose()?,
            Some("application/json")
        );

        main()
    }
}
