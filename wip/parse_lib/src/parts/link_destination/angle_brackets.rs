//! Parses link destinations between < and >.
//!
//! A link destination can consist of a sequence of zero or more characters between an opening < and a closing >
//! that contains no line endings or unescaped < or > characters.
//! <https://spec.commonmark.org/0.31.2/#link-destination>

use winnow::Result;
use winnow::combinator::alt;
use winnow::combinator::delimited;
use winnow::combinator::fail;
use winnow::combinator::preceded;
use winnow::combinator::repeat;
use winnow::error::ContextError;
use winnow::error::ErrMode;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;
use winnow::prelude::*;
use winnow::token::take_while;

/// Parses a sequence of characters that are not '<', '>', or newlines.
fn parse_non_special_chars<'s>(input: &mut &'s str) -> Result<&'s str> {
    take_while(1.., |c: char| {
        c != '<' && c != '>' && c != '\n' && c != '\r'
    })
    .context(Label("non-special characters"))
    .context(Expected(Description(
        "a sequence of characters that are not '<', '>', or newlines.",
    )))
    .parse_next(input)
}

/// Parses an escaped angle bracket e.g., \< or \>.
fn parse_escaped_chars<'s>(input: &mut &'s str) -> Result<&'s str> {
    preceded('\\', alt(("<", ">")))
        .take()
        .context(Label("escaped angle bracket"))
        .context(Expected(Description(r"\< or \>")))
        .parse_next(input)
}

/// Parses characters allowed inside < and > brackets: non-special or \< or \>.
fn parse_content_chars<'s>(input: &mut &'s str) -> Result<&'s str> {
    alt((
        parse_escaped_chars,
        parse_non_special_chars,
        fail.context(Label("character between < and >"))
            .context(Expected(Description(
                "chars that are not '<', '>', or newlines; or escaped < or >",
            ))),
    ))
    .take()
    .parse_next(input)
}

/// Parse link destination between angle brackets: '<' followed by zero or more `parse_content_char` sequences, followed by '>'.
pub(super) fn parse_angle_brackets<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    delimited("<", repeat::<_, _, String, _, _>(0.., parse_content_chars), ">")
        .take()
        .context(Label("angle brackets"))
        .context(Expected(Description("a link destination between angle brackets: < followed by zero or more content characters, followed by >")))
        .parse_next(input)
        .map_err(|e : ContextError| ErrMode::Backtrack(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_escaped_char() {
        assert_eq!(
            parse_escaped_chars.parse_peek(r"\<text"),
            Ok(("text", r"\<"))
        );
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
