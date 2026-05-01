#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to parse a URL and extract a portion of it.
//!
//! The `Url::parse` function is used to parse a URL string into a `Url` object.
//! The `Position` enum is used to specify a position within the URL.
//! In this case, `Position::AfterPath` is used to specify the position after
//! the path. The `cleaned` variable is then assigned a slice of the URL string
//! from the beginning to the specified position. Finally, the `cleaned` string
//! is printed to the console.

use url::ParseError;
use url::Position;
use url::Url;

fn main() -> Result<(), ParseError> {
    let parsed = Url::parse(
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open",
    )?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("`cleaned`: {cleaned}");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
