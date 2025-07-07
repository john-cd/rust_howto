//! A link destination can be a nonempty sequence of characters that does not start with <,
//! does not include ASCII control characters or space character, and
//! includes parentheses only if (a) they are backslash-escaped or
//! (b) they are part of a balanced pair of unescaped parentheses.
//! (Implementations may impose limits on parentheses nesting to avoid performance issues,
//! but at least three levels of nesting should be supported.)
//!
//! <https://spec.commonmark.org/0.31.2/#link-destination>

use winnow::Result;
use winnow::Parser;
use winnow::branch::alt;
use winnow::token::take_while1;
use winnow::character::complete::anychar;
use winnow::bytes::one_of;
use winnow::combinator::cut;
use winnow::combinator::recognize;
use winnow::combinator::verify;
use winnow::multi::many0;
use winnow::combinator::delimited;
use winnow::sequence::preceded;

/// Parses a sequence of characters that are not control, space, '<', '(', or ')'.
fn parse_allowed_chars<'s>(input: &mut &'s str) -> Result< &'s str> {
    take_while1(|c: char| !(c.is_ascii_control() || c == ' ' || c == '<' || c == '(' || c == ')'))
        .parse_next(input)
}

// /// Parses characters that are not a control character, space, '<', '(', or ')'.
// fn parse_allowed_chars2<'s>(input: &mut &'s str) -> Result< &'s str> {
//     is_not("\u{0000}-\u{001F} <>()").parse_next(input)
// }

/// Parses an escaped parenthesis, e.g., "\(" or "\)".
fn parse_escaped_parenthesis<'s>(input: &mut &'s str) -> Result< &'s str> {
    preceded(one_of('\\'), alt(("(", ")"))).take().parse_next(input)
}

/// Parses content within balanced parentheses. This is a recursive parser.
/// It can contain allowed characters, escaped parentheses, or nested balanced parentheses.
fn parse_balanced_parentheses_content<'s>(input: &mut &'s str) -> Result< &'s str> {
    many0(alt((
        parse_allowed_chars,
        parse_escaped_parenthesis,
        parse_balanced_parentheses, // Recursively parse nested parentheses.
    ))).take()
    .parse_next(input)
}

/// Parses a balanced pair of unescaped parentheses, e.g., "(foo(bar)baz)".
/// The content inside can be anything allowed by `balanced_parentheses_content`.
fn parse_balanced_parentheses<'s>(input: &mut &'s str) -> Result< &'s str> {
    delimited(
        "(",
        parse_balanced_parentheses_content,
        cut(")"), // Use cut to ensure ')' is present if '(' was matched.
    ).take()
    .parse_next(input)
}

/// The main parser for a non-empty sequence of characters not starting by `<`.
pub(super) fn parse_non_empty_sequence<'s>(input: &mut &'s str) -> Result< &'s str> {
    // Ensure the first character is not '<'.
    verify(anychar, |c| *c != '<').parse_next(input)?;

    many0(alt((
        parse_escaped_parenthesis,
        parse_balanced_parentheses,
        parse_allowed_chars,
    ))).take()
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sequences() {
        assert_eq!(parse_non_empty_sequence.parse_peek("hello"), Ok(("", "hello")));
        assert_eq!(
            parse_non_empty_sequence.parse_peek(r"h\(ello\)"),
            Ok(("", r"h\(ello\)"))
        );
        assert_eq!(
            parse_non_empty_sequence.parse_peek(r"just\(a\)char"),
            Ok(("", r"just\(a\)char"))
        );
        assert_eq!(
            parse_non_empty_sequence.parse_peek("foo(bar)baz"),
            Ok(("", "foo(bar)baz"))
        );
        assert_eq!(parse_non_empty_sequence.parse_peek("a(b)c(d)e"), Ok(("", "a(b)c(d)e")));
        assert_eq!(parse_non_empty_sequence.parse_peek("a(b(c)d)e"), Ok(("", "a(b(c)d)e")));
        assert_eq!(parse_non_empty_sequence.parse_peek("(a)"), Ok(("", "(a)")));
        assert_eq!(parse_non_empty_sequence.parse_peek("foo)bar"), Ok((")bar", "foo")));
        assert_eq!(
            parse_non_empty_sequence.parse_peek(r"parens\(escaped\)and(balanced)"),
            Ok(("", r"parens\(escaped\)and(balanced)"))
        );
        assert_eq!(
            parse_non_empty_sequence.parse_peek(r"a(b\(c\)d)e"),
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
