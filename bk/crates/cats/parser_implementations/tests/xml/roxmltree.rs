// ANCHOR: example
use roxmltree::Document;

// `roxmltree` represents an XML document as a read-only tree.
// Built on top of this API, a mapping to the Serde data model is available via
// the serde-roxmltree crate.

fn main() -> anyhow::Result<()> {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?><library><book id="1"><title>The Rust Programming Language</title><author>Steve Klabnik</author><author>Carol Nichols</author><year>2018</year><categories><category>Programming</category><category>Rust</category></categories></book></library>"#;

    // Parse the XML
    let doc = Document::parse(xml)?;

    // Get the root element
    let root = doc.root_element();
    println!("Root element: {}", root.tag_name().name());

    // Iterate through all books
    for book in root.children().filter(|n| n.has_tag_name("book")) {
        println!("\nBook ID: {}", book.attribute("id").unwrap_or("unknown"));

        // Get the title
        if let Some(title) = book.children().find(|n| n.has_tag_name("title")) {
            println!("Title: {}", title.text().unwrap_or(""));
        }

        // Get all authors
        let authors: Vec<&str> = book
            .children()
            .filter(|n| n.has_tag_name("author"))
            .filter_map(|n| n.text())
            .collect();

        println!("Authors: {}", authors.join(", "));

        // Get the year
        if let Some(year) = book.children().find(|n| n.has_tag_name("year")) {
            println!("Year: {}", year.text().unwrap_or(""));
        }

        // Get categories
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

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
