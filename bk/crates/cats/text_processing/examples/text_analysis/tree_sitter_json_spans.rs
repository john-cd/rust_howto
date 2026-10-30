// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Classifies JSON keys, strings, numbers, and literals into byte spans with
//! the Tree-sitter JSON grammar.

use anyhow::anyhow;
use tree_sitter::Node;
use tree_sitter::Parser;

/// A token class and its half-open byte range in the source.
type Span = (&'static str, usize, usize);

/// Walks JSON nodes and records the token ranges a highlighter can style.
fn collect_spans(node: Node<'_>, spans: &mut Vec<Span>) {
    match node.kind() {
        "pair" => {
            if let Some(key) = node.child_by_field_name("key") {
                spans.push(("key", key.start_byte(), key.end_byte()));
            }
            if let Some(value) = node.child_by_field_name("value") {
                collect_spans(value, spans);
            }
        }
        "string" => spans.push(("string", node.start_byte(), node.end_byte())),
        "number" => spans.push(("number", node.start_byte(), node.end_byte())),
        "true" | "false" | "null" => {
            spans.push(("literal", node.start_byte(), node.end_byte()));
        }
        _ => {
            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                collect_spans(child, spans);
            }
        }
    }
}

/// Parses JSON and returns byte ranges for the supported highlight classes.
fn json_spans(source: &str) -> anyhow::Result<Vec<Span>> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_json::LANGUAGE.into())?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| anyhow!("failed to parse JSON"))?;
    if tree.root_node().has_error() {
        anyhow::bail!("input contains invalid JSON");
    }
    let mut spans = Vec::new();
    collect_spans(tree.root_node(), &mut spans);
    Ok(spans)
}

/// Prints the token spans from a small JSON document.
fn main() -> anyhow::Result<()> {
    let source = r#"{"name":"Ada","age":37,"active":true,"note":null}"#;
    let spans = json_spans(source)?;
    for (class, start, end) in &spans {
        println!("{class:8} {:?}", &source[*start..*end]);
    }

    let tokens: Vec<_> = spans
        .iter()
        .map(|(class, start, end)| (*class, &source[*start..*end]))
        .collect();
    assert!(tokens.contains(&("key", "\"name\"")));
    assert!(tokens.contains(&("string", "\"Ada\"")));
    assert!(tokens.contains(&("number", "37")));
    assert!(tokens.contains(&("literal", "true")));
    assert!(tokens.contains(&("literal", "null")));
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that invalid JSON is reported instead of returning spans.
    #[test]
    fn invalid_json_is_rejected() {
        assert!(json_spans(r#"{"name": }"#).is_err());
    }

    /// Checks that the complete highlighting example parses and classifies
    /// JSON.
    #[test]
    fn highlights_json_tokens() -> anyhow::Result<()> {
        main()
    }
}
