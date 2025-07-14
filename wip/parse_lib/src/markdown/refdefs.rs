//! Parse link reference definitions in Markdown.
//!
//! <https://spec.commonmark.org/0.31.2/#link-reference-definitions>
//!
//! A link reference definition consists of a link label, optionally preceded by up to three spaces of indentation,
//! followed by a colon (:), optional spaces or tabs (including up to one line ending),
//! a link destination, optional spaces or tabs (including up to one line ending),
//! and an optional link title, which if it is present must be separated from the link destination by spaces or tabs.
//! No further character may occur.

use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::line_ending;
use winnow::ascii::space0;
use winnow::ascii::space1;
use winnow::combinator::opt;
use winnow::combinator::preceded;
use winnow::combinator::seq;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;
use winnow::token::take_while;

use crate::ast::*;
use crate::parse_parts::*;

/// Parses up to three spaces of indentation.
///
/// Example: "   " -> ()
fn parse_indentation(input: &mut &str) -> ModalResult<()> {
    take_while(0..=3, |c| c == ' ').void().parse_next(input)
}

/// Parses optional spaces or tabs, including up to one line ending.
/// This is used for the whitespace around the colon and destination.
///
/// Example: " \t\n " -> ()
fn parse_optional_whitespace_with_one_newline(input: &mut &str) -> ModalResult<()> {
    (
        space0,           // Zero or more spaces/tabs
        opt(line_ending), // Optional single line ending
        space0,           // Zero or more spaces/tabs after the newline
    )
        .void()
        .parse_next(input)
}

