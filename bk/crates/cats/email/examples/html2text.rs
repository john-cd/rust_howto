// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Converts an HTML email body to wrapped plain text with `html2text`.

/// Converts the sample HTML email and checks its text alternative.
fn main() -> anyhow::Result<()> {
    let html = r#"<html><body>
        <h1>Order shipped</h1>
        <p>Your order is on its way. Track it <a href="https://example.com/track/42">here</a>.</p>
        <ul><li>1 x Rust book</li><li>2 x Stickers</li></ul>
    </body></html>"#;

    // Wrap lines at 60 columns.
    let text = html2text::from_read(html.as_bytes(), 60)?;
    println!("{text}");

    assert!(text.contains("Order shipped"));
    // Links are kept as footnote-style references.
    assert!(text.contains("https://example.com/track/42"));
    // List items survive.
    assert!(text.contains("Rust book"));
    assert!(text.contains("Stickers"));
    Ok(())
}
// ANCHOR_END: example

#[allow(clippy::main_recursion)]
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the email text preserves its heading, link, and list.
    #[test]
    fn html2text_example_runs() -> anyhow::Result<()> {
        main()
    }
}
