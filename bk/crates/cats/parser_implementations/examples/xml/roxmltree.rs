// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to parse an XML document using the `roxmltree`
//! crate.
//!
//! The `roxmltree` crate provides a read-only tree representation of an XML
//! document. It's a good choice for parsing XML when we don't need to modify
//! the document.
//!
//! Built on top of this API, a mapping to the Serde data model is available via
//! the serde-roxmltree crate.
//!
//! This example parses a simple XML document representing a library of books.

use roxmltree::Document;

/// Parses an XML document and prints its elements.
fn main() -> anyhow::Result<()> {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?><library><book id="1"><title>The Rust Programming Language</title><author>Steve Klabnik</author><author>Carol Nichols</author><year>2018</year><categories><category>Programming</category><category>Rust</category></categories></book></library>"#;

    // Parse the XML.
    let doc = Document::parse(xml)?;

    // Get the root element.
    let root = doc.root_element();
    println!("Root element: {}", root.tag_name().name());

    // Iterate through all books.
    for book in root.children().filter(|n| n.has_tag_name("book")) {
        println!("\nBook ID: {}", book.attribute("id").unwrap_or("unknown"));

        // Get the title.
        if let Some(title) = book.children().find(|n| n.has_tag_name("title")) {
            println!("Title: {}", title.text().unwrap_or(""));
        }

        // Get all authors.
        let authors: Vec<&str> = book
            .children()
            .filter(|n| n.has_tag_name("author"))
            .filter_map(|n| n.text())
            .collect();

        println!("Authors: {}", authors.join(", "));

        // Get the year.
        if let Some(year) = book.children().find(|n| n.has_tag_name("year")) {
            println!("Year: {}", year.text().unwrap_or(""));
        }

        // Get categories.
        if let Some(categories) =
            book.children().find(|n| n.has_tag_name("categories"))
        {
            let category_list: Vec<&str> = categories
                .children()
                .filter(|n| n.has_tag_name("category"))
                .filter_map(|n| n.text())
                .collect();

            println!("Categories: {}", category_list.join(", "));
        }
    }
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the roxmltree example through its entry point.
    #[test]
    fn test_roxmltree_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Elements and attributes can be navigated.
    #[test]
    fn navigates_the_tree() -> anyhow::Result<()> {
        let doc = Document::parse(r#"<a id="1"><b>x</b><b>y</b></a>"#)?;
        let root = doc.root_element();
        assert_eq!(root.tag_name().name(), "a");
        assert_eq!(root.attribute("id"), Some("1"));
        let texts: Vec<_> = root
            .children()
            .filter(|n| n.has_tag_name("b"))
            .filter_map(|n| n.text())
            .collect();
        assert_eq!(texts, ["x", "y"]);
        Ok(())
    }

    /// Mismatched tags are a parse error.
    #[test]
    fn malformed_xml_is_rejected() {
        assert!(Document::parse("<a><b></a>").is_err());
    }
}
