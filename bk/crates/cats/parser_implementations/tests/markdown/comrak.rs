// ANCHOR: example
use comrak::Arena;
use comrak::ComrakOptions;
use comrak::Options;
use comrak::format_html;
use comrak::markdown_to_html;
use comrak::nodes::NodeValue;
use comrak::parse_document;

// `comrak` is a CommonMark and GitHub-flavored Markdown compatible parser.

fn to_html() {
    let markdown_input =
        "# Hello world\n\nThis is **bold** text and *italic* text.";
    let html_output =
        markdown_to_html(markdown_input, &ComrakOptions::default());

    println!("{}", html_output);
}

fn replace_text(
    document: &str,
    orig_string: &str,
    replacement: &str,
) -> String {
    // The returned nodes are created in the supplied Arena, and are bound by
    // its lifetime.
    let arena = Arena::new();

    // Parse the document into a root `AstNode`
    let root = parse_document(&arena, document, &Options::default());

    // Iterate over all the descendants of root.
    for node in root.descendants() {
        if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
            // If the node is a text node, perform the string replacement.
            *text = text.replace(orig_string, replacement);
        }
    }

    let mut html = vec![];
    format_html(root, &Options::default(), &mut html).unwrap();

    String::from_utf8(html).unwrap()
}

fn main() {
    to_html();

    let doc = "This is my input.\n\n1. Also [my](#) input.\n2. Certainly *my* input.\n";
    let orig = "my";
    let repl = "your";
    let html = replace_text(doc, orig, repl);

    println!("{}", html);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
