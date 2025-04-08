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

fn extract_links(content: &str) -> HashSet<Cow<str>> {
    lazy_static! {
        static ref WIKI_REGEX: Regex = Regex::new(
            r"(?x)
                \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
                |
                (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
            "
        )
        .unwrap();
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

#[tokio::main]
async fn main() -> Result<()> {
    let content = reqwest::get(
    "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
  )
  .await?
  .text()
  .await?;

    println!("{:#?}", extract_links(content.as_str()));

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> Result<()> {
    main()?;
    Ok(())
}
