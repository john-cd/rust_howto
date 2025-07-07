use winnow::Parser;

use winnow::Result;
use winnow::ascii::Caseless;
use winnow::ascii::space0;
use winnow::ascii::space1;
use winnow::combinator::delimited;
use winnow::token::literal;
use winnow::token::take_until;

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
fn parse_hidden_div_open_tag<'s>(input: &mut &'s str) -> Result<&'s str> {
    (
        "<",
        space0,
        "div",
        space1, // At least one whitespace after "div"
        literal(Caseless("class")),
        space0, // Optional whitespace before '='
        "=",
        space0, // Optional whitespace after '='
        literal(Caseless(r#""hidden""#)), // Case-insensitive for "hidden"
        space0, // Optional whitespace before '>'
        ">",
    )
        .take()
        .parse_next(input)
}

/// Parses a hidden HTML div block: `<div class="hidden">...</div>`.
/// This is a simplified parser and does not handle nested divs properly.
pub fn parse_hidden_html_div<'s>(input: &mut &'s str) -> Result<Element<'s>> {
    delimited(
        parse_hidden_div_open_tag,
        take_until(0.., "</div>"), // Content of the div.
        "</div>",
    )
    .map(Element::HiddenHtmlDiv)
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_simple_hidden_div() {
        let input = r#"<div class="hidden">some content</div>"#;
        let result = parse_hidden_html_div.parse_peek(input);
        assert_eq!(result, Ok(("", Element::HiddenHtmlDiv("some content"))));
    }

    #[test]
    fn test_parse_hidden_div_with_trailing_text() {
        let input = r#"<div class="hidden">content</div> and more"#;
        let result = parse_hidden_html_div.parse_peek(input);
        assert_eq!(result, Ok((" and more", Element::HiddenHtmlDiv("content"))));
    }

    #[test]
    fn test_parse_empty_hidden_div() {
        let input = r#"<div class="hidden"></div>"#;
        let result = parse_hidden_html_div.parse_peek(input);
        assert_eq!(result, Ok(("", Element::HiddenHtmlDiv(""))));
    }

    #[test]
    fn test_parse_multiline_hidden_div() {
        let input = "<div class=\"hidden\">line 1\nline 2</div>";
        let result = parse_hidden_html_div.parse_peek(input);
        assert_eq!(result, Ok(("", Element::HiddenHtmlDiv("line 1\nline 2"))));
    }

    #[test]
    fn test_fail_on_missing_closing_tag() {
        let mut input = r#"<div class="hidden">no closing tag"#;
        assert!(parse_hidden_html_div(&mut input).is_err());
    }

    #[test]
    fn test_fail_on_wrong_class() {
        let mut input = r#"<div class="other">content</div>"#;
        assert!(parse_hidden_html_div(&mut input).is_err());
    }

    #[test]
    fn test_nested_div_behavior() {
        // The parser is simple and takes until the first "</div>".
        let input = r#"<div class="hidden">outer<div>inner</div></div>"#;
        let result = parse_hidden_html_div.parse_peek(input);
        assert_eq!(
            result,
            Ok(("</div>", Element::HiddenHtmlDiv("outer<div>inner")))
        );
    }

    // Tests for `parse_hidden_div_open_tag`.

    #[test]
    fn test_parse_hidden_div_open_tag_basic() {
        assert_eq!(
            parse_hidden_div_open_tag.parse_peek(r#"<div class="hidden">hello"#),
            Ok(("hello", r#"<div class="hidden">"#))
        );
    }

    #[test]
    fn test_parse_hidden_div_open_tag_flexible_whitespace() {
        assert_eq!(
            parse_hidden_div_open_tag.parse_peek(r#"<div   class="hidden"  >content"#),
            Ok(("content", r#"<div   class="hidden"  >"#))
        );
        assert_eq!(
            parse_hidden_div_open_tag.parse_peek(r#"<div class = "hidden" >data"#),
            Ok(("data", r#"<div class = "hidden" >"#))
        );
        assert_eq!(
            parse_hidden_div_open_tag.parse_peek(r#"<div class="hidden">"#),
            Ok(("", r#"<div class="hidden">"#))
        );
    }

    #[test]
    fn test_parse_hidden_div_open_tag_case_insensitivity() {
        assert_eq!(
            parse_hidden_div_open_tag.parse_peek(r#"<div CLASS="HIDDEN">"#),
            Ok(("", r#"<div CLASS="HIDDEN">"#))
        );
        assert_eq!(
            parse_hidden_div_open_tag.parse_peek(r#"<div cLaSs="hIdDeN">"#),
            Ok(("", r#"<div cLaSs="hIdDeN">"#))
        );
    }

    #[test]
    fn test_parse_hidden_div_open_tag_no_match() {
        // Missing class
        assert!(parse_hidden_div_open_tag(&mut r#"<div>"#).is_err());
        // Wrong class value
        assert!(parse_hidden_div_open_tag(&mut r#"<div class="visible">"#).is_err());
        // Different tag
        assert!(parse_hidden_div_open_tag(&mut r#"<span class="hidden">"#).is_err());
        // Missing closing quote
        assert!(parse_hidden_div_open_tag(&mut r#"<div class="hidden>"#).is_err());
        // Missing opening quote
        assert!(parse_hidden_div_open_tag(&mut r#"<div class=hidden">"#).is_err());
        // No whitespace after div
        assert!(parse_hidden_div_open_tag(&mut r#"<divclass="hidden">"#).is_err());
    }
}
