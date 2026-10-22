#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Build successive paginated URLs for an HTTP API.

use anyhow::Result;
use url::Url;

/// The position in a paginated API.
#[derive(Clone, Copy, Debug)]
struct Pagination {
    /// The number of the page, starting at 1.
    page: u32,
    /// The number of items per page.
    per_page: u32,
}

impl Pagination {
    /// Returns the pagination of the next page.
    fn next(self) -> Self {
        Self {
            page: self.page + 1,
            per_page: self.per_page,
        }
    }

    /// Returns the URL of the page, with the pagination as query parameters of
    /// `base`.
    fn into_url(self, base: &str) -> Result<Url> {
        let mut url = Url::parse(base)?;
        url.query_pairs_mut()
            .append_pair("page", &self.page.to_string())
            .append_pair("per_page", &self.per_page.to_string());
        Ok(url)
    }
}

/// Prints the URLs of successive pages.
fn main() -> Result<()> {
    let mut pagination = Pagination {
        page: 1,
        per_page: 50,
    };

    for _ in 0..3 {
        let url = pagination.into_url("https://api.example.com/v1/items")?;
        println!("Fetching page {} via {url}", pagination.page);
        pagination = pagination.next();
    }

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

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
    // [fix - the API no longer returns a crate_id - need to get](https://github.com/john-cd/rust_howto/issues/860)
    // <https://github.com/rust-lang/crates.io/issues/856>
}
