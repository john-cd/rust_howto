//! Parses link destinations between < and >.
//!
//! A link destination can consist of a sequence of zero or more characters between an opening < and a closing >
//! that contains no line endings or unescaped < or > characters.
//! <https://spec.commonmark.org/0.31.2/#link-destination>

use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::preceded;

/// Parses a sequence of characters that are not '<', '>', or newlines.
fn parse_non_special_chars(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c != '<' && c != '>' && c != '\n')(input)
}

/// Parses an escaped angle bracket (e.g., '\<', '\>').
fn parse_escaped_char(input: &str) -> IResult<&str, &str> {
    recognize(preceded(char('\\'), alt((char('<'), char('>'))))).parse(input)
}

// Parses any character allowed inside the brackets: non-special, or escaped '<' or '>'.
fn parse_content_char(input: &str) -> IResult<&str, &str> {
    recognize(alt((parse_escaped_char, parse_non_special_chars))).parse(input)
}

// Parse link destination between angle brackets: '<' followed by zero or more `parse_content_char` sequences, followed by '>'.
pub(super) fn parse_angle_brackets(input: &str) -> IResult<&str, &str> {
    delimited(char('<'), recognize(many0(parse_content_char)), char('>')).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_escaped_char() {
        assert_eq!(parse_escaped_char(r"\<text"), Ok(("text", r"\<")));
    }

    #[test]
    fn test_basic_parse() {
        assert_eq!(
            parse_angle_brackets("<hello world>"),
            Ok(("", "hello world"))
        );
    }

    #[test]
    fn test_with_escaped_chars() {
        assert_eq!(
            parse_angle_brackets(r"<with\<escaped chars>"),
            Ok(("", r"with\<escaped chars"))
        );
    }

    #[test]
    fn test_empty_content() {
        assert_eq!(parse_angle_brackets("<>"), Ok(("", "")));
    }

    #[test]
    fn test_contains_newline_error() {
        // This should fail because of the newline.
        assert!(parse_angle_brackets("<no\nnewlines>").is_err());
    }

    #[test]
    fn test_starts_with_invalid_char_error() {
        // This should fail because it doesn't start with '<'.
        assert!(parse_angle_brackets("just text").is_err());
    }
}
