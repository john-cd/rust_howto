// use nom::IResult;
// use nom::Parser;
// use nom::bytes::complete::is_not;
// use nom::bytes::complete::tag;
// use nom::character::complete::char;
// use nom::combinator::map;
// use nom::sequence::delimited;

// use super::ast::Element;

// TODO finish: add reference-style image parsing

// /// Parses an image: `![desc](url "title")`.
// pub fn parse_inline_image<'a>(input: &'a str) -> IResult<&'a str, Element<'a>> {
//     map(
//         (
//             delimited(tag("!["), is_not("]"), char(']')), // Link text.
//             delimited(char('('), is_not(")"), char(')')), // Link label.

//         ),
//         |(image_description, url, title): (&str, &str, Option<&str>)| Element::InlineImage { image_description, url, title }
//     )
//     .parse(input)
// }
