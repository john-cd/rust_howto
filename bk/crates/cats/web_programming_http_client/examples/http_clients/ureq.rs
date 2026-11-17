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
    let response = get_text(url)?;

    println!("Response: {response}");
    Ok(())
}

/// Sends a blocking GET request and returns the response body as text.
fn get_text(url: &str) -> anyhow::Result<String> {
    let mut response = ureq::get(url).call()?;
    Ok(response.body_mut().read_to_string()?)
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
    use super::*;

    /// Requires a live network service to run the example.
    #[test]
    #[ignore = "needs network"]
    fn require_network() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// A GET request returns the body text from the server.
    #[test]
    fn gets_text_from_a_local_server() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/posts/1")
            .with_status(200)
            .with_body("a local post")
            .create();

        let response = get_text(&format!("{}/posts/1", server.url()))?;

        mock.assert();
        assert_eq!(response, "a local post");
        Ok(())
    }
}
// write more?
