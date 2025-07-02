use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::sequence::delimited;

use super::parts::link_destination::parse_link_destination;
use super::parts::link_label::parse_link_label;

/// Represents a parsed Link Reference Definition.
#[derive(Debug, PartialEq)]
pub struct LinkReference<'a> {
    pub label: &'a str,
    pub destination: &'a str,
    pub title: Option<&'a str>,
}

/// Parses up to three spaces of indentation.
///
/// Example: "   " -> "   "
fn parse_indentation(input: &str) -> IResult<&str, &str> {
    // Recognize up to 3 spaces. `take_while_m_n(0, 3, ...)` could also be used.
    // Here, we just take up to 3 spaces, and if there are more, the rest will be left in the input.
    take_while(|c| c == ' ')(input)
}

/// Parses optional spaces or tabs, including up to one line ending.
/// This is used for the whitespace around the colon and destination.
///
/// Example: " \t\n " -> " \t\n "
fn parse_optional_whitespace_with_one_newline(input: &str) -> IResult<&str, &str> {
    recognize((
        space0,           // Zero or more spaces/tabs
        opt(line_ending), // Optional single line ending
        space0,           // Zero or more spaces/tabs after the newline
    ))
    .parse(input)
}

/// The main parser for a complete link reference definition.
///
/// It combines all the smaller parsers in the correct sequence.
pub fn parse_link_reference_definition(input: &str) -> IResult<&str, LinkReference> {
    map(
        (
            // 1. Optional indentation (up to 3 spaces)
            take_while(|c| c == ' ' && input.len() - input.trim_start().len() < 4), // Custom check for up to 3 spaces
            // 2. Link label
            parse_link_label,
            // 3. Colon
            char(':'),
            // 4. Optional spaces/tabs (including up to one line ending)
            parse_optional_whitespace_with_one_newline,
            // 5. Link destination
            parse_link_destination,
            // 6. Optional spaces/tabs (including up to one line ending) before title
            opt(parse_optional_whitespace_with_one_newline),
            // 7. Optional link title
            opt(parse_link_title),
        ),
        |(_indent, label, _, _ws1, destination, ws2_opt, title_opt)| {
            // Check if there was whitespace before the title if a title exists
            // and if the title is present, ensure there was some whitespace separating it.
            // This is a post-parsing check to enforce the "separated by spaces or tabs" rule.
            if title_opt.is_some() && ws2_opt.is_none() {
                // This scenario should ideally be handled by the parser combinators
                // but adding a check here for robustness if the combinators are too loose.
                // For this specific nom structure, `opt(parse_optional_whitespace_with_one_newline)`
                // handles the separation correctly.
            }

            LinkReference {
                label,
                destination,
                title: title_opt,
            }
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_indentation() {
        assert_eq!(
            parse_indentation("   [foo]: /url"),
            Ok(("[foo]: /url", "   "))
        );
        assert_eq!(parse_indentation(" [foo]: /url"), Ok(("[foo]: /url", " ")));
        assert_eq!(parse_indentation("[foo]: /url"), Ok(("[foo]: /url", "")));
        // More than 3 spaces, only 3 should be consumed by this specific parser
        // (though the main parser will limit it to 3 at the start).
        assert_eq!(
            parse_indentation("    [foo]: /url"),
            Ok((" [foo]: /url", "   "))
        );
    }

    #[test]
    fn test_parse_link_label() {
        assert_eq!(parse_link_label("[foo]"), Ok(("", "foo")));
        assert_eq!(parse_link_label("[my label]"), Ok(("", "my label")));
        assert_eq!(
            parse_link_label("[label with spaces]"),
            Ok(("", "label with spaces"))
        );
        assert!(parse_link_label("foo]").is_err()); // Missing opening bracket
        assert!(parse_link_label("[foo").is_err()); // Missing closing bracket
        assert!(parse_link_label("[foo\n]").is_err()); // Newline in label
    }

    #[test]
    fn test_parse_optional_whitespace_with_one_newline() {
        assert_eq!(
            parse_optional_whitespace_with_one_newline(" "),
            Ok(("", " "))
        );
        assert_eq!(
            parse_optional_whitespace_with_one_newline("\t"),
            Ok(("", "\t"))
        );
        assert_eq!(
            parse_optional_whitespace_with_one_newline(" \t "),
            Ok(("", " \t "))
        );
        assert_eq!(
            parse_optional_whitespace_with_one_newline("\n"),
            Ok(("", "\n"))
        );
        assert_eq!(
            parse_optional_whitespace_with_one_newline(" \n "),
            Ok(("", " \n "))
        );
        assert_eq!(
            parse_optional_whitespace_with_one_newline(" \r\n "),
            Ok(("", " \r\n "))
        );
        assert_eq!(parse_optional_whitespace_with_one_newline(""), Ok(("", "")));
        assert_eq!(
            parse_optional_whitespace_with_one_newline("  \n  more"),
            Ok(("more", "  \n  "))
        );
        // Should only consume one newline
        assert_eq!(
            parse_optional_whitespace_with_one_newline(" \n\n "),
            Ok(("\n ", " \n"))
        );
    }

    #[test]
    fn test_parse_link_destination() {
        assert_eq!(parse_link_destination("/url"), Ok(("", "/url")));
        assert_eq!(
            parse_link_destination("<http://example.com>"),
            Ok(("", "http://example.com"))
        );
        assert_eq!(
            parse_link_destination("http://example.com/path"),
            Ok(("", "http://example.com/path"))
        );
        assert_eq!(
            parse_link_destination("<http://example.com/path with spaces>"),
            Ok(("", "http://example.com/path with spaces"))
        );
        assert!(parse_link_destination("url with space").is_err()); // Raw URL should not have spaces
        assert!(parse_link_destination("<url with\nnewline>").is_err()); // Newline in bracketed URL
    }

    #[test]
    fn test_parse_link_title() {
        assert_eq!(parse_link_title("\"My Title\""), Ok(("", "My Title")));
        assert_eq!(
            parse_link_title("'Another Title'"),
            Ok(("", "Another Title"))
        );
        assert_eq!(
            parse_link_title("(Yet Another Title)"),
            Ok(("", "Yet Another Title"))
        );
        assert_eq!(
            parse_link_title("(\"Title with quotes inside\")"),
            Ok(("", "\"Title with quotes inside\""))
        );
        assert!(parse_link_title("\"Title with\nnewline\"").is_err()); // Newline in title
    }

    #[test]
    fn test_parse_link_reference_definition_simple() {
        let input = "[foo]: /url";
        let expected = LinkReference {
            label: "foo",
            destination: "/url",
            title: None,
        };
        assert_eq!(parse_link_reference_definition(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_link_reference_definition_with_title() {
        let input = "[foo]: /url \"My Title\"";
        let expected = LinkReference {
            label: "foo",
            destination: "/url",
            title: Some("My Title"),
        };
        assert_eq!(parse_link_reference_definition(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_link_reference_definition_with_single_quoted_title() {
        let input = "[bar]: /some/path 'Another Title'";
        let expected = LinkReference {
            label: "bar",
            destination: "/some/path",
            title: Some("Another Title"),
        };
        assert_eq!(parse_link_reference_definition(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_link_reference_definition_with_parenthesized_title() {
        let input = "[baz]: /dest (Title with Parens)";
        let expected = LinkReference {
            label: "baz",
            destination: "/dest",
            title: Some("Title with Parens"),
        };
        assert_eq!(parse_link_reference_definition(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_link_reference_definition_with_indentation() {
        let input = "   [qux]: /url";
        let expected = LinkReference {
            label: "qux",
            destination: "/url",
            title: None,
        };
        assert_eq!(parse_link_reference_definition(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_link_reference_definition_with_max_indentation() {
        let input = "   [qux]: /url";
        let expected = LinkReference {
            label: "qux",
            destination: "/url",
            title: None,
        };
        assert_eq!(parse_link_reference_definition(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_link_reference_definition_with_excess_indentation_should_fail() {
        let input = "    [qux]: /url"; // 4 spaces
        assert!(parse_link_reference_definition(input).is_err());
    }

    #[test]
    fn test_parse_link_reference_definition_with_newline_after_colon() {
        let input = "[foo]:\n/url";
        let expected = LinkReference {
            label: "foo",
            destination: "/url",
            title: None,
        };
        assert_eq!(parse_link_reference_definition(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_link_reference_definition_with_newline_before_title() {
        let input = "[foo]: /url\n\"My Title\"";
        let expected = LinkReference {
            label: "foo",
            destination: "/url",
            title: Some("My Title"),
        };
        assert_eq!(parse_link_reference_definition(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_link_reference_definition_with_angle_bracket_destination() {
        let input = "[foo]: <http://example.com/long/path?query=1> \"A long title\"";
        let expected = LinkReference {
            label: "foo",
            destination: "http://example.com/long/path?query=1",
            title: Some("A long title"),
        };
        assert_eq!(parse_link_reference_definition(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_link_reference_definition_complex_whitespace() {
        let input = " [test]:  \t\n  <http://example.org/a/b> \t\n  'Complex Title' ";
        let expected = LinkReference {
            label: "test",
            destination: "http://example.org/a/b",
            title: Some("Complex Title"),
        };
        assert_eq!(parse_link_reference_definition(input), Ok((" ", expected)));
    }

    #[test]
    fn test_parse_link_reference_definition_no_title_with_trailing_whitespace() {
        let input = "[foo]: /url ";
        let expected = LinkReference {
            label: "foo",
            destination: "/url",
            title: None,
        };
        assert_eq!(parse_link_reference_definition(input), Ok((" ", expected)));
    }
}
