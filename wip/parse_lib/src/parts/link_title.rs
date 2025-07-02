//! Parses link title.
//!
//! Simplified from <https://spec.commonmark.org/0.31.2/#link-title>.

use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::sequence::delimited;

/// Parses an optional link title.
/// The title can be enclosed in double quotes `""`, single quotes `''`, or parentheses `()`.
///
/// Example: "\"My Title\"" -> "My Title"
/// Example: "'Another Title'" -> "Another Title"
/// Example: "(Yet Another Title)" -> "Yet Another Title"
pub fn parse_link_title(input: &str) -> IResult<&str, &str> {
    alt((
        // Double quotes.
        delimited(
            char('"'),
            take_while(|c: char| c != '"' && c != '\n' && c != '\r'),
            char('"'),
        ),
        // Single quotes
        delimited(
            char('\''),
            take_while(|c: char| c != '\'' && c != '\n' && c != '\r'),
            char('\''),
        ),
        // Parentheses
        delimited(
            char('('),
            take_while(|c: char| c != ')' && c != '\n' && c != '\r'),
            char(')'),
        ),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_quoted_title() {
        assert_eq!(parse_link_title(r#""My Title""#), Ok(("", "My Title")));
    }

    #[test]
    fn test_single_quoted_title() {
        assert_eq!(
            parse_link_title(r#"'Another Title'"#),
            Ok(("", "Another Title"))
        );
    }

    #[test]
    fn test_parenthesized_title() {
        assert_eq!(
            parse_link_title(r#"(Yet Another Title)"#),
            Ok(("", "Yet Another Title"))
        );
    }

    #[test]
    fn test_title_with_trailing_text() {
        assert_eq!(
            parse_link_title(r#""title" and more"#),
            Ok((" and more", "title"))
        );
    }

    #[test]
    fn test_empty_title() {
        assert_eq!(parse_link_title(r#""""#), Ok(("", "")));
        assert_eq!(parse_link_title(r#"''"#), Ok(("", "")));
        assert_eq!(parse_link_title(r#"()"#), Ok(("", "")));
    }

    #[test]
    fn test_title_with_other_quotes_inside() {
        assert_eq!(
            parse_link_title(r#""title with 'single' quotes""#),
            Ok(("", "title with 'single' quotes"))
        );
        assert_eq!(
            parse_link_title(r#"'title with "double" quotes'"#),
            Ok(("", "title with \"double\" quotes"))
        );
    }

    #[test]
    fn test_fail_on_unclosed_quotes() {
        assert!(parse_link_title(r#""unclosed"#).is_err());
        assert!(parse_link_title(r#"'unclosed"#).is_err());
        assert!(parse_link_title(r#"(unclosed"#).is_err());
    }

    #[test]
    fn test_fail_on_newline() {
        assert!(parse_link_title("\"title\nwith newline\"").is_err());
        assert!(parse_link_title("'title\nwith newline'").is_err());
        assert!(parse_link_title("(title\nwith newline)").is_err());
    }

    #[test]
    fn test_no_match_on_plain_text() {
        assert!(parse_link_title("just text").is_err());
    }
}
