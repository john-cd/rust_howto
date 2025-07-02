//! Parses a link label, which is enclosed in square brackets `[]`.
//!
//! Simplified from spec: <https://spec.commonmark.org/0.31.2/#link-label>
//!
//! - A link label begins with a left bracket ([) and ends with the first right bracket (]) that is not backslash-escaped.
//! - Between these brackets there must be at least one character that is not a space, tab, or line ending.
//! - Unescaped square bracket characters are not allowed inside the opening and closing square brackets of link labels.
//! - A link label can have at most 999 characters inside the square brackets.

use nom::IResult;
use nom::Parser;
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::char;
use nom::combinator::verify;
use nom::sequence::delimited;

/// Parses link label.
///
/// Example: "[my label]" -> "my label".
/// Does not handle escaped brackets.
pub fn parse_link_label(input: &str) -> IResult<&str, &str> {
    let label_content = verify(
        take_while_m_n(1, 999, |c: char| {
            c != '[' && c != ']' && c != '\n' && c != '\r'
        }),
        |s: &str| !s.trim().is_empty(),
    );

    let mut parser = delimited(char('['), label_content, char(']'));

    parser.parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_label() {
        assert_eq!(parse_link_label("[my label]"), Ok(("", "my label")));
    }

    #[test]
    fn test_label_with_trailing_text() {
        assert_eq!(
            parse_link_label("[label] and more"),
            Ok((" and more", "label"))
        );
    }

    #[test]
    fn test_label_with_internal_whitespace() {
        assert_eq!(parse_link_label("[  my label  ]"), Ok(("", "  my label  ")));
    }

    #[test]
    fn test_label_max_length() {
        let max_label = "a".repeat(999);
        let input = format!("[{max_label}]");
        assert_eq!(parse_link_label(&input), Ok(("", max_label.as_str())));
    }

    #[test]
    fn test_fail_on_empty_label() {
        // Fails because take_while_m_n requires at least 1 char.
        assert!(parse_link_label("[]").is_err());
    }

    #[test]
    fn test_fail_on_whitespace_only_label() {
        // Fails because the `verify` condition checks that the trimmed label is not empty.
        assert!(parse_link_label("[ ]").is_err());
        assert!(parse_link_label("[\t\t]").is_err());
    }

    #[test]
    fn test_fail_on_label_too_long() {
        let long_label = "a".repeat(1000);
        let input = format!("[{long_label}]");
        assert!(parse_link_label(&input).is_err());
    }

    #[test]
    fn test_fail_on_unclosed_bracket() {
        assert!(parse_link_label("[unclosed").is_err());
    }

    #[test]
    fn test_fail_on_nested_brackets() {
        // Fails because `[` and `]` are not allowed in the label content.
        assert!(parse_link_label("[a[b]c]").is_err());
    }

    #[test]
    fn test_fail_on_newline() {
        assert!(parse_link_label("[label\nwith newline]").is_err());
    }
}
