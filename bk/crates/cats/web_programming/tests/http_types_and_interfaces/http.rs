// ANCHOR: example
use http::Request;
use http::Response;
use http::StatusCode;

// The `http` crate contains common types for the HTTP protocol.
// You’ll find:
// - `Request`
// - `Response`
// - `Uri` for what a `Request` is requesting,
// - `Method` for how it’s being requested,
// - `StatusCode` for what sort of response came back,
// - `Version` for how this was communicated,
// - `HeaderName` / `HeaderValue` definitions to get grouped in a `HeaderMap`.
// Implementations of the HTTP protocol are elsewhere - see e.g. `hyper` or `rewest`.

fn main() {
    // Create a request
    let request = Request::builder()
        .method("GET") // POST, DELETE...
        .uri("https://example.com")
        .header("User-Agent", "Rust http crate")
        .body(())
        .unwrap();

    // Create a response
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
// TODO expand; HTTP header interpretation and generation.  LATER
