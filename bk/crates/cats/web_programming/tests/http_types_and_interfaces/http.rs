// ANCHOR: example
use http::Request;
use http::Response;
use http::StatusCode;

fn main() {
    // Create a request
    let request = Request::builder()
        .method("GET")
        .uri("https://example.com")
        .header("User-Agent", "Rust http crate")
        .body(())
        .unwrap();

    // Create a response
    let response = Response::builder()
        .status(StatusCode::OK)
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
// [write; rename folder NOW](https://github.com/john-cd/rust_howto/issues/1129)
