#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to parse a URL and extract a portion of it.
//!
//! The `Url::parse` function is used to parse a URL string into a `Url` object.
//! The `Position` enum is used to specify a position within the URL.
//! In this case, `Position::AfterPath` selects the URL through its path,
//! excluding both its query and fragment.

use url::ParseError;
use url::Position;
use url::Url;

/// Removes the fragment and the query of a URL.
fn main() -> Result<(), ParseError> {
    let parsed = Url::parse(
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open#new-issue",
    )?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    assert_eq!(cleaned, "https://github.com/rust-lang/rust/issues");
    println!("`cleaned`: {cleaned}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the fragment example through its entry point.
    #[test]
    fn test_fragment_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
