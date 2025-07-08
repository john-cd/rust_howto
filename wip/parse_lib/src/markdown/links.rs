use winnow::Result;
use winnow::Parser;
use winnow::bytes::complete::is_not;
use winnow::bytes::one_of;
use winnow::combinator::map;
use winnow::combinator::opt;
use winnow::combinator::delimited;

use super::super::ast::Element;
use winnow::prelude::*;

// /// Parses
// fn parse_url_and_title<'s>(input: &mut &'s str) -> Result< (&'s str, Option<&'s str>)> {
//     alt((
//         delimited("(", is_not(")"), ")"),
//         delimited("(", is_not(r#")""#), ")"),
// fail
//     .context(Label(""))
// .context(Expected(Description("")))
//     ))
//     .parse_next(input)
// }

/// Parses a Markdown-style inline link: `[text](url)`.
fn parse_inline_link<'a>(input: &mut &'a str) -> Result< Element<'a>> {
    map(
        (
            delimited("[", is_not("]"), "]"), // Link text.
            // Link URL and title
            delimited("(", is_not(")"), ")"),
            opt(parse_quoted_string),
        ), // Link title.
        |(text, url, title): (&'s str, &'s str, Option<&'s str>)| Element::InlineLink { text, url, title },
    )
    .parse_next(input)
}

/// Parses a reference-style link: `[text][label]`.
fn parse_reference_style_link<'a>(input: &mut &'a str) -> Result< Element<'a>> {
    map(
        (
            delimited("[", is_not("]"), "]"), // Link text.
            delimited("[", is_not("]"), "]"), // Link label.
        ),
        |(text, label): (&'s str, &'s str)| Element::ReferenceStyleLink { text, label },
    )
    .parse_next(input)
}
