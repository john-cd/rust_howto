// Reviewed 10-28-2026.
// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to parse a URL and extract a portion of it.
//!
//! A URL has parts: scheme, host, port, path, query and fragment. A parser
//! splits them and reports invalid input as an error.

use url::Position;
use url::Url;

/// Parses a URL and prints it up to the end of its path.
///
/// Returns an error if the URL is invalid.
fn example() -> anyhow::Result<String> {
    // Parse a URL. This call can fail if the URL is invalid.
    // It early returns via `?`:
    let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;

    // Extract the portion of the URL, up to the end of the path.
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {cleaned}");
    Ok(cleaned.to_owned())
}
/// This example demonstrates how to parse a URL and extract a portion of it.
fn main() -> anyhow::Result<()> {
    example().map(|_| ())
}
// ANCHOR_END: example

/// Runs the example through its program entry point.
///
/// # Errors
/// Returns an error if the example operation fails.
pub fn run() -> anyhow::Result<()> {
    example().map(|_| ())
}

/// This example demonstrates how to parse a URL and extract a portion of it.
///
/// # Errors
/// Returns an error if an operation in the example fails.
/// Returns an error if a step of the example fails.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main().expect("main failed");
    }
    /// Verifies example strips the query.
    #[test]
    fn test_example_strips_the_query() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(example()?, "https://httpbin.org/cookies/set");
        Ok(())
    }
    /// Verifies invalid URL is an error.
    #[test]
    fn test_invalid_url_is_an_error() {
        assert!(Url::parse("not a url").is_err());
    }
}
