// ANCHOR: example
//! This example demonstrates how to check for broken links on a website.
//!
//! `select` is a library to extract useful data from HTML documents, suitable
//! for web scraping.

use std::collections::HashSet;

use anyhow::Result;
use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
use url::Position;
use url::Url;

/// Extracts the base URL from a document.
///
/// If the HTML document contains a `<base>` tag, its `href` attribute is used
/// as the base URL. Otherwise, the base URL is derived from the URL of the
/// document itself, up to the path component.
async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
    let base_tag_href =
        doc.find(Name("base")).filter_map(|n| n.attr("href")).next();
    let base_url = base_tag_href
        .map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
    Ok(base_url)
}

/// Checks if a link is broken.
///
/// This function sends a GET request to the given URL and checks the HTTP
/// status code. If the status code is 404 (Not Found), the link is
/// considered broken. Otherwise, it's considered OK.
async fn check_link(url: &Url) -> Result<bool> {
    let res = reqwest::get(url.as_ref()).await?;
    Ok(res.status() != StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse("https://www.rust-lang.org/en-US/")?;
    let res = reqwest::get(url.as_ref()).await?.text().await?;
    let document = Document::from(res.as_str());
    let base_url = get_base_url(&url, &document).await?;
    let base_parser = Url::options().base_url(Some(&base_url));
    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();
    let mut tasks = vec![];

    for link in links {
        tasks.push(tokio::spawn(async move {
            if check_link(&link).await.unwrap() {
                println!("{} is OK", link);
            } else {
                println!("{} is Broken", link);
            }
        }));
    }

    for task in tasks {
        task.await?
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
