// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Classifies source text into highlight spans with Tree-sitter, then updates
//! the syntax tree incrementally after an edit.

use anyhow::anyhow;
use tree_sitter::InputEdit;
use tree_sitter::Node;
use tree_sitter::Parser;
use tree_sitter::Point;
use tree_sitter::Tree;

/// A highlight class and the byte range it covers.
type Span = (&'static str, usize, usize);

/// Walks the tree and records the byte range of every node that a
/// highlighter would color.
fn highlight_spans(node: Node<'_>, out: &mut Vec<Span>) {
    let class = match node.kind() {
        "line_comment" | "block_comment" => Some("comment"),
        "string_literal" => Some("string"),
        "integer_literal" | "float_literal" => Some("number"),
        "fn" | "let" => Some("keyword"),
        _ => None,
    };
    if let Some(class) = class {
        out.push((class, node.start_byte(), node.end_byte()));
        // Do not descend: the children of a string are part of the string.
        if node.is_named() {
            return;
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        highlight_spans(child, out);
    }
}

/// Parses `source` and returns the tree and its highlight spans.
fn parse(
    parser: &mut Parser,
    source: &str,
    old_tree: Option<&Tree>,
) -> anyhow::Result<(Tree, Vec<Span>)> {
    let tree = parser
        .parse(source, old_tree)
        .ok_or_else(|| anyhow!("failed to parse source"))?;
    let mut spans = Vec::new();
    highlight_spans(tree.root_node(), &mut spans);
    Ok((tree, spans))
}

fn main() -> anyhow::Result<()> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into())?;

    let source = "fn main() { let n = 1; } // done";
    let (mut tree, spans) = parse(&mut parser, source, None)?;
    for (class, start, end) in &spans {
        println!("{class:8} {:?}", &source[*start..*end]);
    }
    assert!(spans.contains(&("keyword", 0, 2)));
    assert!(spans.contains(&("number", 20, 21)));

    // Incremental parsing: describe the edit to the old tree, then reparse.
    // Tree-sitter reuses the unchanged subtrees.
    let edited = "fn main() { let n = 100; } // done";
    tree.edit(&InputEdit {
        start_byte: 21,
        old_end_byte: 21,
        new_end_byte: 23,
        start_position: Point::new(0, 21),
        old_end_position: Point::new(0, 21),
        new_end_position: Point::new(0, 23),
    });
    let (new_tree, new_spans) = parse(&mut parser, edited, Some(&tree))?;
    assert!(new_spans.contains(&("number", 20, 23)));
    assert!(!new_tree.root_node().has_error());
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the tree sitter spans example through its entry point.
    #[test]
    fn test_tree_sitter_spans_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
