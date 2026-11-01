// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Renders Markdown to HTML, then sanitizes the result.
//!
//! Markdown allows raw HTML, so sanitizing the *Markdown source* is not
//! enough: always sanitize the *HTML output*.

use pulldown_cmark::Parser;
use pulldown_cmark::html;

/// Converts untrusted Markdown to HTML that is safe to embed in a page.
fn render_safe(markdown: &str) -> String {
    let mut unsafe_html = String::new();
    html::push_html(&mut unsafe_html, Parser::new(markdown));
    ammonia::clean(&unsafe_html)
}

/// Renders Markdown to HTML, then sanitizes the result before use.
fn main() -> anyhow::Result<()> {
    let markdown = concat!(
        "# Title\n\n",
        "Some *emphasis* and <script>alert(1)</script>\n\n",
        "[click](javascript:steal())\n"
    );
    let html = render_safe(markdown);
    println!("{html}");

    assert!(html.contains("<h1>Title</h1>"));
    assert!(html.contains("<em>emphasis</em>"));
    assert!(!html.contains("<script"));
    assert!(!html.contains("javascript:"));
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that rendered Markdown is sanitized before it is returned.
    /// Runs the markdown to safe html example through its entry point.
    #[test]
    fn test_markdown_to_safe_html_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
