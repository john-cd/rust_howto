#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Extracts anchor links from an HTML document with CSS selectors.

use anyhow::Result;
use scraper::Html;
use scraper::Selector;

/// Extracts `href` values from anchor elements in an HTML document.
///
/// # Errors
///
/// Returns an error if the built-in CSS selector cannot be parsed.
fn extract_links(html: &str) -> Result<Vec<String>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]")
        .map_err(|error| anyhow::anyhow!("invalid anchor selector: {error}"))?;

    Ok(document
        .select(&selector)
        .filter_map(|anchor| anchor.value().attr("href"))
        .map(str::to_owned)
        .collect())
}

/// Prints the links found on the Rust website.
///
/// # Errors
///
/// Returns an error if the request fails, the server returns an unsuccessful
/// status, or the selector cannot be parsed.
#[tokio::main]
async fn main() -> Result<()> {
    let html = reqwest::get("https://www.rust-lang.org/en-US/")
        .await?
        .error_for_status()?
        .text()
        .await?;

    for link in extract_links(&html)? {
        println!("{link}");
    }

    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
///
/// # Errors
///
/// Returns any request or HTML parsing error from the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::extract_links;
    use super::main;

    /// Verifies anchor links are selected while anchors without `href` are
    /// skipped.
    #[test]
    fn extracts_href_values_from_local_html() -> Result<()> {
        let html = r#"<a href="/first">one</a><a>empty</a><a href="https://example.test">two</a>"#;

        assert_eq!(extract_links(html)?, ["/first", "https://example.test"]);
        Ok(())
    }

    /// Marks the live Rust website request as an opt-in network test.
    #[test]
    #[ignore = "needs network"]
    fn require_network() -> Result<()> {
        main()?;
        Ok(())
    }
}
