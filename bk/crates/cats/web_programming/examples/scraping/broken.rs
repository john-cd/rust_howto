#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Checks HTTP links from an HTML page against their resolved URLs.

use std::collections::HashSet;
use std::time::Duration;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::bail;
use reqwest::Client;
use reqwest::StatusCode;
use scraper::Html;
use scraper::Selector;
use url::Url;

/// Resolves the first valid HTML `<base href>` against the document URL.
///
/// If the page has no base element, the document URL itself is the base.
///
/// # Errors
///
/// Returns an error if the base selector is invalid or the base href cannot be
/// resolved against the document URL.
fn get_base_url(document_url: &Url, document: &Html) -> Result<Url> {
    let selector = Selector::parse("base[href]")
        .map_err(|error| anyhow::anyhow!("invalid base selector: {error}"))?;
    let Some(href) = document
        .select(&selector)
        .find_map(|element| element.value().attr("href"))
    else {
        return Ok(document_url.clone());
    };

    document_url
        .join(href)
        .context("failed to resolve the HTML base href")
}

/// Resolves HTTP and HTTPS anchor links in the page against its base URL.
///
/// Unresolvable links and non-HTTP schemes such as `mailto:` are skipped.
///
/// # Errors
///
/// Returns an error if the anchor selector is invalid.
fn collect_http_links(base_url: &Url, document: &Html) -> Result<HashSet<Url>> {
    let selector = Selector::parse("a[href]")
        .map_err(|error| anyhow::anyhow!("invalid anchor selector: {error}"))?;

    Ok(document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .filter_map(|href| base_url.join(href).ok())
        .filter(|url| matches!(url.scheme(), "http" | "https"))
        .collect())
}

/// Checks one link and classifies success, not-found, and gone responses.
///
/// # Errors
///
/// Returns an error if the request fails or the server returns a status other
/// than success, 404, or 410.
async fn check_link(client: &Client, url: &Url) -> Result<bool> {
    let status = client
        .get(url.clone())
        .send()
        .await
        .with_context(|| format!("request failed for {url}"))?
        .status();

    if status.is_success() {
        return Ok(true);
    }
    if matches!(status, StatusCode::NOT_FOUND | StatusCode::GONE) {
        return Ok(false);
    }

    bail!("unexpected HTTP status {status} while checking {url}");
}

/// Checks links on a web page one at a time and prints their status.
///
/// # Errors
///
/// Returns an error if fetching the page, parsing links, building the client,
/// or checking a link fails.
#[tokio::main]
async fn main() -> Result<()> {
    let document_url = Url::parse("https://www.rust-lang.org/en-US/")?;
    let html = reqwest::get(document_url.clone())
        .await?
        .error_for_status()?
        .text()
        .await?;
    let document = Html::parse_document(&html);
    let base_url = get_base_url(&document_url, &document)?;
    let mut links: Vec<_> = collect_http_links(&base_url, &document)?
        .into_iter()
        .collect();
    links.sort_by(|left, right| left.as_str().cmp(right.as_str()));

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    for link in links {
        if check_link(&client, &link).await? {
            println!("{link} is OK");
        } else {
            println!("{link} is Broken");
        }
    }

    Ok(())
}

// ANCHOR_END: example

/// # Errors
///
/// Returns any request, parsing, or link-checking error from the example.
pub fn run() -> Result<()> {
    main()
}

// [flaky test](https://github.com/john-cd/rust_howto/issues/1419)

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use scraper::Html;
    use url::Url;

    use super::collect_http_links;
    use super::get_base_url;
    use super::main;

    /// Verifies relative links resolve against a relative base href.
    #[test]
    fn resolves_base_href_and_filters_non_http_links() -> Result<()> {
        let document_url =
            Url::parse("https://example.test/articles/item.html")?;
        let document = Html::parse_document(
            r#"<base href="../docs/"><a href="guide.html">guide</a><a href="/about">about</a><a href="mailto:help@example.test">email</a>"#,
        );
        let base_url = get_base_url(&document_url, &document)?;
        let links = collect_http_links(&base_url, &document)?;

        assert_eq!(base_url.as_str(), "https://example.test/docs/");
        assert!(
            links
                .contains(&Url::parse("https://example.test/docs/guide.html")?)
        );
        assert!(links.contains(&Url::parse("https://example.test/about")?));
        assert_eq!(links.len(), 2);
        Ok(())
    }

    /// Verifies pages without a base element retain the full document URL.
    #[test]
    fn defaults_base_to_document_url() -> Result<()> {
        let document_url =
            Url::parse("https://example.test/articles/item.html?version=1")?;
        let document = Html::parse_document("<a href=next.html>next</a>");

        assert_eq!(get_base_url(&document_url, &document)?, document_url);
        Ok(())
    }

    /// Marks the external page scan as an opt-in network test.
    #[test]
    #[ignore = "needs network"]
    fn require_network() -> Result<()> {
        main()?;
        Ok(())
    }
}
