#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates `pest` parsing a flat arithmetic expression
//! into alternating number and operator tokens.
//!
//! In `Cargo.toml`, add:
//! ```toml
//! [dependencies]
//! pest = "2.7.15"
//! pest_derive = "2.7.15"
//!
//! [build-dependencies]
//! pest_generator = "2.7.15"
//! ```
//! The `grammar.pest` file defines the grammar rules, and the ``.rs` file
//! uses the generated parser to parse the input and print the parsed pairs.
//!
//! `pest` uses parsing expression grammars (or PEG) as its input, which are
//! similar in spirit to regular expressions, but which offer the enhanced
//! expressivity needed to parse complex languages.

use pest::Parser as _;
use pest_derive::Parser;

/// A parser for arithmetic expressions, generated from a `pest` grammar.
#[derive(Parser)]
#[grammar = "../examples/pest/grammar.pest"] // Path relative to `src`
struct ArithmeticParser;

/// Parses an arithmetic expression into its number and operator tokens.
fn parse_expression(input: &str) -> anyhow::Result<Vec<String>> {
    let mut pairs = ArithmeticParser::parse(Rule::expression, input)?;
    let expression = pairs
        .next()
        .expect("the expression rule produces one outer pair");

    Ok(expression
        .into_inner()
        .filter(|pair| pair.as_rule() != Rule::EOI)
        .map(|pair| pair.as_str().to_owned())
        .collect())
}

/// Parses an arithmetic expression and prints its tokens.
fn main() -> anyhow::Result<()> {
    let input = "3 + 5 * 2 - 8 / 4";
    for token in parse_expression(input)? {
        println!("{token}");
    }

    Ok(())
}

// ANCHOR_END: example

/// Runs the example and propagates parse errors.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies a valid expression becomes alternating tokens.
    #[test]
    fn pest_parser_returns_number_and_operator_tokens() -> anyhow::Result<()> {
        assert_eq!(
            parse_expression("3 + 5 * 2 - 8 / 4")?,
            ["3", "+", "5", "*", "2", "-", "8", "/", "4"]
        );
        Ok(())
    }

    /// Verifies malformed input returns a parser error.
    #[test]
    fn parser_rejects_an_incomplete_expression() {
        assert!(parse_expression("3 +").is_err());
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
