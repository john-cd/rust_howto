//! Parses link text.
//!
//! Simplified from <https://spec.commonmark.org/0.31.2/#link-text>
//!
//! A link text consists of a sequence of zero or more inline elements enclosed by square brackets ([ and ]). The following rules apply:
//! Links may not contain other links, at any level of nesting. If multiple otherwise valid link definitions appear nested inside each other, the inner-most definition is used.
//! Brackets are allowed in the link text only if (a) they are backslash-escaped or (b) they appear as a matched pair of brackets, with an open bracket [, a sequence of zero or more inlines, and a close bracket ].
//! Backtick code spans, autolinks, and raw HTML tags bind more tightly than the brackets in link text. Thus, for example, [foo`]` could not be a link text, since the second ] is part of a code span.
//! The brackets in link text bind more tightly than markers for emphasis and strong emphasis. Thus, for example, `*[foo*](url)` is a link.

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_till},
    character::complete::{char, none_of},
    combinator::{cut, map, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};
use nom::Parser;
use nom::{
    bytes::complete::{ is_not,take_till1, take_until, take_while},
    character::complete::{one_of},
    combinator::value,
};

use super::super::ast::Element;

// // Parses plain text without square brackets or '!' (to avoid images).
// fn plain_text(input: &str) -> IResult<&str, Element> {
//     map(is_not("[]!"), Element::Text).parse(input)
// }

// // Recursively parses content within link text brackets.
// fn link_text(input: &str) -> IResult<&str, Vec<Element>> {
//     delimited(
//         char('['),
//         many0(alt((image, plain_text))),
//         char(']')
//     ).parse(input)
// }

// --- Main Link Text Parser ---

// pub fn parse_link_text<'a>(input: &'a str) -> IResult<&'a str, &str> {
//     delimited(
//         char('['),
//         parse_link_text_inline_content,
//         cut(char(']')), // `cut` means if we see `[` but no matching `]`, it's a failure.
//     ).parse(input)
// }




// --- AST Definition for Inline Elements ---
// This enum will represent the different types of content that can appear
// within a link's text.
#[derive(Debug, PartialEq)]
pub enum Element<'a> {
    Text(&'a str),
    Code(&'a str),
    Image {
        alt_text: Vec<Element<'a>>, // Alt text can contain other inlines (but not images or links themselves)
        url: &'a str,
        title: Option<&'a str>,
    },
    EscapedChar(char),
    NestedBalancedBrackets(Vec<Element<'a>>), // For `[foo [bar] baz]`
}

// --- Helper Parsers ---

/// Parses a plain text segment. It stops at any character that might start a new inline element.
fn parse_plain_text(input: &str) -> IResult<&str, &str> {
    // Characters that could start another inline element or are special to link text:
    // ` (code), ! (image), [ (nested bracket/link), \ (escape), ] (end of link text)
    recognize(many1(none_of(r"`![\]"))).parse(input)
}

/// Parses an escaped character (e.g., `\[`, `\]`, ``\` ``).
fn parse_escaped_char(input: &str) -> IResult<&str, char> {
    preceded(char('\\'), one_of(r"`![]\")).parse(input)
}

/// Parses a backtick code span.
fn parse_code_span(input: &str) -> IResult<&str, &str> {
    delimited(char('`'), take_till1(|c| c == '`'), cut(char('`'))).parse(input)
}

/// Parses the URL part of a link or image.
/// Handles simple URLs, potentially with spaces if quoted.
fn parse_url(input: &str) -> IResult<&str, &str> {
    // A URL can be anything until the next ')' or '"' (if a title is present)
    // We'll simplify and just take characters until the closing parenthesis or a space.
    // In a real parser, this would be more robust to handle angle brackets for URLs.
    take_till1(|c| c == ')' || c == ' ')(input)
}

/// Parses the optional title part of a link or image (e.g., "This is a title").
fn parse_title(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), is_not("\""), char('"')).parse(input)
}



// --- Recursive Link Text Inline Content Parser ---

/// Parses the content *within* a Markdown link's text `[this content here]`.
/// This is where the core logic for allowing nested elements and handling binding precedence lives.
///
/// Rules applied here:
/// - Links may not contain other links (enforced by not calling a full `parse_link` here).
/// - Brackets are allowed if escaped or matched.
/// - Backtick code spans and images bind more tightly (parsed first).
pub fn parse_link_text_inline_content<'a>(
    input: &'a str,
) -> IResult<&'a str, Vec<Element<'a>>> {
    many0(alt((
        // 1. Elements that bind tighter (Code Spans, Images)
        map(parse_code_span, Element::Code),
        parse_image, // Image parser

        // 2. Escaped Brackets
        map(parse_escaped_char, Element::EscapedChar),

        // 3. Matched Pairs of Brackets (e.g., `[foo [bar] baz]`)
        // This is a recursive call to parse content within the nested brackets.
        // Crucially, this inner content *cannot* contain full links or images that are themselves links.
        map(
            delimited(
                char('['),
                // Recursively call this same function to parse content within nested brackets.
                // This ensures balanced brackets are handled.
                // We pass a 'context' (e.g., a flag) in a real parser to prevent
                // parsing full links/images that are links here. For this example,
                // the `parse_link_text_inline_content` itself doesn't try to parse a full link.
                parse_link_text_inline_content,
                cut(char(']')), // `cut` ensures a matching `]` must be found
            ),
            Element::NestedBalancedBrackets,
        ),

        // 4. Plain Text (anything else)
        map(parse_plain_text, Element::Text),
    )))(input)
}

// --- Main Link Text Parser ---

/// Parses the entire link text part of a Markdown link `[link text]`.
/// This is the entry point for parsing the content between the square brackets.
pub fn parse_link_text<'a>(input: &'a str) -> IResult<&'a str, Vec<Element<'a>>> {
    delimited(
        char('['),
        parse_link_text_inline_content, // Use the recursive inline content parser
        cut(char(']')),                 // `cut` ensures a matching closing bracket is found
    )(input)
}

