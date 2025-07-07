use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::tag;
use nom::bytes::take_until;
use nom::bytes::take_while;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::error::context;
use nom::sequence::delimited;
use nom::sequence::separated_pair;

use super::ast::Element;

/// Parses the inner content of a wikilink, which can be either a simple target
/// or a target and a display text separated by a pipe.
///
/// It handles optional whitespace around the target and display text.
///
/// Examples:
/// - `Target]]` -> `("Target", None)`
/// - `  Target ]]` -> `("Target", None)`
/// - `Target|Display]]` -> `("Target", Some("Display"))`
/// - `  Target |  Display ]]` -> `("Target", Some("Display"))`
/// - `Target| ]]` -> `("Target", None)` (empty display text)
/// - `|Display]]` -> `("", Some("Display"))` (empty target)
/// - `]]` -> `("", None)` (empty target and display)
fn parse_wikilink_inner(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    let piped_inner = context("piped_inner error", map(
        separated_pair(take_until("|"), tag("|"), take_until("]]")),
        |(target, display): (&str, &str)| {
            (
                target.trim(),
                Some(display.trim()).filter(|s| !s.is_empty()),
            )
        },
    ));
    let simple_inner = context("simple_inner error", map(take_until("]]"), |target: &str| (target.trim(), None)));
    alt((piped_inner, simple_inner)).parse(input)
}

pub fn parse_wikilink(input: &str) -> IResult<&str, Element> {
    // Letters and other non-punctuation characters immediately after a wikilink's closing brackets,
    // with no intervening space, become part of its displayed link text. The target is unchanged.
    let immediately_after = context("immediately_after error", opt(take_while(|c: char| c.is_alphanumeric())));

    let wikilink = (
        delimited(tag("[["), parse_wikilink_inner, tag("]]")),
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
    use nom::error::ErrorKind;
    use nom::Err;

    // TODO fix failing tests below
    //
    // fn simple_inner(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    //     context("simple_inner error", map(take_until("]]"), |target: &str| (target.trim(), None))).parse(input)
    // }
    //
    // fn piped_inner(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    //     let mut piped_inner = context("piped_inner error", map(
    //         separated_pair(take_until("|"), tag("|"), take_until("]]")),
    //         |(target, display): (&str, &str)| {
    //             (
    //                 target.trim(),
    //                 Some(display.trim()).filter(|s| !s.is_empty()),
    //             )
    //         },
    //     ));
    //     piped_inner.parse(input)
    // }
    //
    // // Parse wikilinks `[[target_page]]` or `[[target_page | display]]`, with or without spaces.
    // fn parse(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    //     delimited(tag("[["), parse_wikilink_inner, tag("]]")).parse(input)
    // }
    //
    // #[test]
    // fn test() {
    //     assert_eq!(piped_inner("file|display]]"), Ok(("]]", ("file", Some("display")))));
    // }

    // Helper to easily check successful parsing.
    fn assert_parsed_ok<'a>(input: &'a str, expected_target: &'a str, expected_display: Option<&'a str>) {
        match parse_wikilink_inner(input) {
            Ok((remaining, (target, display))) => {
                assert_eq!(remaining, "", "Remaining input should be empty");
                assert_eq!(target, expected_target, "Target should match");
                assert_eq!(display, expected_display, "Display should match");
            }
            Err(e) => panic!("Parsing failed: {e:?}"),
        }
    }

    // Helper to easily check parsing errors.
    fn assert_parsed_err(input: &str, expected_error_kind: ErrorKind) {
        match parse_wikilink_inner(input) {
            Ok((_, (target, display))) => panic!("Parsing unexpectedly succeeded: target='{target}', display='{display:?}'"),
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                assert_eq!(e.to_string(), format!("ParserError {{ input: \"{input}\", kind: {expected_error_kind:?} }}"));
            },
            Err(Err::Incomplete(needed)) => panic!("Parsing incomplete, but expected error: {needed:?}"),
        }
    }

    #[test]
    fn test_parse_wikilink_inner_simple() {
        assert_parsed_ok("Target]]", "Target", None);
    }

    #[test]
    fn test_parse_wikilink_inner_simple_with_spaces() {
        assert_parsed_ok("  Target ]]", "Target", None);
    }

    #[test]
    fn test_parse_wikilink_inner_piped() {
        assert_parsed_ok("Target|Display]]", "Target", Some("Display"));
    }

    #[test]
    fn test_parse_wikilink_inner_piped_with_spaces() {
        assert_parsed_ok("  Target |  Display ]]", "Target", Some("Display"));
    }

    #[test]
    fn test_parse_wikilink_inner_piped_empty_display() {
        assert_parsed_ok("Target| ]]", "Target", None);
        assert_parsed_ok("Target|]]", "Target", None);
    }

    #[test]
    fn test_parse_wikilink_inner_empty_target_piped() {
        assert_parsed_ok("|Display]]", "", Some("Display"));
    }

    #[test]
    fn test_parse_wikilink_inner_empty_target_simple() {
        assert_parsed_ok("]]", "", None);
    }

    #[test]
    fn test_parse_wikilink_inner_no_closing_brackets() {
        // The parser expects ']]' so it should fail if not present.
        assert_parsed_err("Target", ErrorKind::TakeUntil);
        assert_parsed_err("Target|Display", ErrorKind::TakeUntil);
    }

    #[test]
    fn test_parse_wikilink_inner_just_target_and_pipe() {
        // Missing the closing brackets should result in an error.
        assert_parsed_err("Target|", ErrorKind::TakeUntil);
    }

    #[test]
    fn test_parse_wikilink_inner_leading_pipe_simple() {
        assert_parsed_ok("|Target]]", "|Target", None);
    }

    // Test `parse_wikilink`:

    // [[file]]
    #[test]
    fn test_simple_wikilink() {
        assert_eq!(
            parse_wikilink("[[file]],"),
            Ok((
                ",",
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
    // Wikipedia allows empty `display`:
    // <https://en.wikipedia.org/wiki/Help:Pipe_trick>
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

    // Test "immediately after" text:

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
