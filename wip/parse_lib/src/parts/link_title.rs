//! Parses link title.
//!
//! Simplified from <https://spec.commonmark.org/0.31.2/#link-title>.

use winnow::Parser;
use winnow::Result;
use winnow::combinator::alt;
use winnow::combinator::delimited;
use winnow::combinator::fail;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;
use winnow::token::take_while;

/// Parses an optional link title.
/// The title can be enclosed in double quotes `""`, single quotes `''`, or parentheses `()`.
///
/// Example: "\"My Title\"" -> "My Title"
/// Example: "'Another Title'" -> "Another Title"
/// Example: "(Yet Another Title)" -> "Yet Another Title"
pub fn parse_link_title<'s>(input: &mut &'s str) -> Result<&'s str> {
    alt((
        // Double quotes.
        delimited(
            '"',
            take_while(0.., |c: char| c != '"' && c != '\n' && c != '\r'),
            '"',
        ),
        // Single quotes.
        delimited(
            '\'',
            take_while(0.., |c: char| c != '\'' && c != '\n' && c != '\r'),
            '\'',
        ),
        // Parentheses.
        delimited(
            "(",
            take_while(0.., |c: char| c != ')' && c != '\n' && c != '\r'),
            ")",
        ),
        fail.context(Label("link title"))
            .context(Expected(Description(
                r#"enclosed in double quotes `""`, single quotes `''`, or parentheses"#,
            ))),
    ))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_quoted_title() {
        assert_eq!(
            parse_link_title.parse_peek(r#""My Title""#),
            Ok(("", "My Title"))
        );
    }

    #[test]
    fn test_single_quoted_title() {
        assert_eq!(
            parse_link_title.parse_peek(r#"'Another Title'"#),
            Ok(("", "Another Title"))
        );
    }

    #[test]
    fn test_parenthesized_title() {
        assert_eq!(
            parse_link_title.parse_peek(r#"(Yet Another Title)"#),
            Ok(("", "Yet Another Title"))
        );
    }

    #[test]
    fn test_title_with_trailing_text() {
        assert_eq!(
            parse_link_title.parse_peek(r#""title" and more"#),
            Ok((" and more", "title"))
        );
    }

    #[test]
    fn test_empty_title() {
        assert_eq!(parse_link_title.parse_peek(r#""""#), Ok(("", "")));
        assert_eq!(parse_link_title.parse_peek(r#"''"#), Ok(("", "")));
        assert_eq!(parse_link_title.parse_peek(r#"()"#), Ok(("", "")));
    }

    #[test]
    fn test_title_with_other_quotes_inside() {
        assert_eq!(
            parse_link_title.parse_peek(r#""title with 'single' quotes""#),
            Ok(("", "title with 'single' quotes"))
        );
        assert_eq!(
            parse_link_title.parse_peek(r#"'title with "double" quotes'"#),
            Ok(("", "title with \"double\" quotes"))
        );
    }

    #[test]
    fn test_fail_on_unclosed_quotes() {
        assert!(parse_link_title(&mut r#""unclosed"#).is_err());
        assert!(parse_link_title(&mut r#"'unclosed"#).is_err());
        assert!(parse_link_title(&mut r#"(unclosed"#).is_err());
    }

    #[test]
    fn test_fail_on_newline() {
        assert!(parse_link_title(&mut "\"title\nwith newline\"").is_err());
        assert!(parse_link_title(&mut "'title\nwith newline'").is_err());
        assert!(parse_link_title(&mut "(title\nwith newline)").is_err());
    }

    #[test]
    fn test_no_match_on_plain_text() {
        assert!(parse_link_title(&mut "just text").is_err());
    }
}
