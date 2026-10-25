// ANCHOR: example
//! Selects HTML elements and extracts their text with CSS selectors.

use ::scraper::Html;
use ::scraper::Selector;

/// Parses a small HTML document and prints its article headings.
fn main() -> anyhow::Result<()> {
    let document = Html::parse_document(
        "
        <article>
            <h2>Parsing HTML</h2>
            <p>Use the parser that fits the task.</p>
            <h2>Selecting elements</h2>
        </article>
        ",
    );

    for heading in select_text(&document, "article h2")? {
        println!("{heading}");
    }

    Ok(())
}

/// Returns the text of elements that match a CSS selector.
fn select_text(document: &Html, query: &str) -> anyhow::Result<Vec<String>> {
    let selector = Selector::parse(query).map_err(|error| {
        anyhow::anyhow!("invalid CSS selector {query:?}: {error:?}")
    })?;
    let text = document
        .select(&selector)
        .map(|element| element.text().collect::<String>().trim().to_owned())
        .collect();

    Ok(text)
}

// ANCHOR_END: example

/// Runs the example for the command-line dispatcher and test suite.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use ::scraper::Html;

    use super::select_text;

    /// Selects only matching headings and preserves their document order.
    #[test]
    fn scraper_selects_html_by_css_selector() -> anyhow::Result<()> {
        let document = Html::parse_document(
            "<article><h2>First</h2><p>Skip</p><h2>Second</h2></article>",
        );

        let headings = select_text(&document, "article > h2")?;

        assert_eq!(headings, vec!["First".to_owned(), "Second".to_owned()]);
        Ok(())
    }
}
