#![allow(dead_code)]
// TODO Reviewed ??-??-2026

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

/// Returns the unique internal and external links found in the wiki markup
/// `content`.
fn extract_links(content: &str) -> HashSet<Cow<'_, str>> {
    lazy_static! {
    /// Matches Wikipedia article URLs in scraped text.
            static ref WIKI_REGEX: Regex = Regex::new(
                r"(?x)
                \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
                |
                (url=|URL\||\[)(?P<external>http.*?)[\x20|}] # external links
            "
            )
            .expect("the fixed MediaWiki link regex is valid");
        }

    let links: HashSet<_> = WIKI_REGEX
        .captures_iter(content)
        .map(|c| match (c.name("internal"), c.name("external")) {
            (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
            (None, Some(val)) => Cow::from(val.as_str()),
            _ => unreachable!(),
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

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

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
