//! A link destination can be a nonempty sequence of characters that does not start with <,
//! does not include ASCII control characters or space character, and
//! includes parentheses only if (a) they are backslash-escaped or
//! (b) they are part of a balanced pair of unescaped parentheses.
//! (Implementations may impose limits on parentheses nesting to avoid performance issues,
//! but at least three levels of nesting should be supported.)
//!
//! <https://spec.commonmark.org/0.31.2/#link-destination>

use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::anychar;
use nom::character::complete::char;
use nom::combinator::cut;
use nom::combinator::recognize;
use nom::combinator::verify;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::preceded;

/// Parses a sequence of characters that are not control, space, '<', '(', or ')'.
fn parse_allowed_chars(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !(c.is_ascii_control() || c == ' ' || c == '<' || c == '(' || c == ')'))
        .parse(input)
}

// /// Parses characters that are not a control character, space, '<', '(', or ')'.
// fn parse_allowed_chars2(input: &str) -> IResult<&str, &str> {
//     is_not("\u{0000}-\u{001F} <>()").parse(input)
// }

/// Parses an escaped parenthesis, e.g., "\(" or "\)".
fn parse_escaped_parenthesis(input: &str) -> IResult<&str, &str> {
    recognize(preceded(char('\\'), alt((char('('), char(')'))))).parse(input)
}

/// Parses content within balanced parentheses. This is a recursive parser.
/// It can contain allowed characters, escaped parentheses, or nested balanced parentheses.
fn parse_balanced_parentheses_content(input: &str) -> IResult<&str, &str> {
    recognize(many0(alt((
        parse_allowed_chars,
        parse_escaped_parenthesis,
        parse_balanced_parentheses, // Recursively parse nested parentheses.
    ))))
    .parse(input)
}

/// Parses a balanced pair of unescaped parentheses, e.g., "(foo(bar)baz)".
/// The content inside can be anything allowed by `balanced_parentheses_content`.
fn parse_balanced_parentheses(input: &str) -> IResult<&str, &str> {
    recognize(delimited(
        char('('),
        parse_balanced_parentheses_content,
        cut(char(')')), // Use cut to ensure ')' is present if '(' was matched.
    ))
    .parse(input)
}

/// The main parser for a non-empty sequence of characters not starting by `<`.
pub(super) fn parse_non_empty_sequence(input: &str) -> IResult<&str, &str> {
    // Ensure the first character is not '<'.
    verify(anychar, |c| *c != '<').parse(input)?;

    recognize(many0(alt((
        parse_escaped_parenthesis,
        parse_balanced_parentheses,
        parse_allowed_chars,
    ))))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sequences() {
        assert_eq!(parse_non_empty_sequence("hello"), Ok(("", "hello")));
        assert_eq!(
            parse_non_empty_sequence(r"h\(ello\)"),
            Ok(("", r"h\(ello\)"))
        );
        assert_eq!(
            parse_non_empty_sequence(r"just\(a\)char"),
            Ok(("", r"just\(a\)char"))
        );
        assert_eq!(
            parse_non_empty_sequence("foo(bar)baz"),
            Ok(("", "foo(bar)baz"))
        );
        assert_eq!(parse_non_empty_sequence("a(b)c(d)e"), Ok(("", "a(b)c(d)e")));
        assert_eq!(parse_non_empty_sequence("a(b(c)d)e"), Ok(("", "a(b(c)d)e")));
        assert_eq!(parse_non_empty_sequence("(a)"), Ok(("", "(a)")));
        assert_eq!(parse_non_empty_sequence("foo)bar"), Ok((")bar", "foo")));
        assert_eq!(
            parse_non_empty_sequence(r"parens\(escaped\)and(balanced)"),
            Ok(("", r"parens\(escaped\)and(balanced)"))
        );
        assert_eq!(
            parse_non_empty_sequence(r"a(b\(c\)d)e"),
            Ok(("", r"a(b\(c\)d)e"))
        ); // Nested with escaped
        // Contains space
        assert_eq!(
            parse_non_empty_sequence("hello world"),
            Ok((" world", "hello",),)
        );
        // Contains control character (null).
        assert_eq!(
            parse_non_empty_sequence("hello\0world"),
            Ok(("\0world", "hello",))
        );
    }

    #[test]
    fn test_invalid_sequences() {
        // Empty string
        assert!(parse_non_empty_sequence("").is_err());
        // Starts with '<'.
        assert!(parse_non_empty_sequence("<hello").is_err());
        // Unbalanced parentheses.
        assert!(parse_non_empty_sequence("foo(bar").is_err());
        assert!(parse_non_empty_sequence("(foo").is_err());
        assert!(parse_non_empty_sequence("(a(b)c").is_err());
    }
}
