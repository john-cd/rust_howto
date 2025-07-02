use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::map;
use nom::sequence::delimited;

use super::super::ast::Element;

/// Parses text enclosed in backticks (`).
/// Simplified from <https://spec.commonmark.org/0.31.2/#code-spans>
pub fn parse_code_span<'a>(input: &'a str) -> IResult<&'a str, Element<'a>> {
    map(
        delimited(
            tag("`"),        // Opening backtick.
            take_until("`"), // Content.
            tag("`"),        // Closing backtick.
        ),
        Element::CodeSpan,
    )
    .parse(input)
}

/// Parses text enclosed in triple backticks (```).
/// Simplified from <https://spec.commonmark.org/0.31.2/#fenced-code-blocks>
pub fn parse_fenced_code_block<'a>(input: &'a str) -> IResult<&'a str, Element<'a>> {
    map(
        delimited(
            tag("```"),        // Opening triple backticks.
            take_until("```"), // Content.
            tag("```"),        // Closing triple backticks.
        ),
        Element::FencedCodeBlock,
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests for parse_code_span ---

    #[test]
    fn test_parse_code_span_basic() {
        assert_eq!(
            parse_code_span("`hello world`"),
            Ok(("", Element::CodeSpan("hello world")))
        );
    }

    #[test]
    fn test_parse_code_span_empty_content() {
        assert_eq!(parse_code_span("``"), Ok(("", Element::CodeSpan(""))));
    }

    #[test]
    fn test_parse_code_span_with_trailing_text() {
        assert_eq!(
            parse_code_span("`code here` more text"),
            Ok((" more text", Element::CodeSpan("code here")))
        );
    }

    #[test]
    fn test_parse_code_span_no_closing_backtick() {
        assert!(parse_code_span("`unclosed span").is_err());
    }

    #[test]
    fn test_parse_code_span_no_opening_backtick() {
        assert!(parse_code_span("no opening backtick`").is_err());
    }

    #[test]
    fn test_parse_code_span_only_backticks() {
        assert!(parse_code_span("`").is_err());
    }

    #[test]
    fn test_parse_code_span_not_a_code_span() {
        assert!(parse_code_span("just plain text").is_err());
    }

    // --- Tests for parse_fenced_code_block ---

    #[test]
    fn test_parse_fenced_code_block_basic() {
        let input = "```fn main() {\n    println!(\"Hello\");\n}```";
        let expected = Element::FencedCodeBlock("fn main() {\n    println!(\"Hello\");\n}");
        assert_eq!(parse_fenced_code_block(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_fenced_code_block_empty_content() {
        assert_eq!(
            parse_fenced_code_block("``````"),
            Ok(("", Element::FencedCodeBlock("")))
        );
    }

    #[test]
    fn test_parse_fenced_code_block_with_trailing_text() {
        let input = "```rust,editable\nprintln!(\"hi\");\n``` more text";
        let expected = Element::FencedCodeBlock("rust,editable\nprintln!(\"hi\");\n");
        assert_eq!(parse_fenced_code_block(input), Ok((" more text", expected)));
    }

    #[test]
    fn test_parse_fenced_code_block_no_closing_delimiter() {
        assert!(parse_fenced_code_block("```unclosed block").is_err());
    }

    #[test]
    fn test_parse_fenced_code_block_no_opening_delimiter() {
        assert!(parse_fenced_code_block("unopened block```").is_err());
    }

    #[test]
    fn test_parse_fenced_code_block_inner_triple_backticks() {
        // `take_until` stops at the *first* closing delimiter.
        assert_eq!(
            parse_fenced_code_block("```foo ``` bar```"),
            Ok((" bar```", Element::FencedCodeBlock("foo ")))
        );
    }

    #[test]
    fn test_parse_fenced_code_block_only_delimiters() {
        assert!(parse_fenced_code_block("```").is_err());
    }

    #[test]
    fn test_parse_fenced_code_block_not_a_code_block() {
        assert!(parse_fenced_code_block("plain text").is_err());
    }

    #[test]
    fn test_parse_fenced_code_block_contains_single_backticks() {
        let input = "```code with `single` backticks```";
        let expected = Element::FencedCodeBlock("code with `single` backticks");
        assert_eq!(parse_fenced_code_block(input), Ok(("", expected)));
    }
}
