// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Parse Rust source code and query its syntax tree with Tree-sitter.
//!
//! Tree-sitter builds a concrete syntax tree from source code, quickly and
//! tolerating errors, and can update it as the text changes. Editors use it for
//! syntax highlighting, code folding and navigation. Queries match patterns in
//! the tree.

use anyhow::anyhow;
use tree_sitter::Language;
use tree_sitter::Parser;
use tree_sitter::Query;
use tree_sitter::QueryCursor;
use tree_sitter::StreamingIterator as _;

/// Parses Rust source code and returns the names captured by the example query.
fn query_rust(source: &str) -> anyhow::Result<Vec<(String, String)>> {
    let language: Language = tree_sitter_rust::LANGUAGE.into();
    let mut parser = Parser::new();
    parser.set_language(&language)?;

    let tree = parser
        .parse(source, None)
        .ok_or_else(|| anyhow!("failed to parse source"))?;
    let query = Query::new(
        &language,
        "
		(function_item name: (identifier) @function)
		(let_declaration pattern: (identifier) @binding)
		",
    )?;
    let capture_names = query.capture_names();
    let mut cursor = QueryCursor::new();
    let mut captures = Vec::new();

    let mut matches =
        cursor.matches(&query, tree.root_node(), source.as_bytes());
    while let Some(matched) = matches.next() {
        for capture in matched.captures() {
            let label = &capture_names[capture.index as usize];
            let node = capture.node;
            let text = &source[node.start_byte()..node.end_byte()];
            captures.push(((*label).to_owned(), text.to_owned()));
        }
    }

    Ok(captures)
}

/// Parses Rust source code, prints its syntax tree and query captures.
fn main() -> anyhow::Result<()> {
    let source = r#"
/// Greets the supplied user.
fn greet() {
    let message = "hello";
    println!("{message}");
}
"#;
    let language: Language = tree_sitter_rust::LANGUAGE.into();
    let mut parser = Parser::new();
    parser.set_language(&language)?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| anyhow!("failed to parse source"))?;
    println!("{}", tree.root_node().to_sexp());

    for (label, text) in query_rust(source)? {
        println!("{label}: {text}");
    }

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the query identifies function and local binding names.
    #[test]
    fn query_captures_function_and_binding_names() -> anyhow::Result<()> {
        let source = "fn greet() { let message = \"hello\"; }";
        let mut captures = query_rust(source)?;
        captures.sort();

        assert_eq!(
            captures,
            [
                ("binding".to_owned(), "message".to_owned()),
                ("function".to_owned(), "greet".to_owned()),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
