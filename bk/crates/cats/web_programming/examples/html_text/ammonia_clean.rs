// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Removes dangerous markup from untrusted HTML with `ammonia`.

use ammonia::Builder;

fn main() -> anyhow::Result<()> {
    let dirty = r#"<p onclick="steal()">Hi<script>alert(1)</script>
        <a href="javascript:steal()">bad</a>
        <a href="https://example.com/">good</a></p>"#;

    // `clean` uses a conservative allow list of tags, attributes and URL
    // schemes.
    let clean = ammonia::clean(dirty);
    println!("{clean}");
    assert!(!clean.contains("<script"));
    assert!(!clean.contains("onclick"));
    assert!(!clean.contains("javascript:"));
    assert!(clean.contains("https://example.com/"));

    // A custom `Builder` narrows or widens the policy.
    let strict = Builder::new()
        .tags(["p", "a", "b"].into())
        .url_schemes(["https"].into())
        .link_rel(Some("noopener noreferrer nofollow"))
        .clean("<p><b>bold</b> <i>italic</i> <a href=\"http://example.com/\">x</a></p>")
        .to_string();
    println!("{strict}");
    assert!(!strict.contains("<i>"));
    // The `http:` URL is not an allowed scheme, so the link loses its href.
    assert!(!strict.contains("href"));
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
}
