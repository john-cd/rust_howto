#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `markdown` crate to parse Markdown
//! text and convert it to HTML.
//!
//! `markdown` is a simple native Rust library for parsing Markdown and
//! outputting HTML.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! markdown = "0.3" # You may also try 1.0.0-alpha.xx
//! ```
use markdown::to_html;

fn main() {
    let markdown_text = r#"
# Hello, Markdown!

This is a simple paragraph with **bold** and *italic* text.

* List item 1
* List item 2
"#;

    let html_output = to_html(markdown_text);
    // Also consider using: `markdown::file_to_html`, `markdown::tokenize`.

    println!("{html_output}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
