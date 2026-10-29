// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates URL parsing.

use url::ParseError;
use url::Url;

/// Demonstrates URL parsing.
///
/// This function parses a URL string and prints the path part of the URL.
fn main() -> Result<(), ParseError> {
    // Define a URL string.
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), ParseError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the parse example through its entry point.
    #[test]
    fn test_parse_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// The parts of the URL are available after parsing.
    #[test]
    fn exposes_url_parts() {
        let url = Url::parse(
            "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open",
        )
        .unwrap();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.host_str(), Some("github.com"));
        assert_eq!(url.path(), "/rust-lang/rust/issues");
        assert_eq!(url.query(), Some("labels=E-easy&state=open"));
    }

    /// Malformed URLs are rejected.
    #[test]
    fn rejects_malformed_urls() {
        assert_eq!(
            Url::parse("no scheme"),
            Err(ParseError::RelativeUrlWithoutBase)
        );
        assert_eq!(Url::parse("http://"), Err(ParseError::EmptyHost));
    }
}
