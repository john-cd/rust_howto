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

use http::Request;
use http::Response;
use http::StatusCode;

fn main() {
    // Create a request.
    let request = Request::builder()
        .method("GET") // POST, DELETE...
        .uri("https://example.com")
        .header("User-Agent", "Rust http crate")
        .body(())
        .unwrap();

    // Create a response.
    let response = Response::builder()
        .status(StatusCode::OK) // NOT_FOUND...
        .header("Content-Type", "text/plain")
        .body("Hello, HTTP World!".to_string())
        .unwrap();

    println!("Request URI: {}", request.uri());
    println!("Response Status: {}", response.status());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [expand; HTTP header interpretation and generation.  LATER](https://github.com/john-cd/rust_howto/issues/1355)
