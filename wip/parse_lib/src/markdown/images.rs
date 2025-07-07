use winnow::Result;
use winnow::Parser;
use winnow::bytes::complete::is_not;
use winnow::token::literal;
use winnow::bytes::one_of;
use winnow::combinator::map;
use winnow::combinator::delimited;

use super::super::ast::Element;
use winnow::prelude::*;

/// Parses an image: `![desc](url "title")`.
pub fn parse_inline_image<'a>(input: &mut &'a str) -> Result< Element<'a>> {
    map(
        (
            delimited("![", is_not("]"), "]"), // Link text.
            delimited("(", is_not(")"), ")"), // Link label.

        ),
        |(image_description, url, title): (&'s str, &'s str, Option<&'s str>)| Element::InlineImage { image_description, url, title }
    )
    .parse_next(input)
}


/// Parses an image: `![desc][label]`.
pub fn parse_reference_style_image<'a>(input: &mut &'a str) -> Result< Element<'a>> {
    map(
        (
            delimited("![", is_not("]"), "]"), // Link text.
            delimited("[", is_not("]"), "]"), // Link label.

        ),
        |(image_description, url, title): (&'s str, &'s str, Option<&'s str>)| Element::InlineImage { image_description, url, title }
    )
    .parse_next(input)
}


// --- Image Parser ---

// /// Parses the content inside an image's alt text `![alt text]`.
// /// This content can contain other inline elements, but *not* nested images or links
// /// to prevent infinite recursion and adhere to markdown spec (alt text is usually simpler).
// fn parse_image_alt_text_content<'a>(
//     input: &mut &'a str,
// ) -> Result< Vec<Element<'a>>> {
//     many0(alt((
//         map(parse_code_span, Element::Code),
//         map(parse_escaped_char, Element::EscapedChar),
//         // For simplicity, we're not allowing deeply nested balanced brackets in alt text
//         // or other images. A robust parser would need a context flag here.
//         map(parse_plain_text, Element::Text),
//     ))).parse_next(input)
// }

// /// Parses a full Markdown image `![alt text](url "optional title")`.
// fn parse_image<'a>(input: &mut &'a str) -> Result< Element<'a>> {
//     map(
//         (
//             "![", // Starts with ![
//             parse_image_alt_text_content, // Parse alt text content
//             cut("]"), // Must have closing ]
//             cut("("), // Must have opening ( for URL
//             parse_url,      // Parse the URL
//             opt(preceded(many0(" "), parse_title)), // Optional title
//             cut(")"), // Must have closing )
//         ),
//         |(_, alt_text_content, _, _, url, title, _)| Element::Image {
//             alt_text: alt_text_content,
//             url,
//             title,
//         },
//     ).parse_next(input)
// }
