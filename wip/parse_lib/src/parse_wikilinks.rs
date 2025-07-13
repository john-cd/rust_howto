use winnow::ModalResult;
use winnow::Parser;
use winnow::Result;
use winnow::combinator::alt;
use winnow::combinator::delimited;
use winnow::combinator::fail;
use winnow::combinator::opt;
use winnow::combinator::separated_pair;
use winnow::error::ErrMode;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;
use winnow::token::take_until;
use winnow::token::take_while;

use crate::ast::*;

/// Parses the inner content of a wikilink, which can be either a simple target
/// or a target and a display text separated by a pipe.
///
/// It handles optional whitespace around the target and display text.
///
/// Examples:
/// - `target]]` -> `("target", None)`
/// - `  target ]]` -> `("target", None)`
/// - `target|display]]` -> `("target", Some("display"))`
/// - `  target  |  display ]]` -> `("target", Some("display"))`
/// - `target| ]]` -> `("target", None)` (empty display text)
/// - `target|]]` -> `("target", None)` (empty display text)
fn parse_wikilink_inner<'s>(input: &mut &'s str) -> Result<(&'s str, Option<&'s str>)> {
    let piped_inner = separated_pair(take_until(1.., "|"), "|", take_until(0.., "]]"))
        .map(|(target, display): (&str, &str)| {
            (
                target.trim(),
                Some(display.trim()).filter(|s| !s.is_empty()),
            )
        })
        .verify(|(target, _)| !target.is_empty())
        .context(Label("wikilink inner elements with pipe"))
        .context(Expected(Description("target | display]]")));

    let simple_inner = take_until(1.., "]]")
        .map(|target: &str| (target.trim(), None))
        .verify(|(target, _)| !target.is_empty())
        .context(Label("wikilink inner elements without pipe"))
        .context(Expected(Description("target]]")));

    alt((
        piped_inner,
        simple_inner,
        fail.context(Label("wikilink inner elements"))
            .context(Expected(Description("target]] or target | display]]"))),
    ))
    .parse_next(input)
}

/// Parses a wikilink in the format `[[target]]` or `[[target | display]]`.
/// It also captures any alphanumeric characters immediately after the closing brackets,
/// which become part of the displayed link text.
fn parse_wikilink_to_strings<'s>(
    input: &mut &'s str,
) -> Result<((&'s str, Option<&'s str>), Option<&'s str>)> {
    // Letters and other non-punctuation characters immediately after a wikilink's closing brackets,
    // with no intervening space, become part of its displayed link text. The target is unchanged.
    let immediately_after = opt(take_while(0.., |c: char| c.is_alphanumeric()))
        .context(Label("text immediately after wikilink"))
        .context(Expected(Description("alphanumeric characters")));

    (
        delimited("[[", parse_wikilink_inner, "]]"),
        immediately_after,
    )
        .parse_next(input)
}

