#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Parse Rust source code and query its syntax tree with Tree-sitter.

use anyhow::anyhow;
use tree_sitter::Language;
use tree_sitter::Parser;
use tree_sitter::Query;
use tree_sitter::QueryCursor;
use tree_sitter::StreamingIterator as _;

/// Parses Rust source code and queries the syntax tree.
fn main() -> anyhow::Result<()> {
    let language: Language = tree_sitter_rust::LANGUAGE.into();
    let mut parser = Parser::new();
    parser.set_language(&language)?;

    let source = r#"
		fn greet() {
			let message = "hello";
			println!("{message}");
		}
	"#;

    let tree = parser
        .parse(source, None)
        .ok_or_else(|| anyhow!("failed to parse source"))?;
    println!("{}", tree.root_node().to_sexp());

    let query = Query::new(
        &language,
        "
		(function_item name: (identifier) @function)
		(let_declaration pattern: (identifier) @binding)
		",
    )?;
    let capture_names = query.capture_names();
    let mut cursor = QueryCursor::new();

    let mut matches =
        cursor.matches(&query, tree.root_node(), source.as_bytes());
    while let Some(matched) = matches.next() {
        for capture in matched.captures {
            let label = &capture_names[capture.index as usize];
            let node = capture.node;
            let text = &source[node.start_byte()..node.end_byte()];
            println!("{label}: {text}");
        }
    }

    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/827)
}
