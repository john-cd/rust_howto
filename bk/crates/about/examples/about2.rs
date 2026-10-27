// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to parse a URL and extract a portion of it.

use url::Position;
use url::Url;

// If URL parsing fails, `example` returns the error and `main` propagates it
// to the runtime, which reports it on the console.
/// Parses a URL and prints it up to the end of its path.
///
/// Returns an error if the URL is invalid.
fn example() -> anyhow::Result<()> {
    // Parse a URL. This call can fail if the URL is invalid. It early returns
    // via `?`:
    let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;

    // Extract the portion of the URL, up to the end of the path.
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {cleaned}");
    Ok(())
}

/// Runs the about2 example.
fn main() -> anyhow::Result<()> {
    example()
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the second about example entry point.
    #[test]
    fn test_about_example_two() -> anyhow::Result<()> {
        example()?;
        Ok(())
    }
}
