// TODO validate the URL? Url::parse("https://example.net")?;

use winnow::Parser;
use winnow::Result;
use winnow::combinator::alt;
use winnow::combinator::seq;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;
use winnow::token::take_while;

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

// TODO use one_of(('0'..='9', 'a'..='z', 'A'..='Z'))

/// Parses one or more valid URL characters.
fn parse_url_chars<'s>(input: &mut &'s str) -> Result<&'s str> {
    take_while(1.., is_valid_url_char).parse_next(input)
}

/// Recognize an absolute HTTP / HTTPS URL.
pub fn recognize_naked_url<'s>(input: &mut &'s str) -> Result<&'s str> {
    // If the child parser was successful, return the consumed input as produced value.
    seq!(
        alt(("http://", "https://")), // Protocols.
        parse_url_chars,
    )
    .take()
    .context(Label("TODO"))
    .context(Expected(Description("")))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url_chars_basic() {
        // Stops at `|`, which is not a reserved char. Otherwise we would get an `Incomplete`.
        assert_eq!(
            parse_url_chars.parse_peek("abcdefg12345|"),
            Ok(("|", "abcdefg12345"))
        );
        // Stops at space.
        assert_eq!(
            parse_url_chars.parse_peek("hello world"),
            Ok((" world", "hello"))
        );
        assert!(parse_url_chars.parse_peek("").is_err());
    }

    #[test]
    fn test_parse_url_chars_reserved() {
        assert_eq!(
            // Stops at `|`.
            parse_url_chars.parse_peek(":/?#[]@!$&'()*+,,;%+=|"),
            Ok(("|", ":/?#[]@!$&'()*+,,;%+="))
        );
        // Mixed.
        assert_eq!(
            parse_url_chars.parse_peek("foo-bar.baz_qux~ "),
            Ok((" ", "foo-bar.baz_qux~"))
        );
    }

    #[test]
    fn test_parse_url_chars_url_like() {
        assert_eq!(
            parse_url_chars.parse_peek("some/path?query#fragment "),
            Ok((" ", "some/path?query#fragment"))
        );
        assert_eq!(
            parse_url_chars.parse_peek("user:pass@host.com "),
            Ok((" ", "user:pass@host.com"))
        );
        assert_eq!(
            parse_url_chars.parse_peek("item;param=value "),
            Ok((" ", "item;param=value"))
        );
        assert_eq!(
            parse_url_chars
                .parse_peek("http://example.com/path/to/resource?id=123&name=test#section "),
            Ok((
                " ",
                "http://example.com/path/to/resource?id=123&name=test#section"
            ))
        );
        assert_eq!(
            parse_url_chars.parse_peek("weird_file!.txt "),
            Ok((" ", "weird_file!.txt"))
        );
        assert_eq!(parse_url_chars(&mut "foo%20bar "), Ok("foo%20bar")); // Includes '%'.
    }

    #[test]
    fn test_parse_url_chars_invalid() {
        assert_eq!(
            parse_url_chars.parse_peek("invalid<char"),
            Ok(("<char", "invalid"))
        );
        assert_eq!(
            parse_url_chars.parse_peek("newline\nchar"),
            Ok(("\nchar", "newline"))
        );
        assert_eq!(
            parse_url_chars.parse_peek("back\\slash"),
            Ok(("\\slash", "back"))
        );
    }
}
