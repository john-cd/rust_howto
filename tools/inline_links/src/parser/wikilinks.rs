use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::take_while;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::sequence::separated_pair;

use super::ast::Element;

// Parse wikilinks `[[target_page]]` or `[[target_page | display]]`, with or without spaces.
//
// ```rust
// fn main() {
//     let input1 = "[[Rust]]";
//     let input2 = "[[rust_lang | Rust]]";
//
//     println!("{:?}", parse_wikilink(input1)); // Ok(("", Element::WikiLink { target: "Rust", display: None }))
//     println!("{:?}", parse_wikilink(input2)); // Ok(("", Element::WikiLink { target: "rust_lang", display: Some("Rust") }))
// }
// ```
fn parse_wikilink(input: &str) -> IResult<&str, Element> {
    let piped_link = map(
        separated_pair(take_until("|"), tag("|"), take_until("]]")),
        |(target, display): (&str, &str)| {
            (
                target.trim(),
                Some(display.trim()).filter(|s| !s.is_empty()),
            )
        },
    );

    let simple_link = map(take_until("]]"), |target: &str| (target.trim(), None));

    let link_parts = alt((piped_link, simple_link));

    // Letters and other non-punctuation characters immediately after a wikilink's closing brackets,
    // with no intervening space, become part of its displayed link text. The target is unchanged.
    let immediately_after = opt(take_while(|c: char| c.is_alphanumeric()));

    let wikilink = (
        delimited(tag("[["), link_parts, tag("]]")),
        immediately_after,
    );

    // `map_res` applies a function returning a `Result` over the result of a parser.
    let mut parser = map_res(wikilink, |((target, display), immediately_after)| {
        if target.is_empty() {
            Err("Wikilink target cannot be empty.")
        } else {
            Ok(Element::WikiLink {
                target,
                display,
                immediately_after,
            })
        }
    });

    parser.parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // [[file]]
    #[test]
    fn test_simple_wikilink() {
        assert_eq!(
            parse_wikilink("[[file]]"),
            Ok((
                "",
                Element::WikiLink {
                    target: "file",
                    display: None,
                    immediately_after: None
                }
            ))
        );
    }

    // [[ file ]]
    #[test]
    fn test_simple_wikilink_with_spaces() {
        assert_eq!(
            parse_wikilink("[[ file ]]"),
            Ok((
                "",
                Element::WikiLink {
                    target: "file",
                    display: None,
                    immediately_after: None
                }
            ))
        );
    }

    // [[file|title]]
    #[test]
    fn test_piped_wikilink_with_display_text() {
        assert_eq!(
            parse_wikilink("[[file|display text]]"),
            Ok((
                "",
                Element::WikiLink {
                    target: "file",
                    display: Some("display text"),
                    immediately_after: None
                }
            ))
        );
    }

    // [[ file | title2 ]]
    // [[ file|title3]]
    // [[file |title4]]
    // [[file| title5]]
    // [[file|title6 ]]
    #[test]
    fn test_piped_wikilink_with_spaces() {
        assert_eq!(
            parse_wikilink("[[  file  |  display text  ]]"),
            Ok((
                "",
                Element::WikiLink {
                    target: "file",
                    display: Some("display text"),
                    immediately_after: None
                }
            ))
        );
    }

    // [[ file |]]
    // [[ file |  ]]
    #[test]
    fn test_piped_wikilink_without_display() {
        assert_eq!(
            parse_wikilink("[[file |    ]]"),
            Ok((
                "",
                Element::WikiLink {
                    target: "file",
                    display: None,
                    immediately_after: None
                }
            ))
        );
    }

    // Failures:

    #[test]
    fn fails_on_invalid_format() {
        let input = "[[Incomplete";
        let result = parse_wikilink(input);
        assert!(result.is_err());
    }

    // [[]]
    #[test]
    fn fails_on_empty() {
        let input = "[[]]";
        let result = parse_wikilink(input);
        assert!(result.is_err());
    }

    // [[|title7]]
    // [[| title8]]
    // [[|title9 ]]
    // [[| title10 ]]
    #[test]
    fn fails_on_no_target() {
        let input = "[[| title ]]";
        let result = parse_wikilink(input);
        assert!(result.is_err());
    }

    // Immediataley after:

    // [[file]]s
    #[test]
    fn test_simple_wikilink_with_text_after() {
        assert_eq!(
            parse_wikilink("[[file]]s"),
            Ok((
                "",
                Element::WikiLink {
                    target: "file",
                    display: None,
                    immediately_after: Some("s")
                }
            ))
        );
    }

    // [[file | display text]]s
    #[test]
    fn test_piped_wikilink_with_text_after() {
        assert_eq!(
            parse_wikilink("[[file | display text]]s"),
            Ok((
                "",
                Element::WikiLink {
                    target: "file",
                    display: Some("display text"),
                    immediately_after: Some("s")
                }
            ))
        );
    }
}
