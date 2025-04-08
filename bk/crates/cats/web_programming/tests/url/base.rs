// ANCHOR: example
//! This module demonstrates how to extract the base URL from a given URL.
//!
//! It uses the `url` crate to parse and manipulate URLs.
//! The `base_url` function removes the path segments and query parameters,
//! effectively returning the base URL.
use anyhow::Result;
use anyhow::anyhow;
use url::Url;

/// Extracts the base URL from a given URL.
///
/// This function takes a URL and removes the path segments and query
/// parameters, effectively returning the base URL.
///
/// # Arguments
///
/// * `url` - The URL to extract the base from.
///
/// # Returns
///
/// Returns a `Result` containing the base URL or an error if the URL cannot be
/// processed.
fn base_url(mut url: Url) -> Result<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(anyhow!("This URL is cannot-be-a-base."));
        }
    }

    url.set_query(None);

    Ok(url)
}

/// The main function demonstrates the usage of the `base_url` function.
///
/// It parses a sample URL, extracts its base, and asserts that the result is as
/// expected. It also prints the base URL to the console.
fn main() -> Result<()> {
    let full = "https://github.com/rust-lang/cargo?asdf";

    let url = Url::parse(full)?;
    let base = base_url(url)?;

    assert_eq!(base.as_str(), "https://github.com/");
    println!("The base of the URL is: {}", base);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
// [review NOW](https://github.com/john-cd/rust_howto/issues/174)
