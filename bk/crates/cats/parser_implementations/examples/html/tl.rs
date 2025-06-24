#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `tl` crate to parse HTML.
use tl::Node;
use tl::Parser;
use tl::VDom;

fn main() -> anyhow::Result<()> {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>My Page</title>
        </head>
        <body>
            <h1 id="text">Hello, tl!</h1>
            <p class="content">This is a paragraph.</p>
            <ul>
                <li>Item 1</li>
                <li>Item 2</li>
            </ul>
        </body>
        </html>
    "#;

    // `tl::parse` accepts an HTML source code string and parses it into a DOM.
    // `tl` currently silently ignores tags that are invalid, sort of like
    // browsers do.
    let dom: VDom<'_> = tl::parse(html, tl::ParserOptions::default())?;
    // Underlying parser:
    let parser: &Parser<'_> = dom.parser();

    // Get element by id.
    // `handle` is a NodeHandle; it contains an identifier that uniquely
    // identifies an HTML node.
    if let Some(handle) = dom.get_element_by_id("text") {
        let element: &Node<'_> = handle
            .get(parser)
            .ok_or(anyhow::anyhow!("Failed to retrieve node."))?;
        assert_eq!(element.inner_text(parser), "Hello, tl!");
    }

    // Find the title.
    // Also try: `p.content`, `li`, `p[class]`.
    if let Some(handle) =
        dom.query_selector("title").and_then(|mut iter| iter.next())
    {
        let node = handle
            .get(parser)
            .ok_or(anyhow::anyhow!("Failed to retrieve node."))?;
        let title_text = node.inner_text(parser);
        println!("Title: {}", title_text);
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
