// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This module demonstrates how to reduce a hierarchical URL to its origin.
//!
//! A URL has parts: scheme (`https`), host (`example.com`), optional port, path, query and fragment. A hierarchical URL, such as `https://...`, can be cut back to its origin or its base to resolve relative links.

use anyhow::Result;
use url::Url;

/// Returns the origin root of a hierarchical URL.
///
/// This function takes a URL and removes the path segments and query
/// parameters, effectively returning the base URL.
///
/// The query and fragment are removed, and the path becomes `/`.
fn base_url(mut url: Url) -> Result<Url> {
    url.set_fragment(None);
    url.set_query(None);
    url.set_path("");
    Ok(url)
}

/// Extracts the base of a URL.
fn main() -> Result<()> {
    let full = "https://github.com/rust-lang/cargo?tab=readme#usage";

    let url = Url::parse(full)?;
    let base = base_url(url)?;

    assert_eq!(base.as_str(), "https://github.com/");
    println!("The base of the URL is: {base}");

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the base example through its entry point.
    #[test]
    fn test_base_example_entry_point() -> Result<()> {
        main()?;
        Ok(())
    }

    /// Verifies origin reduction removes path, query, and fragment.
    #[test]
    fn removes_path_query_and_fragment() -> Result<()> {
        let url = Url::parse("https://example.test:8443/a/b?q=rust#part")?;

        assert_eq!(base_url(url)?.as_str(), "https://example.test:8443/");
        Ok(())
    }
}
