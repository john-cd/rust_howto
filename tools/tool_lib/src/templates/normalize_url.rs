use regex::Regex;

/// Normalizes a documentation URL from `docs.rs`.
///
/// This function cleans up and standardizes URLs from `docs.rs`. It handles various formats,
/// including those with version numbers, crate prefixes, and function paths.
///
/// # Examples
///
/// *   `https://docs.rs/quote/latest/quote` becomes `https://docs.rs/quote`
/// *   `https://docs.rs/crate/termbook` becomes `https://docs.rs/termbook`
/// *   `https://docs.rs/tungstenite/0.24.0` becomes `https://docs.rs/tungstenite`
pub fn normalize_docs_url(url: &str) -> std::borrow::Cow<'_, str> {
    let re1: &Regex =
        crate::regex!(r"http(s)?://docs.rs/(?:crate/)?(?<crt>[A-Za-z0-9_-]+)(?:/.*)?");
    // If no match is found, then the haystack is returned unchanged.
    re1.replace(url, "https://docs.rs/${crt}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_docs_url() {
        assert_eq!(
            normalize_docs_url("https://docs.rs/approx"),
            "https://docs.rs/approx"
        );
        assert_eq!(
            normalize_docs_url("http://docs.rs/prost"),
            "https://docs.rs/prost"
        );
        assert_eq!(
            normalize_docs_url("https://docs.rs/serde/"),
            "https://docs.rs/serde"
        );
        assert_eq!(
            normalize_docs_url("https://docs.rs/crate/termbook"),
            "https://docs.rs/termbook"
        );
        assert_eq!(
            normalize_docs_url("http://docs.rs/crate/doc-comment"),
            "https://docs.rs/doc-comment"
        );
        assert_eq!(
            normalize_docs_url("https://docs.rs/tungstenite/0.24.0"),
            "https://docs.rs/tungstenite"
        );
        assert_eq!(
            normalize_docs_url("https://docs.rs/quote/latest/quote"),
            "https://docs.rs/quote"
        );
        assert_eq!(
            normalize_docs_url("https://docs.rs/watchmaker/0.1.0/watchmaker/fn.solve.html"),
            "https://docs.rs/watchmaker"
        );
    }
}
