//! Parse naked link destinations (e.g. naked URLs).
//!
//! Simplified from <https://spec.commonmark.org/0.31.2/#link-destination>
//!
//! A link destination can be a nonempty sequence of characters that does not start with <,
//! does not include ASCII control characters or space character, and
//! includes parentheses only if (a) they are backslash-escaped or
//! (b) they are part of a balanced pair of unescaped parentheses.
//! (Implementations may impose limits on parentheses nesting to avoid performance issues,
//! but at least three levels of nesting should be supported.)

use winnow::ModalResult;
use winnow::Result;
use winnow::ascii::take_escaped;
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::fail;
use winnow::combinator::repeat;
use winnow::error::ContextError;
use winnow::error::ErrMode;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;
use winnow::prelude::*;
use winnow::token::any;
use winnow::token::one_of;
use winnow::token::take_while;

/// Returns true if a character that is not control, space, < ( or ).
fn is_allowed_char(c: char) -> bool {
    !(c.is_ascii_control() || c == ' ' || c == '<' || c == '(' || c == ')')
}

/// Parses a sequence of allowed chars.
fn parse_allowed_chars<'s>(input: &mut &'s str) -> Result<&'s str> {
    take_while(1.., is_allowed_char)
        .context(Label("character"))
        .context(Expected(Description(
            "a character that is not a control char; a space; or < ( )",
        )))
        .parse_next(input)
}

/// Parses text that may contain escaped parentheses, e.g., "\(" or "\)".
fn parse_text_with_escapes<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    let mut p = take_escaped(parse_allowed_chars, '\\', one_of(['(', ')', '\\']))
        .context(Label("text"))
        .context(Expected(Description(
            r"text that may contain escaped parentheses or backslashes, e.g., \( or \) or \\",
        )));

    p.parse_next(input)
        .map_err(|e: ContextError| ErrMode::Backtrack(e))
}

/// Recursively parses content within balanced parentheses.
/// It can contain allowed characters, escaped parentheses, or nested balanced parentheses.
fn parse_balanced_parentheses_content<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    repeat::<_, _, String, _, _>(
        0..,
        alt((
            parse_balanced_parentheses, // Recursively parse.
            parse_text_with_escapes,
            fail.context(Label("balanced parentheses content"))
                .context(Expected(Description(
                    "content within balanced parentheses.",
                ))),
        )),
    )
    .take()
    .parse_next(input)
}

/// Parses a balanced pair of unescaped parentheses, e.g., "(foo(bar)baz)".
/// The content inside can be anything allowed by `balanced_parentheses_content`.
fn parse_balanced_parentheses<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    delimited(
        "(",
        parse_balanced_parentheses_content,
        cut_err(")"), // Use cut_err to ensure ')' is present if '(' was matched.
    )
    .take()
    .context(Label("balanced pair of unescaped parentheses"))
    .parse_next(input)
}

/// The main parser for a non-empty sequence of characters not starting by `<`.
pub(crate) fn parse_non_empty_sequence<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    // Ensure the first character is not '<'.
    any.verify(|c| *c != '<').parse_next(input)?;

    repeat::<_, _, String, _, _>(
        0..,
        alt((
            parse_balanced_parentheses,
            parse_text_with_escapes,
            fail
              .context(Label("link destination not between < and >"))
              .context(Expected(Description("a non-empty sequence of characters not starting by < without ASCII control characters or space"))),
        )),
    )
    .take()
    .parse_next(input)
}

#[cfg(test)]
mod tests_parse_non_empty_sequence {
    use super::*;

