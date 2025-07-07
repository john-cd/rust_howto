//! Parses link destinations between < and >.
//!
//! A link destination can consist of a sequence of zero or more characters between an opening < and a closing >
//! that contains no line endings or unescaped < or > characters.
//! <https://spec.commonmark.org/0.31.2/#link-destination>

use winnow::Result;
use winnow::Parser;
use winnow::branch::alt;
use winnow::token::take_while1;
use winnow::bytes::one_of;
use winnow::combinator::recognize;
use winnow::multi::many0;
use winnow::combinator::delimited;
use winnow::sequence::preceded;

/// Parses a sequence of characters that are not '<', '>', or newlines.
fn parse_non_special_chars<'s>(input: &mut &'s str) -> Result< &'s str> {
    take_while1(|c: char| c != '<' && c != '>' && c != '\n')(input)
}

/// Parses an escaped angle bracket (e.g., '\<', '\>').
fn parse_escaped_char<'s>(input: &mut &'s str) -> Result< &'s str> {
    preceded(one_of('\\'), alt(("<", ">"))).take().parse_next(input)
}

// Parses any character allowed inside the brackets: non-special, or escaped '<' or '>'.
fn parse_content_char<'s>(input: &mut &'s str) -> Result< &'s str> {
    alt((parse_escaped_char, parse_non_special_chars)).take().parse_next(input)
}

// Parse link destination between angle brackets: '<' followed by zero or more `parse_content_char` sequences, followed by '>'.
pub(super) fn parse_angle_brackets<'s>(input: &mut &'s str) -> Result< &'s str> {
    delimited("<", many0(parse_content_char).take(), ">").parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_escaped_char() {
        assert_eq!(parse_escaped_char.parse_peek(r"\<text"), Ok(("text", r"\<")));
    }

    #[test]
    fn test_basic_parse() {
        assert_eq!(
            parse_angle_brackets.parse_peek("<hello world>"),
            Ok(("", "hello world"))
        );
    }

    #[test]
    fn test_with_escaped_chars() {
        assert_eq!(
            parse_angle_brackets.parse_peek(r"<with\<escaped chars>"),
            Ok(("", r"with\<escaped chars"))
        );
    }

    #[test]
    fn test_empty_content() {
        assert_eq!(parse_angle_brackets.parse_peek("<>"), Ok(("", "")));
    }

    #[test]
    fn test_contains_newline_error() {
        // This should fail because of the newline.
        assert!(parse_angle_brackets(&mut "<no\nnewlines>").is_err());
    }

    #[test]
    fn test_starts_with_invalid_char_error() {
        // This should fail because it doesn't start with '<'.
        assert!(parse_angle_brackets(&mut "just text").is_err());
    }
}
