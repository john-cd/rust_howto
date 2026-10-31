// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `comrak` is a `CommonMark` and GitHub-flavored Markdown compatible parser.
//!
//! CommonMark is a precise specification of Markdown, which has many ambiguous
//! dialects. GitHub-flavored Markdown (GFM) adds tables, task lists and
//! strikethrough. `comrak` parses and renders both, for example to convert
//! Markdown to HTML.

use comrak::Arena;
use comrak::Options;
use comrak::format_html;
use comrak::markdown_to_html;
use comrak::nodes::NodeValue;
use comrak::parse_document;

/// Converts markdown to HTML.
fn to_html() {
    let markdown_input =
        "# Hello world\n\nThis is **bold** text and *italic* text.";
    let html_output = markdown_to_html(markdown_input, &Options::default());

    println!("{html_output}");
}

/// Replaces all instances of `orig_string` with `replacement` in the
/// markdown document.
///
/// # Arguments
/// * `document` - The markdown document to modify.
/// * `orig_string` - The string to replace.
fn replace_text(
    document: &str,
    orig_string: &str,
    replacement: &str,
) -> String {
    // The returned nodes are created in the supplied `Arena`, and are bound by
    // its lifetime.
    let arena = Arena::new();

    // Parse the document into a root `AstNode`.
    let root = parse_document(&arena, document, &Options::default());

    // Iterate over all the descendants of root, looking for text nodes.
    for node in root.descendants() {
        if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
            // If the node is a text node, perform the string replacement.
            *text = text.replace(orig_string, replacement).into();
        }
    }

    let mut html = String::new();
    format_html(root, &Options::default(), &mut html)
        .expect("failed to format HTML");

    html
}

/// Converts Markdown to HTML and replaces text in a Markdown document.
fn main() {
    to_html();

    let doc = "This is my input.\n\n1. Also [my](#) input.\n2. Certainly *my* input.\n";
    let orig = "my";
    let repl = "your";
    let html = replace_text(doc, orig, repl);

    println!("{html}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the comrak example through its entry point.
    #[test]
    fn test_comrak_example_entry_point() {
        main();
    }

    /// Text nodes are replaced, markup is kept.
    #[test]
    fn replaces_text_in_the_document() {
        let html = replace_text(
            "This is my input.\n\n1. Also [my](#) input.\n",
            "my",
            "your",
        );
        assert!(html.contains("<p>This is your input.</p>"), "{html}");
        assert!(html.contains("<a href=\"#\">your</a>"), "{html}");
    }

    /// A document without the text is unchanged.
    #[test]
    fn no_match_leaves_text_unchanged() {
        let html = replace_text("plain *text*", "absent", "x");
        assert_eq!(html.trim(), "<p>plain <em>text</em></p>");
    }

    /// Markdown is converted to HTML.
    #[test]
    fn converts_markdown_to_html() {
        let html = markdown_to_html("# Hi\n\n**b**", &Options::default());
        assert!(html.contains("<h1>Hi</h1>"));
        assert!(html.contains("<strong>b</strong>"));
    }
}
