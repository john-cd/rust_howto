// ANCHOR: example
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use pulldown_cmark::html;

// `pulldown-cmark` is a parser for CommonMark, a standard dialect of Markdown.
// `pulldown-cmark` uses an event-based parsing model. This means it generates a
// stream of events representing the Markdown structure, which are then used to
// generate the HTML.

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
    // You can enable or disable various Markdown extensions through the
    // `Options` struct, giving you fine-grained control over the parsing
    // process. Here, we enable all common extensions.
    let options = Options::all();

    // `Parser::new_ext(markdown_input, options)` creates a `Parser` instance
    // from the Markdown input string and the specified options. This parser
    // will iterate through the Markdown tokens.
    let parser = Parser::new_ext(markdown_input, options);

    // Render the parser output to HTML.
    // `html::push_html(&mut html_output, parser)` takes the parser and an empty
    // `String` (`html_output`) and renders the Markdown content as HTML. The
    // `push_html` function iterates through the parser's events and appends the
    // corresponding HTML to the output string.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    println!("{}", html_output);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
