use nom::IResult;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::opt;
use nom::sequence::delimited;

use super::ast::Element;

// TODO finish:

/// Parses a simple quoted string (between " and ").
fn parse_quoted_string(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), is_not(r#"""#), char('"')).parse(input)
}

// /// Parses
// fn parse_url_and_title(input: &str) -> IResult<&str, (&str, Option<&str>)> {
//     alt((
//         delimited(char('('), is_not(")"), char(')')),
//         delimited(char('('), is_not(r#")""#), char(')')),
//     ))
//     .parse(input)
// }

/// Parses a Markdown-style inline link: `[text](url)`.
fn parse_inline_link<'a>(input: &'a str) -> IResult<&'a str, Element<'a>> {
    map(
        (
            delimited(char('['), is_not("]"), char(']')), // Link text.
            // Link URL and title
            delimited(char('('), is_not(")"), char(')')),
            opt(parse_quoted_string),
        ), // Link title.
        |(text, url, title): (&str, &str, Option<&str>)| Element::InlineLink { text, url, title },
    )
    .parse(input)
}

/// Parses a reference-style link: `[text][label]`.
fn parse_reference_style_link<'a>(input: &'a str) -> IResult<&'a str, Element<'a>> {
    map(
        (
            delimited(char('['), is_not("]"), char(']')), // Link text.
            delimited(char('['), is_not("]"), char(']')), // Link label.
        ),
        |(text, label): (&str, &str)| Element::ReferenceStyleLink { text, label },
    )
    .parse(input)
}
