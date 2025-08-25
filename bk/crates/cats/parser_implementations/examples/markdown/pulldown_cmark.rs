#![allow(dead_code)]
// ANCHOR: example
//! Example demonstrating the use of `pulldown-cmark` to parse Markdown and
//! convert it to HTML.
//!
//! `pulldown-cmark` is a parser for CommonMark, a standard dialect of Markdown.
//! It uses an event-based parsing model. This means it generates a stream of
//! events representing the Markdown structure, which can then be used to
//! generate the HTML.

use pulldown_cmark::Options;
/// The `Parser` struct is the core component of `pulldown-cmark`. It takes
/// a Markdown string as input and produces a stream of events representing
/// the parsed Markdown structure.
use pulldown_cmark::Parser;
use pulldown_cmark::html;

fn main() {
    let markdown_input = r#"
# Hello, Pulldown CMark!

This is a simple paragraph with **bold** and *italic* text.

* List item 1
* List item 2

```rust
fn example() {
    println!("Hello from Rust!");
}
```
"#;
    // Set up parser options.
    //
    // Enable or disable various Markdown extensions through the
    // `Options` struct to get fine-grained control over the parsing
    // process. Here, we enable all common extensions.
    let options = Options::all();

    // `Parser::new_ext(markdown_input, options)` creates a `Parser` instance.
    let parser = Parser::new_ext(markdown_input, options);

    // Render the parser output to HTML.
    // `html::push_html(&mut html_output, parser)` takes the parser and an empty
    // `String` (`html_output`) and renders the Markdown content as HTML. The
    // `push_html` function iterates through the parser's events and appends the
    // corresponding HTML to the output string.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    println!("{html_output}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