/// The main parser for a complete link reference definition.
pub fn parse_link_reference_definition<'s>(input: &mut &'s str) -> ModalResult<Element<'s>> {
    seq!(
        // Optional indentation (up to 3 spaces).
        _: parse_indentation,
        // Link label.
        parse_link_label,
        // Colon.
        _: ":",
        // Optional spaces/tabs (including up to one line ending).
        _: parse_optional_whitespace_with_one_newline,
        // Link destination.
        parse_link_destination,
        // Optional spaces/tabs (including up to one line ending) before title.
        _: opt(parse_optional_whitespace_with_one_newline),
        // Optional link title. If it is present, it must be separated from the link destination by spaces or tabs.
        opt(preceded(space1, parse_link_title)),
        _: line_ending,
    )
    .map(|(label, url, title)| {
        Element::ReferenceDefinition(ReferenceDefinitionData { label, url, title })
    })
    .context(Label(""))
    .context(Expected(Description("")))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_indentation() {
        assert_eq!(
            parse_indentation.parse_peek("   [foo]: /url"),
            Ok(("[foo]: /url", ()))
        );
        assert_eq!(
            parse_indentation.parse_peek("[foo]: /url"),
            Ok(("[foo]: /url", ()))
        );
        assert_eq!(
            parse_indentation.parse_peek("    [foo]: /url"),
            Ok((" [foo]: /url", ()))
        );
    }

    #[test]
    fn test_parse_optional_whitespace_with_one_newline() {
        assert_eq!(
            parse_optional_whitespace_with_one_newline.parse_peek(" "),
            Ok(("", ()))
        );
        assert_eq!(
            parse_optional_whitespace_with_one_newline.parse_peek("\t"),
            Ok(("", ()))
        );
        assert_eq!(
            parse_optional_whitespace_with_one_newline.parse_peek(" \t "),
            Ok(("", ()))
        );
        assert_eq!(
            parse_optional_whitespace_with_one_newline.parse_peek("\n"),
            Ok(("", ()))
        );
        assert_eq!(
            parse_optional_whitespace_with_one_newline.parse_peek(" \n "),
            Ok(("", ()))
        );
        assert_eq!(
            parse_optional_whitespace_with_one_newline.parse_peek(" \r\n "),
            Ok(("", ()))
        );
        assert_eq!(
            parse_optional_whitespace_with_one_newline.parse_peek("  \n  more"),
            Ok(("more", ()))
        );
        // Should only consume one newline
        assert_eq!(
            parse_optional_whitespace_with_one_newline.parse_peek(" \n\n "),
            Ok(("\n ", ()))
        );
    }

    #[test]
    fn test_parse_link_reference_definition_simple() {
        let input = "[foo]: /url";
        let expected = Element::ReferenceDefinition(ReferenceDefinitionData {
            label: "foo",
            url: "/url",
            title: None,
        });
        assert_eq!(
            parse_link_reference_definition.parse_peek(input),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_parse_link_reference_definition_with_title() {
        let input = "[foo]: /url \"My Title\"";
        let expected = Element::ReferenceDefinition(ReferenceDefinitionData {
            label: "foo",
            url: "/url",
            title: Some("My Title"),
        });
        assert_eq!(
            parse_link_reference_definition.parse_peek(input),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_parse_link_reference_definition_with_single_quoted_title() {
        let input = "[bar]: /some/path 'Another Title'";
        let expected = Element::ReferenceDefinition(ReferenceDefinitionData {
            label: "bar",
            url: "/some/path",
            title: Some("Another Title"),
        });
        assert_eq!(
            parse_link_reference_definition.parse_peek(input),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_parse_link_reference_definition_with_parenthesized_title() {
        let input = "[baz]: /dest (Title with Parens)";
        let expected = Element::ReferenceDefinition(ReferenceDefinitionData {
            label: "baz",
            url: "/dest",
            title: Some("Title with Parens"),
        });
        assert_eq!(
            parse_link_reference_definition.parse_peek(input),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_parse_link_reference_definition_with_indentation() {
        let input = " [qux]: /url";
        let expected = Element::ReferenceDefinition(ReferenceDefinitionData {
            label: "qux",
            url: "/url",
            title: None,
        });
        assert_eq!(
            parse_link_reference_definition.parse_peek(input),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_parse_link_reference_definition_with_max_indentation() {
        let input = "   [qux]: /url";
        let expected = Element::ReferenceDefinition(ReferenceDefinitionData {
            label: "qux",
            url: "/url",
            title: None,
        });
        assert_eq!(
            parse_link_reference_definition.parse_peek(input),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_parse_link_reference_definition_with_excess_indentation_should_fail() {
        let mut input = "    [qux]: /url"; // 4 spaces.
        assert!(parse_link_reference_definition(&mut input).is_err());
    }

    #[test]
    fn test_parse_link_reference_definition_with_newline_after_colon() {
        let input = "[foo]:\n/url";
        let expected = Element::ReferenceDefinition(ReferenceDefinitionData {
            label: "foo",
            url: "/url",
            title: None,
        });
        assert_eq!(
            parse_link_reference_definition.parse_peek(input),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_parse_link_reference_definition_with_newline_before_title() {
        let input = "[foo]: /url\n\"My Title\"";
        let expected = Element::ReferenceDefinition(ReferenceDefinitionData {
            label: "foo",
            url: "/url",
            title: Some("My Title"),
        });
        assert_eq!(
            parse_link_reference_definition.parse_peek(input),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_parse_link_reference_definition_with_angle_bracket_destination() {
        let input = "[foo]: <http://example.com/long/path?query=1> \"A long title\"";
        let expected = Element::ReferenceDefinition(ReferenceDefinitionData {
            label: "foo",
            url: "http://example.com/long/path?query=1",
            title: Some("A long title"),
        });
        assert_eq!(
            parse_link_reference_definition.parse_peek(input),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_parse_link_reference_definition_complex_whitespace() {
        let input = " [test]:  \t\n  <http://example.org/a/b> \t\n  'Complex Title' ";
        let expected = Element::ReferenceDefinition(ReferenceDefinitionData {
            label: "test",
            url: "http://example.org/a/b",
            title: Some("Complex Title"),
        });
        assert_eq!(
            parse_link_reference_definition.parse_peek(input),
            Ok((" ", expected))
        );
    }

    #[test]
    fn test_parse_link_reference_definition_no_title_with_trailing_whitespace() {
        let input = "[foo]: /url ";
        let expected = Element::ReferenceDefinition(ReferenceDefinitionData {
            label: "foo",
            url: "/url",
            title: None,
        });
        assert_eq!(
            parse_link_reference_definition.parse_peek(input),
            Ok((" ", expected))
        );
    }
}
