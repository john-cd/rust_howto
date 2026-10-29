// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example demonstrating the use of `pulldown-cmark` to parse Markdown and
//! convert it to HTML.
//!
//! `pulldown-cmark` is a parser for `CommonMark`, a standard dialect of
//! Markdown. It uses an event-based parsing model. This means it generates a
//! stream of events representing the Markdown structure, which can then be used
//! to generate the HTML.

use pulldown_cmark::Options;
/// The `Parser` struct is the core component of `pulldown-cmark`. It takes
/// a Markdown string as input and produces a stream of events representing
/// the parsed Markdown structure.
use pulldown_cmark::Parser;
use pulldown_cmark::html;

/// Parses Markdown into events and renders them as HTML.
fn main() {
    let markdown_input = r#"
# Hello, Pulldown CMark!

This is a simple paragraph with **bold** and *italic* text.

* List item 1
* List item 2

```rust
/// Prints the greeting embedded in the Markdown code fence.
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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the pulldown cmark example through its entry point.
    #[test]
    fn test_pulldown_cmark_example_entry_point() {
        main();
    }

    /// Renders markdown with all extensions enabled.
    fn render(input: &str) -> String {
        let mut html_output = String::new();
        html::push_html(
            &mut html_output,
            Parser::new_ext(input, Options::all()),
        );
        html_output
    }

    /// Headings, lists and fenced code blocks are rendered.
    #[test]
    fn renders_blocks() {
        let html = render("# Hi\n\n* a\n* b\n\n```rust\nfn x() {}\n```\n");
        assert!(html.contains("<h1>Hi</h1>"), "{html}");
        assert!(html.contains("<li>b</li>"), "{html}");
        assert!(html.contains("<code class=\"language-rust\">"), "{html}");
    }

    /// Enabled extensions such as strikethrough are applied.
    #[test]
    fn extensions_are_enabled() {
        assert!(render("~~gone~~").contains("<del>gone</del>"));
    }

    /// Empty input renders nothing.
    #[test]
    fn empty_input_renders_nothing() {
        assert_eq!(render(""), "");
    }
}
