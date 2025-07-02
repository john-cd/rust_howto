//! Parses Markdown Headings
//! < commonMark spec TODO >

// TODO finish: should parse {#some-anchor}

use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, line_ending, space0, space1},
    combinator::{map, opt, recognize},
    multi::{count, many0, many_m_n},
    sequence::{delimited, preceded, terminated, tuple},
};

use super::ast::Element;

#[derive(Debug, PartialEq)]
struct Heading<'a> {
    level: usize,
    content: &'a str,
}

// ```rust
// fn main() {
//     let input = "  ## Hello World\n";
//     let result = heading(input);
//     println!("{:?}", result);
//     // Ok(("", Element::Heading { level: 2, content: "Hello World" }))
// }
// ```
fn heading(input: &str) -> IResult<&str, Element> {
    let (input, _) = take_while(|c| c == ' ')[..=3](input)?; // Up to 3 leading spaces
    let (input, hashes) = take_while1(|c| c == '#')(input)?;
    let level = hashes.len().min(6); // Clamp to 6

    let (input, content) = opt(preceded(
        space1,
        map(
            recognize(tuple((
                take_while(|c| c != '\n' && c != '#'),
                opt(tuple((
                    space1,
                    take_while(|c| c == '#'),
                    space0,
                ))),
            ))),
            str::trim,
        ),
    )).parse(input)?;

    let (input, _) = line_ending(input)?;

    Ok((input, Element::Heading {
        level,
        content: content.unwrap_or(""),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_heading() {
        let input = "# Element 1\n";
        let result = heading(input);
        assert_eq!(
            result,
            Ok(("", Element { level: 1, content: "Heading 1" }))
        );
    }

    #[test]
    fn parses_heading_with_trailing_hashes() {
        let input = "### A title with hashes ### \n";
        let result = heading(input);
        assert_eq!(
            result,
            Ok(("", Element { level: 3, content: "A title with hashes" }))
        );
    }

    #[test]
    fn parses_heading_with_max_level() {
        let input = "###### Deep heading\n";
        let result = heading(input);
        assert_eq!(
            result,
            Ok(("", Element { level: 6, content: "Deep heading" }))
        );
    }

    #[test]
    fn limits_heading_level_to_six() {
        let input = "####### Too many hashes\n";
        let result = heading(input);
        assert_eq!(
            result,
            Ok(("", Element { level: 6, content: "Too many hashes" }))
        );
    }

    #[test]
    fn parses_heading_with_leading_spaces() {
        let input = "   ### Indented heading\n";
        let result = heading(input);
        assert_eq!(
            result,
            Ok(("", Element { level: 3, content: "Indented heading" }))
        );
    }

    #[test]
    fn fails_without_newline() {
        let input = "## No newline";
        let result = heading(input);
        assert!(result.is_err());
    }
}
