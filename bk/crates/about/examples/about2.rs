// Reviewed 10-28-2026.

// ANCHOR: example
//! This example demonstrates how to parse a URL and extract a portion of it.

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

fn main() -> anyhow::Result<()> {
    example().map(|_| ())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main().expect("main failed");
    }

    #[test]
    fn test_example_strips_the_query() -> Result<(), Box<dyn std::error::Error>>
    {
        assert_eq!(example()?, "https://httpbin.org/cookies/set");
        Ok(())
    }

    #[test]
    fn test_invalid_url_is_an_error() {
        assert!(Url::parse("not a url").is_err());
    }
}
