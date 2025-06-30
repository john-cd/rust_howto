//! Parse link destinations.
//! <https://spec.commonmark.org/0.31.2/#link-destination>

use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::complete::none_of;
use nom::combinator::cut;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::preceded;

/// Parses a single allowed character that is not a control character, space, '<', '(', or ')'.
fn allowed_char_base(input: &str) -> IResult<&str, char> {
    none_of("\u{0000}-\u{001F} <>()")(input)
}

/// Parses an escaped parenthesis, e.g., "\(" or "\)".
fn escaped_parenthesis(input: &str) -> IResult<&str, char> {
    preceded(char('\\'), alt((char('('), char(')')))).parse(input)
}

/// Parses content within balanced parentheses. This is a recursive parser.
/// It can contain allowed characters, escaped parentheses, or nested balanced parentheses.
fn balanced_parentheses_content(input: &str) -> IResult<&str, String> {
    let (input, chars) = many0(alt((
        map(allowed_char_base, |c| c.to_string()),
        map(escaped_parenthesis, |c| format!("\\{c}")), // Keep the escape char for content.
        map(balanced_parentheses, |s| s),                 // Recursively parse nested parentheses.
    )))
    .parse(input)?;
    Ok((input, chars.join("")))
}

/// Parses a balanced pair of unescaped parentheses, e.g., "(foo(bar)baz)".
/// The content inside can be anything allowed by `balanced_parentheses_content`.
fn balanced_parentheses(input: &str) -> IResult<&str, String> {
    map(
        delimited(
            char('('),
            balanced_parentheses_content,
            cut(char(')')), // Use cut to ensure ')' is present if '(' was matched.
        ),
        |s| format!("({s})"), // Include the parentheses in the output string.
    )
    .parse(input)
}

/// Parses a sequence of characters that are not control, space, '<', '(', or ')'.
fn basic_segment(input: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| {
            !(c.is_ascii_control() || c == ' ' || c == '<' || c == '(' || c == ')')
        }),
        |s: &str| s.to_string(),
    )
    .parse(input)
}

/// The main parser for the non-empty sequence of characters.
/// It ensures the sequence does not start with '<' and handles all other rules.
pub fn non_empty_sequence_parser(input: &str) -> IResult<&str, String> {
    // Ensure the first character is not '<'
    let (remaining_input, first_char) = none_of("<")(input)?;
    let mut parsed_chars = String::from(first_char);

    // Parse the rest of the string
    let (final_input, rest_segments) = many0(alt((
        map(escaped_parenthesis, |c| format!("\\{c}")),
        map(balanced_parentheses, |s| s),
        basic_segment,
    )))
    .parse(remaining_input)?;

    for segment in rest_segments {
        parsed_chars.push_str(&segment);
    }

    Ok((final_input, parsed_chars))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sequences() {
        assert_eq!(
            non_empty_sequence_parser("hello"),
            Ok(("", "hello".to_string()))
        );
        assert_eq!(
            non_empty_sequence_parser("h\\(ello\\)"),
            Ok(("", "h\\(ello\\)".to_string()))
        );
        assert_eq!(
            non_empty_sequence_parser("foo(bar)baz"),
            Ok(("", "foo(bar)baz".to_string()))
        );
        assert_eq!(
            non_empty_sequence_parser("a(b(c)d)e"),
            Ok(("", "a(b(c)d)e".to_string()))
        );
        assert_eq!(
            non_empty_sequence_parser("parens\\(escaped\\)and(balanced)"),
            Ok(("", "parens\\(escaped\\)and(balanced)".to_string()))
        );
        assert_eq!(
            non_empty_sequence_parser("just\\(a\\)char"),
            Ok(("", "just\\(a\\)char".to_string()))
        );
        assert_eq!(
            non_empty_sequence_parser("no_parens"),
            Ok(("", "no_parens".to_string()))
        );
        assert_eq!(
            non_empty_sequence_parser("one"),
            Ok(("", "one".to_string()))
        );
        assert_eq!(
            non_empty_sequence_parser("a(b\\(c\\)d)e"),
            Ok(("", "a(b\\(c\\)d)e".to_string()))
        ); // Nested with escaped
        assert_eq!(
            non_empty_sequence_parser("(a)"),
            Ok(("", "(a)".to_string()))
        );
        assert_eq!(
            non_empty_sequence_parser("a(b)c(d)e"),
            Ok(("", "a(b)c(d)e".to_string()))
        );
    }

    #[test]
    fn test_invalid_sequences() {
        // Starts with '<'
        assert!(non_empty_sequence_parser("<hello").is_err());
        // Contains space
        assert!(non_empty_sequence_parser("hello world").is_err());
        // Contains control character (null)
        assert!(non_empty_sequence_parser("hello\0world").is_err());
        // Unbalanced parentheses
        assert!(non_empty_sequence_parser("foo(bar").is_err());
        assert!(non_empty_sequence_parser("foo)bar").is_err());
        assert!(non_empty_sequence_parser("(foo").is_err());
        assert!(non_empty_sequence_parser("foo)").is_err());
        assert!(non_empty_sequence_parser("()").is_err()); // empty content in balanced parentheses is handled by `take_while1` in basic_segment
        assert!(non_empty_sequence_parser("(a(b)c").is_err());
        // Empty string
        assert!(non_empty_sequence_parser("").is_err());
    }
}
