#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to extract all the links from a web page.
//!
//! `select` is a library to extract useful data from HTML documents, suitable
//! for web scraping.

use anyhow::Result;
use select::document::Document;
use select::predicate::Name;

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{x}"));

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
