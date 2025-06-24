#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates how to build a URL by joining a base URL with a path.

use url::ParseError;
use url::Url;

/// Builds a GitHub URL by joining a base URL with a given path.
///
/// # Arguments
///
/// * `path` - The path to append to the base GitHub URL.
///
/// Returns a `Result` containing the joined `Url` or a `ParseError`.
fn build_github_url(path: &str) -> Result<Url, ParseError> {
    const GITHUB: &str = "https://github.com";

    let base =
        Url::parse(GITHUB).expect("This hardcoded URL is known to be valid");
    let joined = base.join(path)?;

    Ok(joined)
}

fn main() -> Result<(), ParseError> {
    let path = "/rust-lang/cargo";

    let gh = build_github_url(path)?;
    println!("The joined URL is: {}", gh);
    assert_eq!(gh.as_str(), "https://github.com/rust-lang/cargo");

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
