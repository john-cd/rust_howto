// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Wrap a paginated API (crates.io reverse dependencies) in an iterator.
//!
//! APIs return large result sets in pages, to bound response size. A client
//! requests a page, reads the cursor or next-page number from the response, and
//! asks for the next one. Wrapping this in an iterator hides the paging from
//! the caller.

use std::collections::HashMap;

use anyhow::Result;
use reqwest::blocking::Client;
use serde::Deserialize;

/// One page of the response of
/// `GET /api/v1/crates/{name}/reverse_dependencies`.
#[derive(Debug, Deserialize)]
struct Page {
    /// The dependency edges. `crate_id` is the crate being depended upon (the
    /// one queried), NOT the dependent crate.
    dependencies: Vec<Dependency>,
    /// The versions of the dependent crates.
    versions: Vec<Version>,
    /// The total number of reverse dependencies.
    meta: Meta,
}

/// A dependency edge.
#[derive(Debug, Deserialize)]
struct Dependency {
    /// The id of the version of the dependent crate that declares this edge.
    version_id: u64,
}

/// A version of a dependent crate.
#[derive(Debug, Deserialize)]
struct Version {
    /// The id of the version.
    id: u64,
    /// The name of the dependent crate.
    #[serde(rename = "crate")]
    crate_name: String,
}

/// Pagination metadata.
#[derive(Debug, Deserialize)]
struct Meta {
    /// The total number of items, across all pages.
    total: u64,
}

impl Page {
    /// Returns the names of the dependent crates on this page.
    ///
    /// The `dependencies` entries no longer carry the dependent crate's name:
    /// look it up through `version_id` in the `versions` array.
    fn dependent_names(&self) -> Vec<&str> {
        let names: HashMap<u64, &str> = self
            .versions
            .iter()
            .map(|v| (v.id, v.crate_name.as_str()))
            .collect();
        self.dependencies
            .iter()
            .filter_map(|d| names.get(&d.version_id).copied())
            .collect()
    }
}

/// Fetches one page (1-based) from crates.io.
#[expect(
    dead_code,
    reason = "makes real network requests; `main` uses canned pages instead"
)]
fn fetch_page(client: &Client, name: &str, page: u64) -> Result<Page> {
    let url = format!(
        "https://crates.io/api/v1/crates/{name}/reverse_dependencies?page={page}&per_page=100"
    );
    // crates.io requires a `User-Agent` header, which identifies the client.
    Ok(client
        .get(url)
        .header("User-Agent", "rust_howto-example")
        .send()?
        .error_for_status()?
        .json()?)
}

/// Iterates over every dependent crate, fetching pages lazily with `fetch`.
struct ReverseDependencies<F: FnMut(u64) -> Result<Page>> {
    /// Fetches the page with the given number.
    fetch: F,
    /// The number of the next page to fetch.
    next_page: u64,
    /// Names from the current page not yet yielded (in reverse order).
    buffer: Vec<String>,
    /// The number of items fetched so far.
    seen: u64,
    /// Whether the last page was reached or an error was returned.
    done: bool,
}

impl<F: FnMut(u64) -> Result<Page>> ReverseDependencies<F> {
    /// Creates the iterator.
    fn new(fetch: F) -> Self {
        Self {
            fetch,
            next_page: 1,
            buffer: Vec::new(),
            seen: 0,
            done: false,
        }
    }
}

impl<F: FnMut(u64) -> Result<Page>> Iterator for ReverseDependencies<F> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() && !self.done {
            match (self.fetch)(self.next_page) {
                Ok(page) => {
                    self.buffer = page
                        .dependent_names()
                        .into_iter()
                        .rev()
                        .map(String::from)
                        .collect();
                    self.next_page += 1;
                    self.seen += page.dependencies.len() as u64;
                    // Stop on an empty page, or once all items were seen.
                    self.done =
                        self.buffer.is_empty() || self.seen >= page.meta.total;
                }
                Err(e) => {
                    self.done = true;
                    return Some(Err(e));
                }
            }
        }
        self.buffer.pop().map(Ok)
    }
}

/// Prints dependent crates, using canned pages instead of the network.
fn main() -> Result<()> {
    let pages = [
        r#"{"dependencies":[{"version_id":1,"crate_id":"serde"},{"version_id":2,"crate_id":"serde"}],
            "versions":[{"id":1,"crate":"hashbrown"},{"id":2,"crate":"toml"}],
            "meta":{"total":3}}"#,
        r#"{"dependencies":[{"version_id":3,"crate_id":"serde"}],
            "versions":[{"id":3,"crate":"url"}],
            "meta":{"total":3}}"#,
    ];
    let fetch = |page: u64| -> Result<Page> {
        // Replace with `fetch_page(&Client::new(), "serde", page)` to query
        // crates.io.
        let json = pages.get(page as usize - 1).copied().unwrap_or(
            r#"{"dependencies":[],"versions":[],"meta":{"total":3}}"#,
        );
        Ok(serde_json::from_str(json)?)
    };

    for name in ReverseDependencies::new(fetch) {
        println!("{}", name?);
    }
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the paginated example through its entry point.
    #[test]
    fn test_paginated_example_entry_point() -> Result<()> {
        main()
    }

    /// All pages are walked, in order, and the walk stops at the total.
    #[test]
    fn walks_every_page() -> Result<()> {
        let mut requested = Vec::new();
        let names = ReverseDependencies::new(|page| {
            requested.push(page);
            let json = match page {
                1 => {
                    r#"{"dependencies":[{"version_id":1},{"version_id":2}],
                       "versions":[{"id":1,"crate":"a"},{"id":2,"crate":"b"}],
                       "meta":{"total":3}}"#
                }
                _ => {
                    r#"{"dependencies":[{"version_id":3}],
                       "versions":[{"id":3,"crate":"c"}],
                       "meta":{"total":3}}"#
                }
            };
            Ok(serde_json::from_str(json)?)
        })
        .collect::<Result<Vec<_>>>()?;
        assert_eq!(names, ["a", "b", "c"]);
        assert_eq!(requested, [1, 2]);
        Ok(())
    }

    /// A failing fetch is yielded once, then iteration ends.
    #[test]
    fn fetch_error_ends_iteration() {
        let mut iterator =
            ReverseDependencies::new(|_| anyhow::bail!("network down"));
        assert!(iterator.next().is_some_and(|item| item.is_err()));
        assert!(iterator.next().is_none());
    }

    /// Dependencies whose version is unknown are skipped.
    #[test]
    fn unknown_versions_are_skipped() -> Result<(), Box<dyn std::error::Error>>
    {
        let page: Page = serde_json::from_str(
            r#"{"dependencies":[{"version_id":9}],"versions":[],"meta":{"total":1}}"#,
        )?;
        assert!(page.dependent_names().is_empty());
        Ok(())
    }
}
