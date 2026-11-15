// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! This example demonstrates a simple HTTP GET request using the `ureq` crate.
//! It fetches data from a public API endpoint and prints the response.
//!
//! `ureq` is a small blocking HTTP client: each call waits for the response, so
//! no async runtime is needed. It suits scripts and command-line tools; use an
//! async client such as `reqwest` when you make many concurrent requests.

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

/// This example demonstrates a simple HTTP GET request using the `ureq` crate. It fetches data from
/// a public API endpoint and prints the response.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// `ureq` crate.
#[cfg(test)]
mod tests {
    // output-only: the example fetches a live URL (ignored offline); see the
    // other tests for its logic.
    use super::*;

    /// Requires a live network service to run the example.
    #[test]
    fn require_network() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
// write more?
