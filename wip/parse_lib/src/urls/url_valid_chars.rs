// TODO validate the URL? Url::parse("https://example.net")?;

use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::tag;
use nom::bytes::take_while1;
use nom::combinator::recognize;

/// Predicate function to check if a character is one of the valid URL characters.
/// Valid chars in URLs are A-Z, a-z, 0-9, -, ., _, ~, :, /, ?, #, [, ], @, !, $, &, ', (, ), *, +, ,, ;, %, =
/// ":" | "/" | "@" | "?" | "#" have special meaning in URLs.
fn is_valid_url_char(c: char) -> bool {
    matches!(c,
        'A'..='Z'
        | 'a'..='z'
        | '0'..='9'
        | '-' | '.' | '_' | '~'
        | ':' | '/' | '?' | '#' | '[' | ']' | '@' | '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '%' | '='
    )
}

/// Parses one or more valid URL characters.
fn parse_url_chars(input: &str) -> IResult<&str, &str> {
    take_while1(is_valid_url_char).parse(input)
}

/// Recognize an absolute HTTP / HTTPS URL.
pub fn recognize_naked_url(input: &str) -> IResult<&str, &str> {
    // If the child parser was successful, return the consumed input as produced value.
    recognize((
        alt((tag("http://"), tag("https://"))), // Protocols.
        parse_url_chars,
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url_chars_basic() {
        // Stops at `|`, which is not a reserved char. Otherwise we would get an `Incomplete`.
        assert_eq!(parse_url_chars("abcdefg12345|"), Ok(("|", "abcdefg12345")));
        // Stops at space.
        assert_eq!(parse_url_chars("hello world"), Ok((" world", "hello")));
        assert!(parse_url_chars("").is_err());
    }

    #[test]
    fn test_parse_url_chars_reserved() {
        assert_eq!(
            // Stops at `|`.
            parse_url_chars(":/?#[]@!$&'()*+,,;%+=|"),
            Ok(("|", ":/?#[]@!$&'()*+,,;%+="))
        );
        // Mixed.
        assert_eq!(
            parse_url_chars("foo-bar.baz_qux~ "),
            Ok((" ", "foo-bar.baz_qux~"))
        );
    }

    #[test]
    fn test_parse_url_chars_url_like() {
        assert_eq!(
            parse_url_chars("some/path?query#fragment "),
            Ok((" ", "some/path?query#fragment"))
        );
        assert_eq!(
            parse_url_chars("user:pass@host.com "),
            Ok((" ", "user:pass@host.com"))
        );
        assert_eq!(
            parse_url_chars("item;param=value "),
            Ok((" ", "item;param=value"))
        );
        assert_eq!(
            parse_url_chars("http://example.com/path/to/resource?id=123&name=test#section "),
            Ok((
                " ",
                "http://example.com/path/to/resource?id=123&name=test#section"
            ))
        );
        assert_eq!(
            parse_url_chars("weird_file!.txt "),
            Ok((" ", "weird_file!.txt"))
        );
        assert_eq!(parse_url_chars("foo%20bar "), Ok((" ", "foo%20bar"))); // Includes '%'.
    }

    #[test]
    fn test_parse_url_chars_invalid() {
        assert_eq!(parse_url_chars("invalid<char"), Ok(("<char", "invalid")));
        assert_eq!(parse_url_chars("newline\nchar"), Ok(("\nchar", "newline")));
        assert_eq!(parse_url_chars("back\\slash"), Ok(("\\slash", "back")));
    }
}
