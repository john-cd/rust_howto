// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to extract unique links from a Wikipedia page.
//!
//! It uses the `reqwest` crate to fetch the page content, the `regex` crate to
//! find links, and the `lazy_static` crate to compile the regex only once.

use std::borrow::Cow;
use std::collections::HashSet;

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

/// Extracts unique link targets from a limited set of MediaWiki markup forms.
///
/// Internal targets are lowercased into owned strings; external URLs borrow
/// from the source. A hash set removes repeated targets without preserving order.
/// This regex handles the demonstrated forms rather than the full MediaWiki grammar.
///
/// # Panics
///
/// Panics if a future edit makes the fixed MediaWiki link regex invalid.
fn extract_links(content: &str) -> HashSet<Cow<'_, str>> {
    lazy_static! {
        /// Compiled once to capture internal links, bracketed URLs, and URL templates.
        static ref WIKI_REGEX: Regex = Regex::new(
            r"(?x)
                \[\[
                    (?P<internal> [^\[\]|]+ )
                    (?: \| [^\[\]]* )?
                \]\]
                |
                \[
                    (?P<external> https?:// [^\s\]]+ )
                    (?: \s [^\]]* )?
                \]
                |
                (?: url= | URL\| )
                (?P<url_external> https?:// [^\s|}]+ )
            "
        )
        .expect("the fixed MediaWiki link regex is valid");
    }

    let links: HashSet<_> = WIKI_REGEX
        .captures_iter(content)
        .filter_map(|captures| {
            if let Some(internal) = captures.name("internal") {
                Some(Cow::from(internal.as_str().to_lowercase()))
            } else {
                captures
                    .name("external")
                    .or_else(|| captures.name("url_external"))
                    .map(|external| Cow::from(external.as_str()))
            }
        })
        .collect();

    links
}

/// Downloads a Wikipedia article and prints the unique links it contains.
#[tokio::main]
async fn main() -> Result<()> {
    let content = reqwest::get(
        "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
    )
    .await?
    .error_for_status()?
    .text()
    .await?;

    println!("{:#?}", extract_links(content.as_str()));

    Ok(())
}
// ANCHOR_END: example

/// Downloads a Wikipedia article and prints the unique links it contains.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> Result<()> {
    main()
}

/// Verifies MediaWiki links are deduplicated and internal links normalized.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies MediaWiki links are deduplicated and internal links normalized.
    #[test]
    fn extracts_unique_wikitext_links() {
        let links = extract_links(
            "[[Rust (programming language)|Rust]] [[RUST (PROGRAMMING LANGUAGE)]] [https://example.test/docs label]",
        );

        assert!(links.contains("rust (programming language)"));
        assert!(links.contains("https://example.test/docs"));
        assert_eq!(links.len(), 2);
    }

    /// Marks the live Wikipedia source request as an opt-in network test.
    #[test]
    #[ignore = "needs network"]
    fn require_network() -> Result<()> {
        main()?;
        Ok(())
    }
}
