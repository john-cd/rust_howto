use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::delimited;
use winnow::token::take_until;

use crate::CodeSpanData;
use crate::FencedCodeBlockData;
use crate::ast::*;

/// Parses text enclosed in backticks (`).
/// Simplified from <https://spec.commonmark.org/0.31.2/#code-spans>
pub fn parse_code_span<'a>(input: &mut &'a str) -> ModalResult<Element<'a>> {
    delimited(
        '`',                  // Opening backtick.
        take_until(0.., '`'), // Content.
        '`',                  // Closing backtick.
    )
    .map(|content| Element::CodeSpan(CodeSpanData { content }))
    .parse_next(input)
}

/// Parses text enclosed in triple backticks (```).
/// Simplified from <https://spec.commonmark.org/0.31.2/#fenced-code-blocks>
pub fn parse_fenced_code_block<'a>(input: &mut &'a str) -> ModalResult<Element<'a>> {
    delimited(
        "```",                  // Opening triple backticks.
        take_until(0.., "```"), // Content.
        "```",                  // Closing triple backticks.
    )
    .map(|content| Element::FencedCodeBlock(FencedCodeBlockData { content }))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests for parse_code_span ---

    #[test]
    fn test_parse_code_span_basic() {
        assert_eq!(
            parse_code_span.parse_peek("`hello world`"),
            Ok((
                "",
                Element::CodeSpan(CodeSpanData {
                    content: "hello world"
                })
            ))
        );
    }

    #[test]
    fn test_parse_code_span_empty_content() {
        assert_eq!(
            parse_code_span.parse_peek("``"),
            Ok(("", Element::CodeSpan(CodeSpanData { content: "" })))
        );
    }

    #[test]
    fn test_parse_code_span_with_trailing_text() {
        assert_eq!(
            parse_code_span.parse_peek("`code here` more text"),
            Ok((
                " more text",
                Element::CodeSpan(CodeSpanData {
                    content: "code here"
                })
            ))
        );
    }

    #[test]
    fn test_parse_code_span_no_closing_backtick() {
        assert!(parse_code_span(&mut "`unclosed span").is_err());
    }

    #[test]
    fn test_parse_code_span_no_opening_backtick() {
        assert!(parse_code_span(&mut "no opening backtick`").is_err());
    }

    #[test]
    fn test_parse_code_span_only_backticks() {
        assert!(parse_code_span(&mut "`").is_err());
    }

    #[test]
    fn test_parse_code_span_not_a_code_span() {
        assert!(parse_code_span(&mut "just plain text").is_err());
    }

    // --- Tests for parse_fenced_code_block ---

    #[test]
    fn test_parse_fenced_code_block_basic() {
        let input = "```fn main() {\n    println!(\"Hello\");\n}```";
        let expected = Element::FencedCodeBlock(FencedCodeBlockData {
            content: "fn main() {\n    println!(\"Hello\");\n}",
        });
        assert_eq!(
            parse_fenced_code_block.parse_peek(input),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_parse_fenced_code_block_empty_content() {
        assert_eq!(
            parse_fenced_code_block.parse_peek("``````"),
            Ok((
                "",
                Element::FencedCodeBlock(FencedCodeBlockData { content: "" })
            ))
        );
    }

    #[test]
    fn test_parse_fenced_code_block_with_trailing_text() {
        let input = "```rust,editable\nprintln!(\"hi\");\n``` more text";
        let expected = Element::FencedCodeBlock(FencedCodeBlockData {
            content: "rust,editable\nprintln!(\"hi\");\n",
        });
        assert_eq!(
            parse_fenced_code_block.parse_peek(input),
            Ok((" more text", expected))
        );
    }

    #[test]
    fn test_parse_fenced_code_block_no_closing_delimiter() {
        assert!(parse_fenced_code_block(&mut "```unclosed block").is_err());
    }

    #[test]
    fn test_parse_fenced_code_block_no_opening_delimiter() {
        assert!(parse_fenced_code_block(&mut "unopened block```").is_err());
    }

    #[test]
    fn test_parse_fenced_code_block_inner_triple_backticks() {
        // `take_until` stops at the *first* closing delimiter.
        assert_eq!(
            parse_fenced_code_block.parse_peek("```foo ``` bar```"),
            Ok((
                " bar```",
                Element::FencedCodeBlock(FencedCodeBlockData { content: "foo " })
            ))
        );
    }

    #[test]
    fn test_parse_fenced_code_block_only_delimiters() {
        assert!(parse_fenced_code_block(&mut "```").is_err());
    }

    #[test]
    fn test_parse_fenced_code_block_not_a_code_block() {
        assert!(parse_fenced_code_block(&mut "plain text").is_err());
    }

    #[test]
    fn test_parse_fenced_code_block_contains_single_backticks() {
        let input = "```code with `single` backticks```";
        let expected = Element::FencedCodeBlock(FencedCodeBlockData {
            content: "code with `single` backticks",
        });
        assert_eq!(
            parse_fenced_code_block.parse_peek(input),
            Ok(("", expected))
        );
    }
}
