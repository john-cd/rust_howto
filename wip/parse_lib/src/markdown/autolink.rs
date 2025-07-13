use winnow::Parser;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;
use winnow::prelude::*;

use super::super::parse_parts::parse_angle_brackets;
use crate::ast::*;

/// Parses an autolink (`<url>`).
///
/// <https://www.w3schools.com/tags//ref_urlencode.asp>
pub fn parse_autolink<'a>(input: &mut &'a str) -> ModalResult<Element<'a>> {
    parse_angle_brackets
        .map(|url| Element::Autolink(AutolinkData { url }))
        .context(Label("autolink"))
        .context(Expected(Description(
            "an autolink starting with < and ending with >",
        )))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_autolink_naked_http() {
        assert_eq!(
            parse_autolink.parse_peek("http://example.com/path"),
            Ok((
                "",
                Element::Autolink(AutolinkData {
                    url: "http://example.com/path"
                })
            ))
        );
    }

    #[test]
    fn test_parse_autolink_naked_https() {
        assert_eq!(
            parse_autolink.parse_peek("https://example.org/path"),
            Ok((
                "",
                Element::Autolink(AutolinkData {
                    url: "https://example.org/path"
                })
            ))
        );
    }

    #[test]
    fn test_parse_autolink_delimited_http() {
        assert_eq!(
            parse_autolink.parse_peek("<http://example.com>"),
            Ok((
                "",
                Element::Autolink(AutolinkData {
                    url: "http://example.com"
                })
            ))
        );
    }

    #[test]
    fn test_parse_autolink_delimited_https() {
        assert_eq!(
            parse_autolink.parse_peek("<https://example.org/path>"),
            Ok((
                "",
                Element::Autolink(AutolinkData {
                    url: "https://example.org/path"
                })
            ))
        );
    }

    #[test]
    fn test_parse_autolink_delimited_with_extra_text_after() {
        assert_eq!(
            parse_autolink.parse_peek("<https://example.org/path> and some text"),
            Ok((
                " and some text",
                Element::Autolink(AutolinkData {
                    url: "https://example.org/path"
                })
            ))
        );
    }

    #[test]
    fn test_parse_autolink_naked_with_extra_text_after() {
        assert_eq!(
            parse_autolink.parse_peek("http://example.com/foo bar"),
            Ok((
                " bar",
                Element::Autolink(AutolinkData {
                    url: "http://example.com/foo"
                })
            ))
        );
    }

    #[test]
    fn test_parse_autolink_invalid_delimited_missing_closing_tag() {
        assert!(parse_autolink.parse_peek("<http://example.com").is_err());
    }

    #[test]
    fn test_parse_autolink_invalid_delimited_missing_opening_tag() {
        assert_eq!(
            parse_autolink.parse_peek("http://example.com>"),
            Ok((
                ">",
                Element::Autolink(AutolinkData {
                    url: "http://example.com"
                })
            ))
        );
    }

    #[test]
    fn test_parse_autolink_invalid_naked_no_protocol() {
        assert!(parse_autolink(&mut "example.com").is_err());
    }

    #[test]
    fn test_parse_autolink_empty_string() {
        assert!(parse_autolink(&mut "").is_err());
    }

    #[test]
    fn test_parse_autolink_text_only() {
        assert!(parse_autolink(&mut "just some text").is_err());
    }

    #[test]
    fn test_parse_autolink_url_not_at_start() {
        assert!(parse_autolink(&mut "foo http://example.com").is_err());
    }

    #[test]
    fn test_parse_autolink_multiple_urls() {
        // This parser will only match the first one if it's a naked URL,
        // or the first delimited one.
        assert_eq!(
            parse_autolink.parse_peek("http://one.com https://two.org"),
            Ok((
                " https://two.org",
                Element::Autolink(AutolinkData {
                    url: "http://one.com"
                })
            ))
        );
        assert_eq!(
            parse_autolink.parse_peek("<http://one.com> <https://two.org>"),
            Ok((
                " <https://two.org>",
                Element::Autolink(AutolinkData {
                    url: "http://one.com"
                })
            ))
        );
    }
}
