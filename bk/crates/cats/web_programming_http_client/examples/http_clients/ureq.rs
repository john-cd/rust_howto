#![allow(dead_code)]
// ANCHOR: example
/// This example demonstrates a simple HTTP GET request using the `ureq` crate.
/// It fetches data from a public API endpoint and prints the response.
fn main() -> anyhow::Result<()> {
    // Define the URL to fetch data from.
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    // Send a GET request to the URL and read the response body as a string.
    let response: String =
        // Make a GET request.
        ureq::get(url)
        // Send the request and blocks the caller until we receive a response.
        .call()?
        // Return a mutable reference to the associated HTTP body.
        .body_mut()
        .read_to_string()?;

    println!("Response: {response}");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [write more? NOW](https://github.com/john-cd/rust_howto/issues/862)