// --- Test Cases ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_link_text() {
        assert_eq!(parse_link_text("[]").unwrap().1, vec![]);
    }

    #[test]
    fn test_plain_text_link_text() {
        assert_eq!(
            parse_link_text("[Hello world]").unwrap().1,
            vec![Element::Text("Hello world")]
        );
    }

    #[test]
    fn test_link_text_with_code_span() {
        assert_eq!(
            parse_link_text("[This is `code` here]").unwrap().1,
            vec![
                Element::Text("This is "),
                Element::Code("code"),
                Element::Text(" here")
            ]
        );
    }

    #[test]
    fn test_link_text_with_escaped_brackets() {
        assert_eq!(
            parse_link_text("[A \\[bracket\\] test]").unwrap().1,
            vec![
                Element::Text("A "),
                Element::EscapedChar('['),
                Element::Text("bracket"),
                Element::EscapedChar(']'),
                Element::Text(" test")
            ]
        );
    }

    #[test]
    fn test_link_text_with_nested_balanced_brackets() {
        assert_eq!(
            parse_link_text("[Outer [inner] text]").unwrap().1,
            vec![
                Element::Text("Outer "),
                Element::NestedBalancedBrackets(vec![Element::Text("inner")]),
                Element::Text(" text")
            ]
        );
    }

    #[test]
    fn test_link_text_with_nested_balanced_brackets_and_code() {
        assert_eq!(
            parse_link_text("[Outer [`code`] text]").unwrap().1,
            vec![
                Element::Text("Outer "),
                Element::NestedBalancedBrackets(vec![Element::Code("code")]),
                Element::Text(" text")
            ]
        );
    }

    #[test]
    fn test_link_text_with_image() {
        let expected_image = Element::Image {
            alt_text: vec![Element::Text("An image")],
            url: "http://example.com/img.png",
            title: None,
        };
        assert_eq!(
            parse_link_text("[Look at this ![An image](http://example.com/img.png) cool image!]")
                .unwrap()
                .1,
            vec![
                Element::Text("Look at this "),
                expected_image,
                Element::Text(" cool image!")
            ]
        );
    }

    #[test]
    fn test_link_text_with_image_with_title() {
        let expected_image = Element::Image {
            alt_text: vec![Element::Text("Another image")],
            url: "/path/to/img.jpg",
            title: Some("My Image Title"),
        };
        assert_eq!(
            parse_link_text(
                "[Here's ![Another image](/path/to/img.jpg \"My Image Title\") with a title]"
            )
            .unwrap()
            .1,
            vec![
                Element::Text("Here's "),
                expected_image,
                Element::Text(" with a title")
            ]
        );
    }

    #[test]
    fn test_link_text_with_image_and_code_in_alt_text() {
        let expected_image = Element::Image {
            alt_text: vec![Element::Text("Image with "), Element::Code("alt")],
            url: "data:image/gif;base64,...",
            title: None,
        };
        assert_eq!(
            parse_link_text("[Complex ![Image with `alt`] (data:image/gif;base64,...) example]")
                .unwrap()
                .1,
            vec![
                Element::Text("Complex "),
                expected_image,
                Element::Text(" example")
            ]
        );
    }

    #[test]
    fn test_link_text_with_multiple_elements() {
        let expected_image = Element::Image {
            alt_text: vec![Element::Text("Pic")],
            url: "/pic.png",
            title: None,
        };
        assert_eq!(
            parse_link_text("[Text `code` \\[escaped\\] [nested] ![Pic](/pic.png) end]")
                .unwrap()
                .1,
            vec![
                Element::Text("Text "),
                Element::Code("code"),
                Element::Text(" "),
                Element::EscapedChar('['),
                Element::Text("escaped"),
                Element::EscapedChar(']'),
                Element::Text(" "),
                Element::NestedBalancedBrackets(vec![Element::Text("nested")]),
                Element::Text(" "),
                expected_image,
                Element::Text(" end")
            ]
        );
    }

    #[test]
    fn test_invalid_unmatched_bracket() {
        // Should fail because of unmatched opening bracket
        assert!(parse_link_text("[foo [bar]").is_err());
    }

    #[test]
    fn test_invalid_code_span_binding() {
        // `]` is part of the code span, not the link text.
        // The `parse_code_span` should consume it.
        assert!(parse_link_text("[foo`]`").is_err()); // This will fail because `parse_code_span` consumes `]`, leaving no `]` for `delimited`
    }
}
