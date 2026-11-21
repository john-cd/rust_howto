// TODO Reviewed ??-??-2026.
//! Replaces one node while preserving the original tree's trivia.

// ANCHOR: example
use rowan::GreenNode;
use rowan::GreenNodeBuilder;
use rowan::SyntaxKind;
use rowan::cursor::SyntaxNode;

const ROOT: SyntaxKind = SyntaxKind(0);
const TEXT: SyntaxKind = SyntaxKind(1);
const VALUE: SyntaxKind = SyntaxKind(2);
const NUMBER: SyntaxKind = SyntaxKind(3);
const WHITESPACE: SyntaxKind = SyntaxKind(4);
const COMMENT: SyntaxKind = SyntaxKind(5);

/// Builds a replacement value node from its number token.
fn value_node(text: &str) -> GreenNode {
    let mut builder = GreenNodeBuilder::new();
    builder.start_node(VALUE);
    builder.token(NUMBER, text);
    builder.finish_node();
    builder.finish()
}

/// Changes a value without rewriting the surrounding whitespace or comment.
///
/// # Errors
///
/// Returns an error if the fixture does not contain a value node.
fn main() -> anyhow::Result<()> {
    let mut builder = GreenNodeBuilder::new();
    builder.start_node(ROOT);
    builder.token(TEXT, "limit");
    builder.token(WHITESPACE, "  ");
    builder.token(TEXT, "=");
    builder.token(WHITESPACE, " ");
    builder.start_node(VALUE);
    builder.token(NUMBER, "3");
    builder.finish_node();
    builder.token(WHITESPACE, "  ");
    builder.token(COMMENT, "# retries");
    builder.token(WHITESPACE, "\n");
    builder.finish_node();

    let root = SyntaxNode::new_root(builder.finish());
    let value = root
        .children()
        .find(|node| node.kind() == VALUE)
        .ok_or_else(|| anyhow::anyhow!("fixture has no value node"))?;
    let edited = SyntaxNode::new_root(value.replace_with(value_node("12")));

    assert_eq!(root.to_string(), "limit  = 3  # retries\n");
    assert_eq!(edited.to_string(), "limit  = 12  # retries\n");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Verifies a node replacement preserves whitespace, comments, and the original tree.
    #[test]
    fn preserves_trivia_around_replacement() -> anyhow::Result<()> {
        super::main()
    }
}