/// Parses a wikilink in the format `[[target]]` or `[[target | display]]`.
/// It also captures any alphanumeric characters immediately after the closing brackets,
/// which become part of the displayed link text.
pub fn parse_wikilink<'s>(input: &mut &'s str) -> ModalResult<Element<'s>> {
    parse_wikilink_to_strings
        .map(|((target, display), immediately_after)| {
            Element::WikiLink(WikiLinkData {
                target,
                display,
                immediately_after,
            })
        })
        .context(Label("wiki link"))
        .context(Expected(Description(r#"[[page | display_text]]"#)))
        .parse_next(input)
        .map_err(|e| ErrMode::Backtrack(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to easily check successful parsing.
    fn assert_parse_wikilink_inner_ok<'a>(
        input: &'a str,
        expected_target: &'a str,
        expected_display: Option<&'a str>,
    ) {
        match parse_wikilink_inner.parse_peek(input) {
            Ok((remaining, (target, display))) => {
                assert_eq!(remaining, "", "Remaining input should be empty");
                assert_eq!(target, expected_target, "Target should match");
                assert_eq!(display, expected_display, "Display should match");
            }
            Err(e) => panic!("Parsing failed: {e:?}"),
        }
    }

    fn assert_parse_wikilink_inner_err<'s>(input: &'s str) {
        match parse_wikilink_inner.parse_peek(input) {
            Ok((_, (target, display))) => {
                panic!("Parsing unexpectedly succeeded: target='{target}', display='{display:?}'")
            }
            Err(e) => {
                println!("Expected parsing error: {e:?}");
            }
        }
    }

    #[test]
    fn test_parse_wikilink_inner_simple() {
        assert_parse_wikilink_inner_ok("Target]]", "Target", None);
    }

    #[test]
    fn test_parse_wikilink_inner_simple_with_spaces() {
        assert_parse_wikilink_inner_ok("  Target ]]", "Target", None);
    }

    #[test]
    fn test_parse_wikilink_inner_piped() {
        assert_parse_wikilink_inner_ok("Target|Display]]", "Target", Some("Display"));
    }

    #[test]
    fn test_parse_wikilink_inner_piped_with_spaces() {
        assert_parse_wikilink_inner_ok("  Target |  Display ]]", "Target", Some("Display"));
    }

    #[test]
    fn test_parse_wikilink_inner_piped_empty_display() {
        assert_parse_wikilink_inner_ok("Target| ]]", "Target", None);
        assert_parse_wikilink_inner_ok("Target|]]", "Target", None);
    }

    #[test]
    fn test_parse_wikilink_inner_empty_target_piped() {
        assert_parse_wikilink_inner_ok("|Display]]", "", Some("Display"));
    }

    #[test]
    fn test_parse_wikilink_inner_empty_target_simple() {
        assert_parse_wikilink_inner_ok("]]", "", None);
    }

    #[test]
    fn test_parse_wikilink_inner_no_closing_brackets() {
        // The parser expects ']]' so it should fail if not present.
        assert_parse_wikilink_inner_err("Target");
        assert_parse_wikilink_inner_err("Target|Display");
    }

    #[test]
    fn test_parse_wikilink_inner_just_target_and_pipe() {
        // Missing the closing brackets should result in an error.
        assert_parse_wikilink_inner_err("Target|");
    }

    #[test]
    fn test_parse_wikilink_inner_leading_pipe_simple() {
        assert_parse_wikilink_inner_ok("|Target]]", "|Target", None);
    }

    // Test `parse_wikilink` -----------------

    // [[file]]
    #[test]
    fn test_simple_wikilink() {
        assert_eq!(
            parse_wikilink.parse_peek("[[file]],"),
            Ok((
                ",",
                Element::WikiLink(WikiLinkData {
                    target: "file",
                    display: None,
                    immediately_after: None
                })
            ))
        );
    }

    // [[ file ]]
    #[test]
    fn test_simple_wikilink_with_spaces() {
        assert_eq!(
            parse_wikilink.parse_peek("[[ file ]]"),
            Ok((
                "",
                Element::WikiLink(WikiLinkData {
                    target: "file",
                    display: None,
                    immediately_after: None
                })
            ))
        );
    }

    // [[file|title]]
    #[test]
    fn test_piped_wikilink_with_display_text() {
        assert_eq!(
            parse_wikilink.parse_peek("[[file|display text]]"),
            Ok((
                "",
                Element::WikiLink(WikiLinkData {
                    target: "file",
                    display: Some("display text"),
                    immediately_after: None
                })
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
            parse_wikilink.parse_peek("[[  file  |  display text  ]]"),
            Ok((
                "",
                Element::WikiLink(WikiLinkData {
                    target: "file",
                    display: Some("display text"),
                    immediately_after: None
                })
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
            parse_wikilink.parse_peek("[[file |    ]]"),
            Ok((
                "",
                Element::WikiLink(WikiLinkData {
                    target: "file",
                    display: None,
                    immediately_after: None
                })
            ))
        );
    }

    // Failures:

    #[test]
    fn fails_on_invalid_format() {
        let input = "[[Incomplete";
        let result = parse_wikilink.parse_peek(input);
        assert!(result.is_err());
    }

    // [[]]
    #[test]
    fn fails_on_empty() {
        let input = "[[]]";
        let result = parse_wikilink.parse_peek(input);
        assert!(result.is_err());
    }

    // [[|title7]]
    // [[| title8]]
    // [[|title9 ]]
    // [[| title10 ]]
    #[test]
    fn fails_on_no_target() {
        let input = "[[| title ]]";
        let result = parse_wikilink.parse_peek(input);
        assert!(result.is_err());
    }

    // Test "immediately after" text:

    // [[file]]s
    #[test]
    fn test_simple_wikilink_with_text_after() {
        assert_eq!(
            parse_wikilink.parse_peek("[[file]]s"),
            Ok((
                "",
                Element::WikiLink(WikiLinkData {
                    target: "file",
                    display: None,
                    immediately_after: Some("s")
                })
            ))
        );
    }

    // [[file | display text]]s
    #[test]
    fn test_piped_wikilink_with_text_after() {
        assert_eq!(
            parse_wikilink.parse_peek("[[file | display text]]s"),
            Ok((
                "",
                Element::WikiLink(WikiLinkData {
                    target: "file",
                    display: Some("display text"),
                    immediately_after: Some("s")
                })
            ))
        );
    }
}
