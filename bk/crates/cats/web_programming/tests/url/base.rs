// ANCHOR: example
//! This module demonstrates how to extract the base URL from a given URL.
//!
//! It uses the `url` crate to parse and manipulate URLs.

use anyhow::Result;
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
    url.set_fragment(None);
    url.set_query(None);
    url.set_path("");

    // You could also use `path_segments_mut` to return an object with methods
    // to manipulate the URL's path segments. match url.path_segments_mut()
    // {     Ok(mut path) => {
    //         path.clear();
    //     }
    //     Err(_) => {
    //         // Some (uncommon) URLs are said to be cannot-be-a-base:
    //         // they don’t have a username, password, host, or port,
    //         // and their "path" is an arbitrary string rather than
    // slash-separated segments.         return Err(anyhow::anyhow!("This
    // URL is cannot-be-a-base."));     }
    // }

    Ok(url)
}

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
