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

use bytes::Bytes;
use http::Method;
use http::Request;
use http::Response;
use http::StatusCode;
use http::header::CONTENT_TYPE;
use http_body_util::Full;

/// Builds an HTTP request and response, then inspects them.
fn main() -> http::Result<()> {
    // The body type is supplied by the caller.
    let request = Request::builder()
        .method(Method::GET)
        .uri("https://example.com/items")
        .header("User-Agent", "Rust http crate")
        .body(Full::new(Bytes::new()))?;

    // Full is a simple http_body::Body implementation for one payload.
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Full::new(Bytes::from_static(b"Hello, HTTP World!")))?;

    println!("Request URI: {}", request.uri());
    println!("Response Status: {}", response.status());
    println!(
        "Response Content-Type: {:?}",
        response.headers().get(CONTENT_TYPE)
    );

    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main().expect("the static HTTP request and response are valid");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_request_and_response_messages() {
        main().expect("the static HTTP request and response are valid");
    }
}
// [expand; HTTP header interpretation and generation.  LATER](https://github.com/john-cd/rust_howto/issues/1355)
