// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Encode and decode forms and path segments.
//!
//! Web forms and URLs can only carry a limited set of characters.
//! Percent-encoding replaces the others with `%` followed by their hex value (a
//! space becomes `%20`, or `+` in form data). Encode values before putting them
//! in a URL.

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

/// A form with a few fields.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Search {
    /// The search terms.
    q: String,
    /// The page number.
    page: u32,
}

/// Serializes a struct to a query string or form body, and parses it back.
pub fn run() -> anyhow::Result<()> {
    let form = Search {
        q: "rust & ferris".to_owned(),
        page: 2,
    };
    // Spaces become `+`, and reserved characters are percent-encoded.
    let body = serde_urlencoded::to_string(&form)?;
    assert_eq!(body, "q=rust+%26+ferris&page=2");

    // Parse it back into the struct, or into a map.
    let parsed: Search = serde_urlencoded::from_str(&body)?;
    assert_eq!(parsed, form);
    let map: HashMap<String, String> = serde_urlencoded::from_str(&body)?;
    assert_eq!(map["q"], "rust & ferris");

    // To encode a single path segment, `urlencoding` escapes everything
    // except unreserved characters. A space becomes `%20`, not `+`.
    let segment = urlencoding::encode("reports/2026 Q1");
    assert_eq!(segment, "reports%2F2026%20Q1");
    assert_eq!(urlencoding::decode(&segment)?, "reports/2026 Q1");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_succeeds() -> anyhow::Result<()> {
        run()
    }
}
