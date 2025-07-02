use nom::IResult;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::delimited;

use super::super::ast::Element;


/// Parses an image: `![desc](url "title")`.
pub fn parse_inline_image<'a>(input: &'a str) -> IResult<&'a str, Element<'a>> {
    map(
        (
            delimited(tag("!["), is_not("]"), char(']')), // Link text.
            delimited(char('('), is_not(")"), char(')')), // Link label.

        ),
        |(image_description, url, title): (&str, &str, Option<&str>)| Element::InlineImage { image_description, url, title }
    )
    .parse(input)
}


/// Parses an image: `![desc][label]`.
pub fn parse_reference_style_image<'a>(input: &'a str) -> IResult<&'a str, Element<'a>> {
    map(
        (
            delimited(tag("!["), is_not("]"), char(']')), // Link text.
            delimited(char('['), is_not("]"), char(']')), // Link label.

        ),
        |(image_description, url, title): (&str, &str, Option<&str>)| Element::InlineImage { image_description, url, title }
    )
    .parse(input)
}


// --- Image Parser ---

// /// Parses the content inside an image's alt text `![alt text]`.
// /// This content can contain other inline elements, but *not* nested images or links
// /// to prevent infinite recursion and adhere to markdown spec (alt text is usually simpler).
// fn parse_image_alt_text_content<'a>(
//     input: &'a str,
// ) -> IResult<&'a str, Vec<Element<'a>>> {
//     many0(alt((
//         map(parse_code_span, Element::Code),
//         map(parse_escaped_char, Element::EscapedChar),
//         // For simplicity, we're not allowing deeply nested balanced brackets in alt text
//         // or other images. A robust parser would need a context flag here.
//         map(parse_plain_text, Element::Text),
//     ))).parse(input)
// }

// /// Parses a full Markdown image `![alt text](url "optional title")`.
// fn parse_image<'a>(input: &'a str) -> IResult<&'a str, Element<'a>> {
//     map(
//         (
//             tag("!["), // Starts with ![
//             parse_image_alt_text_content, // Parse alt text content
//             cut(char(']')), // Must have closing ]
//             cut(char('(')), // Must have opening ( for URL
//             parse_url,      // Parse the URL
//             opt(preceded(many0(char(' ')), parse_title)), // Optional title
//             cut(char(')')), // Must have closing )
//         ),
//         |(_, alt_text_content, _, _, url, title, _)| Element::Image {
//             alt_text: alt_text_content,
//             url,
//             title,
//         },
//     ).parse(input)
// }