    #[test]
    fn test_valid_sequences() {
        assert_eq!(
            parse_non_empty_sequence.parse_peek("hello"),
            Ok(("", "hello"))
        );
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
        assert_eq!(
            parse_non_empty_sequence.parse_peek("a(b)c(d)e"),
            Ok(("", "a(b)c(d)e"))
        );
        assert_eq!(
            parse_non_empty_sequence.parse_peek("a(b(c)d)e"),
            Ok(("", "a(b(c)d)e"))
        );
        assert_eq!(parse_non_empty_sequence.parse_peek("(a)"), Ok(("", "(a)")));
        assert_eq!(
            parse_non_empty_sequence.parse_peek("foo)bar"),
            Ok((")bar", "foo"))
        );
        assert_eq!(
            parse_non_empty_sequence.parse_peek(r"parens\(escaped\)and(balanced)"),
            Ok(("", r"parens\(escaped\)and(balanced)"))
        );
        assert_eq!(
            parse_non_empty_sequence.parse_peek(r"a(b\(c\)d)e"),
            Ok(("", r"a(b\(c\)d)e"))
        ); // Nested with escaped.
        // Contains space.
        assert_eq!(
            parse_non_empty_sequence.parse_peek("hello world"),
            Ok((" world", "hello",),)
        );
        // Contains control character (null).
        assert_eq!(
            parse_non_empty_sequence.parse_peek("hello\0world"),
            Ok(("\0world", "hello",))
        );
    }

    #[test]
    fn test_invalid_sequences() {
        // Empty string.
        assert!(parse_non_empty_sequence(&mut "").is_err());
        // Starts with '<'.
        assert!(parse_non_empty_sequence(&mut "<hello").is_err());
        // Unbalanced parentheses.
        assert!(parse_non_empty_sequence(&mut "foo(bar").is_err());
        assert!(parse_non_empty_sequence(&mut "(foo").is_err());
        assert!(parse_non_empty_sequence(&mut "(a(b)c").is_err());
    }
}

#[cfg(test)]
mod text_with_escapes_tests {
    use super::*;

    #[test]
    fn test_text_with_escapes_basic() {
        // Simple allowed chars.
        assert_eq!(
            parse_text_with_escapes.parse_peek("hello"),
            Ok(("", "hello"))
        );
        // Escaped parenthesis.
        assert_eq!(parse_text_with_escapes.parse_peek(r"\("), Ok(("", r"\(")));
        assert_eq!(parse_text_with_escapes.parse_peek(r"\)"), Ok(("", r"\)")));
        // Escaped backslash.
        assert_eq!(
            parse_text_with_escapes.parse_peek(r"foo\\bar"),
            Ok(("", r"foo\\bar"))
        );
        // Mixed escapes.
        assert_eq!(
            parse_text_with_escapes.parse_peek(r"foo\)bar\("),
            Ok(("", r"foo\)bar\("))
        );
        // Escaped parenthesis in the middle.
        assert_eq!(
            parse_text_with_escapes.parse_peek(r"abc\)def"),
            Ok(("", r"abc\)def"))
        );
        // Escaped parenthesis at the start.
        assert_eq!(
            parse_text_with_escapes.parse_peek(r"\)abc"),
            Ok(("", r"\)abc"))
        );
        // Escaped parenthesis at the end.
        assert_eq!(
            parse_text_with_escapes.parse_peek(r"abc\("),
            Ok(("", r"abc\("))
        );
    }

    #[test]
    fn test_text_with_escapes_partial() {
        // Stops at unescaped parenthesis.
        assert_eq!(
            parse_text_with_escapes.parse_peek("foo(bar"),
            Ok(("(", "foo"))
        );
        // Stops at unescaped parenthesis after escapes.
        assert_eq!(
            parse_text_with_escapes.parse_peek(r"foo\)bar(baz"),
            Ok(("(", r"foo\)bar"))
        );
        // Stops at unescaped parenthesis after escaped.
        assert_eq!(
            parse_text_with_escapes.parse_peek(r"\(foo)bar"),
            Ok((")", r"\(foo"))
        );
    }

    #[test]
    fn test_text_with_escapes_invalid() {
        // Starts with forbidden char.
        assert!(parse_text_with_escapes(&mut "(").is_err());
        assert!(parse_text_with_escapes(&mut ")").is_err());
        assert!(parse_text_with_escapes(&mut "<").is_err());
        assert!(parse_text_with_escapes(&mut " ").is_err());
        assert!(parse_text_with_escapes(&mut "\0").is_err());
        // Lone backslash at end.
        assert!(parse_text_with_escapes(&mut r"foo\").is_err());
    }
}
