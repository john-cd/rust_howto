use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::cut;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::sequence::delimited;

use super::super::ast::Element;

/// Parses a naked URL or autolink (`<url>`).
/// <https://www.w3schools.com/tags//ref_urlencode.asp>
pub fn parse_autolink<'a>(input: &'a str) -> IResult<&'a str, Element<'a>> {
    map(
        alt((
            delimited(tag("<"), recognize_naked_url, cut(tag(">"))),
            recognize_naked_url,
        )),
        Element::Autolink,
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests for recognize_naked_url ---
    #[test]
    fn test_recognize_naked_url_http_simple() {
        assert_eq!(
            recognize_naked_url("http://example.com"),
            Ok(("", "http://example.com"))
        );
    }

    #[test]
    fn test_recognize_naked_url_https_simple() {
        assert_eq!(
            recognize_naked_url("https://example.org"),
            Ok(("", "https://example.org"))
        );
    }

    #[test]
    fn test_recognize_naked_url_with_path() {
        assert_eq!(
            recognize_naked_url("http://example.com/path/to/page"),
            Ok(("", "http://example.com/path/to/page"))
        );
    }

    #[test]
    fn test_recognize_naked_url_with_query() {
        assert_eq!(
            recognize_naked_url("https://example.com?key=value&id=123"),
            Ok(("", "https://example.com?key=value&id=123"))
        );
    }

    #[test]
    fn test_recognize_naked_url_with_fragment() {
        assert_eq!(
            recognize_naked_url("http://example.com/page#section"),
            Ok(("", "http://example.com/page#section"))
        );
    }

    #[test]
    fn test_recognize_naked_url_with_port() {
        assert_eq!(
            recognize_naked_url("http://localhost:8080/"),
            Ok(("", "http://localhost:8080/"))
        );
    }

    #[test]
    fn test_recognize_naked_url_with_username_password() {
        assert_eq!(
            recognize_naked_url("http://user:pass@host.com"),
            Ok(("", "http://user:pass@host.com"))
        );
    }

    #[test]
    fn test_recognize_naked_url_with_hyphen_underscore_dot() {
        assert_eq!(
            recognize_naked_url("https://sub-domain_name.example.com"),
            Ok(("", "https://sub-domain_name.example.com"))
        );
    }

    #[test]
    fn test_recognize_naked_url_with_percentage_encoding() {
        assert_eq!(
            recognize_naked_url("http://example.com/file%20name.txt"),
            Ok(("", "http://example.com/file%20name.txt"))
        );
    }

    #[test]
    fn test_recognize_naked_url_partial_match() {
        assert_eq!(
            recognize_naked_url("http://example.com/foo bar"),
            Ok((" bar", "http://example.com/foo"))
        );
    }

    #[test]
    fn test_recognize_naked_url_no_protocol() {
        assert!(recognize_naked_url("example.com").is_err());
    }

    #[test]
    fn test_recognize_naked_url_only_protocol() {
        assert!(recognize_naked_url("http://").is_err()); // take_while1 needs at least one char after protocol
    }

    #[test]
    fn test_recognize_naked_url_invalid_char() {
        assert_eq!(
            recognize_naked_url("http://example.com/`badchar"), // ` is not allowed
            Ok(("`badchar", "http://example.com/"))
        );
    }

    // --- Tests for parse_autolink ---
    #[test]
    fn test_parse_autolink_naked_http() {
        assert_eq!(
            parse_autolink("http://example.com/path"),
            Ok(("", Element::Autolink("http://example.com/path")))
        );
    }

    #[test]
    fn test_parse_autolink_naked_https() {
        assert_eq!(
            parse_autolink("https://example.org/path"),
            Ok(("", Element::Autolink("https://example.org/path")))
        );
    }

    #[test]
    fn test_parse_autolink_delimited_http() {
        assert_eq!(
            parse_autolink("<http://example.com>"),
            Ok(("", Element::Autolink("http://example.com")))
        );
    }

    #[test]
    fn test_parse_autolink_delimited_https() {
        assert_eq!(
            parse_autolink("<https://example.org/path>"),
            Ok(("", Element::Autolink("https://example.org/path")))
        );
    }

    #[test]
    fn test_parse_autolink_delimited_with_extra_text_after() {
        assert_eq!(
            parse_autolink("<https://example.org/path> and some text"),
            Ok((
                " and some text",
                Element::Autolink("https://example.org/path")
            ))
        );
    }

    #[test]
    fn test_parse_autolink_naked_with_extra_text_after() {
        assert_eq!(
            parse_autolink("http://example.com/foo bar"),
            Ok((" bar", Element::Autolink("http://example.com/foo")))
        );
    }

    #[test]
    fn test_parse_autolink_invalid_delimited_missing_closing_tag() {
        assert!(parse_autolink("<http://example.com").is_err());
    }

    #[test]
    fn test_parse_autolink_invalid_delimited_missing_opening_tag() {
        assert_eq!(
            parse_autolink("http://example.com>"),
            Ok((">", Element::Autolink("http://example.com")))
        );
    }

    #[test]
    fn test_parse_autolink_invalid_naked_no_protocol() {
        assert!(parse_autolink("example.com").is_err());
    }

    #[test]
    fn test_parse_autolink_empty_string() {
        assert!(parse_autolink("").is_err());
    }

    #[test]
    fn test_parse_autolink_text_only() {
        assert!(parse_autolink("just some text").is_err());
    }

    #[test]
    fn test_parse_autolink_url_not_at_start() {
        assert!(parse_autolink("foo http://example.com").is_err());
    }

    #[test]
    fn test_parse_autolink_multiple_urls() {
        // This parser will only match the first one if it's a naked URL,
        // or the first delimited one.
        assert_eq!(
            parse_autolink("http://one.com https://two.org"),
            Ok((" https://two.org", Element::Autolink("http://one.com")))
        );
        assert_eq!(
            parse_autolink("<http://one.com> <https://two.org>"),
            Ok((" <https://two.org>", Element::Autolink("http://one.com")))
        );
    }
}
