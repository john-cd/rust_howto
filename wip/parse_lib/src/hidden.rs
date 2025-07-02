use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::sequence::delimited;

use super::ast::Element;

/// Parses an HTML opening `<div>` tag with `class="hidden"`.
///
/// It handles flexible whitespace around the tag name, attribute name,
/// and attribute value.
///
/// Examples:
/// `<div class="hidden">`
/// `<div   class="hidden"  >`
///
/// Simplification: partially case-insensitive.
fn parse_hidden_div_open_tag(input: &str) -> IResult<&str, &str> {
    recognize((
        tag("<"),
        multispace0,
        tag("div"),
        multispace1, // At least one whitespace after "div"
        tag_no_case("class"),
        multispace0, // Optional whitespace before '='
        char('='),
        multispace0, // Optional whitespace after '='
        char('"'),
        tag_no_case("hidden"), // Case-insensitive for "hidden"
        char('"'),
        multispace0, // Optional whitespace before '>'
        char('>'),
    ))
    .parse(input)
}

/// Parses a hidden HTML div block: `<div class="hidden">...</div>`.
/// This is a simplified parser and does not handle nested divs properly.
pub fn parse_hidden_html_div<'a>(input: &'a str) -> IResult<&'a str, Element<'a>> {
    map(
        delimited(
            parse_hidden_div_open_tag,
            take_until("</div>"), // Content of the div.
            tag("</div>"),
        ),
        Element::HiddenHtmlDiv,
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_simple_hidden_div() {
        let input = r#"<div class="hidden">some content</div>"#;
        let result = parse_hidden_html_div(input);
        assert_eq!(result, Ok(("", Element::HiddenHtmlDiv("some content"))));
    }

    #[test]
    fn test_parse_hidden_div_with_trailing_text() {
        let input = r#"<div class="hidden">content</div> and more"#;
        let result = parse_hidden_html_div(input);
        assert_eq!(result, Ok((" and more", Element::HiddenHtmlDiv("content"))));
    }

    #[test]
    fn test_parse_empty_hidden_div() {
        let input = r#"<div class="hidden"></div>"#;
        let result = parse_hidden_html_div(input);
        assert_eq!(result, Ok(("", Element::HiddenHtmlDiv(""))));
    }

    #[test]
    fn test_parse_multiline_hidden_div() {
        let input = "<div class=\"hidden\">line 1\nline 2</div>";
        let result = parse_hidden_html_div(input);
        assert_eq!(result, Ok(("", Element::HiddenHtmlDiv("line 1\nline 2"))));
    }

    #[test]
    fn test_fail_on_missing_closing_tag() {
        let input = r#"<div class="hidden">no closing tag"#;
        assert!(parse_hidden_html_div(input).is_err());
    }

    #[test]
    fn test_fail_on_wrong_class() {
        let input = r#"<div class="other">content</div>"#;
        assert!(parse_hidden_html_div(input).is_err());
    }

    #[test]
    fn test_nested_div_behavior() {
        // The parser is simple and takes until the first "</div>".
        let input = r#"<div class="hidden">outer<div>inner</div></div>"#;
        let result = parse_hidden_html_div(input);
        assert_eq!(
            result,
            Ok(("</div>", Element::HiddenHtmlDiv("outer<div>inner")))
        );
    }

    // Tests for `parse_hidden_div_open_tag`.

    #[test]
    fn test_parse_hidden_div_open_tag_basic() {
        assert_eq!(
            parse_hidden_div_open_tag(r#"<div class="hidden">hello"#),
            Ok(("hello", r#"<div class="hidden">"#))
        );
    }

    #[test]
    fn test_parse_hidden_div_open_tag_flexible_whitespace() {
        assert_eq!(
            parse_hidden_div_open_tag(r#"<div   class="hidden"  >content"#),
            Ok(("content", r#"<div   class="hidden"  >"#))
        );
        assert_eq!(
            parse_hidden_div_open_tag(r#"<div class = "hidden" >data"#),
            Ok(("data", r#"<div class = "hidden" >"#))
        );
        assert_eq!(
            parse_hidden_div_open_tag(r#"<div class="hidden">"#),
            Ok(("", r#"<div class="hidden">"#))
        );
    }

    #[test]
    fn test_parse_hidden_div_open_tag_case_insensitivity() {
        assert_eq!(
            parse_hidden_div_open_tag(r#"<div CLASS="HIDDEN">"#),
            Ok(("", r#"<div CLASS="HIDDEN">"#))
        );
        assert_eq!(
            parse_hidden_div_open_tag(r#"<div cLaSs="hIdDeN">"#),
            Ok(("", r#"<div cLaSs="hIdDeN">"#))
        );
    }

    #[test]
    fn test_parse_hidden_div_open_tag_no_match() {
        // Missing class
        assert!(parse_hidden_div_open_tag(r#"<div>"#).is_err());
        // Wrong class value
        assert!(parse_hidden_div_open_tag(r#"<div class="visible">"#).is_err());
        // Different tag
        assert!(parse_hidden_div_open_tag(r#"<span class="hidden">"#).is_err());
        // Missing closing quote
        assert!(parse_hidden_div_open_tag(r#"<div class="hidden>"#).is_err());
        // Missing opening quote
        assert!(parse_hidden_div_open_tag(r#"<div class=hidden">"#).is_err());
        // No whitespace after div
        assert!(parse_hidden_div_open_tag(r#"<divclass="hidden">"#).is_err());
    }
}
