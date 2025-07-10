//! Parses Markdown ATX Headings (with {# anchor} mdbook extension).
//!
//! See <https://spec.commonmark.org/0.31.2/#atx-headings>
//!
//! Example:
//!
//! ```rust
//! fn main() {
//!     let heading1 = "# My first heading {#first}\n";
//!     match parse_atx_heading.parse_peek(heading1) {
//!         Ok((rest, heading)) => {
//!             println!("Parsed: {:?}, Remaining: '{}'", heading, rest);
//!         }
//!         Err(e) => {
//!             eprintln!("Error parsing: {:?}", e);
//!         }
//!     }
//!
//!     let heading2 = "### Simple heading\n";
//!     match parse_atx_heading.parse_peek(heading2) {
//!         Ok((rest, heading)) => {
//!             println!("Parsed: {:?}, Remaining: '{}'", heading, rest);
//!         }
//!         Err(e) => {
//!             eprintln!("Error parsing: {:?}", e);
//!         }
//!     }
//!
//!     let heading3 = "# Heading with trailing hashes ###\n";
//!     match parse_atx_heading.parse_peek(heading3) {
//!         Ok((rest, heading)) => {
//!             println!("Parsed: {:?}, Remaining: '{}'", heading, rest);
//!         }
//!         Err(e) => {
//!             eprintln!("Error parsing: {:?}", e);
//!         }
//!     }
//!
//!     let heading4 = "  ## Another heading ##  {#another-one}  \n";
//!     match parse_atx_heading.parse_peek(heading4) {
//!         Ok((rest, heading)) => {
//!             println!("Parsed: {:?}, Remaining: '{}'", heading, rest);
//!         }
//!         Err(e) => {
//!             eprintln!("Error parsing: {:?}", e);
//!         }
//!     }
//!

//! }
//! ```

use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::line_ending;
use winnow::ascii::space0;
use winnow::ascii::space1;
use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::opt;
use winnow::combinator::preceded;
use winnow::combinator::repeat;
use winnow::combinator::seq;
use winnow::combinator::terminated;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;
use winnow::token::take_while;

use super::super::ast::Element;

// Identation followed by opening sequence of 1–6 unescaped # characters
fn parse_opening_hashes<'s>(input: &mut &'s str) -> ModalResult<u8> {
    preceded(
        // Parse indentation. Up to three spaces of indentation are allowed.
        take_while(0..=3, ' '),
        // Opening sequence of 1–6 unescaped # characters
        repeat(1.., "#")
            .verify_map(|count: usize| if count <= 6 { Some(count as u8) } else { None }),
    )
    .parse_next(input)
}

/// Parse Attribute Block.
/// This is a simplified parser for attributes focusing on the ID.
///
/// <https://rust-lang.github.io/mdBook/format/markdown.html#heading-attributes>
fn parse_attribute_block<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    let attributes_content_chars = |c: char| {
        c != '{' && c != '}' && c != '<' && c != '>' && c != '\\' && c != '\n' && c != '\r'
    };

    let parse_attributes_content = delimited(
        space0,
        preceded("#", take_while(1.., attributes_content_chars)),
        space0,
    )
    .verify_map(|s: &str| {
        let s = s.trim();
        if s.is_empty() {
            None // No ID specified.
        } else {
            Some(s) // Return the ID.
        }
    });

    terminated(
        delimited("{", parse_attributes_content, cut_err("}")),
        space0,
    )
    .context(Label("attribute block"))
    .context(Expected(Description(
        r"attribute block with ID within {# }. No { } < > \ or newlines inside.",
    )))
    .parse_next(input)
}

// Helper to parse content that doesn't contain a closing hash sequence or attribute block start.
fn parse_heading_content<'s>(
    input: &mut &'s str,
) -> ModalResult<(Option<&'s str>, Option<&'s str>)> {
    // Take characters until we hit a newline or a potential attribute block start '{' or #.
    let mut content =
        take_while(0.., |c| c != '\n' && c != '\r' && c != '{' && c != '#').map(|content: &str| {
            let cnt = content.trim();
            if !cnt.is_empty() { Some(cnt) } else { None }
        });

    // Optional closing sequence of any number of unescaped # characters.
    // The optional closing sequence of #s must be preceded by spaces or tabs and may be followed by spaces or tabs only.
    let mut opt_closing_hashes = opt(terminated(take_while(1.., |c| c == '#'), space0));

    seq!(
        // The opening sequence of # characters must be followed by spaces or tabs.
        _: space1,
        content, // Content of the heading.
        _: opt_closing_hashes,
        opt(parse_attribute_block),
        _: line_ending
    )
    .context(Label("heading content"))
    .context(Expected(Description(
        " a new line or heading content followed by an optional attribute block",
    )))
    .parse_next(input)
}

/// Parses a Markdown ATX heading.
pub fn parse_atx_heading<'s>(input: &mut &'s str) -> ModalResult<Element<'s>> {
    let sequence = (
        parse_opening_hashes,
        // Followed by the end of line or heading contents + optional attribute block.
        cut_err(parse_heading_content),
    );

    sequence
        .map(|(level, (content, id))| Element::Heading { level, content, id })
        .parse_next(input)
}

// Example usage and tests:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opening_hashes() {
        assert_eq!(
            parse_opening_hashes.parse_peek("# hello"),
            Ok((" hello", 1))
        );
        assert_eq!(
            parse_opening_hashes.parse_peek("   ##  world"),
            Ok(("  world", 2))
        );
        assert_eq!(
            parse_opening_hashes.parse_peek("###### test"),
            Ok((" test", 6))
        );
        assert!(
            parse_opening_hashes
                .parse_peek("####### too many hashes")
                .is_err()
        );
        assert!(parse_opening_hashes.parse_peek("no hash").is_err());
        assert!(
            parse_opening_hashes
                .parse_peek("    # too many spaces")
                .is_err()
        );
    }

    #[test]
    fn test_parse_attribute_block() {
        assert_eq!(
            parse_attribute_block.parse_peek("{#my-id}"),
            Ok(("", "my-id"))
        );
        assert_eq!(
            parse_attribute_block.parse_peek("{  #another-id  }  "),
            Ok(("", "another-id"))
        );
        assert!(parse_attribute_block.parse_peek("{}").is_err());
        assert!(parse_attribute_block.parse_peek("{  }").is_err());
        assert!(parse_attribute_block.parse_peek("{.class}").is_err()); // Our current parser only extracts ID.
        assert!(parse_attribute_block.parse_peek("{ #broken{ }").is_err()); // Nested braces.
        assert!(parse_attribute_block.parse_peek("{ #broken< }").is_err()); // Invalid char.
    }

    #[test]
    fn test_parse_atx_heading_simple() {
        assert_eq!(
            parse_atx_heading.parse_peek("# Hello, world!\n"),
            Ok((
                "",
                Element::Heading {
                    level: 1,
                    content: Some("Hello, world!"),
                    id: None
                }
            ))
        );
        assert_eq!(
            parse_atx_heading.parse_peek("  ##  Another heading  \n"),
            Ok((
                "",
                Element::Heading {
                    level: 2,
                    content: Some("Another heading"),
                    id: None
                }
            ))
        );
        assert_eq!(
            parse_atx_heading.parse_peek("   ### Third heading ### \n"),
            Ok((
                "",
                Element::Heading {
                    level: 3,
                    content: Some("Third heading"),
                    id: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_atx_heading_with_id() {
        assert_eq!(
            parse_atx_heading.parse_peek("# My Heading {#my-heading}\n"),
            Ok((
                "",
                Element::Heading {
                    level: 1,
                    content: Some("My Heading"),
                    id: Some("my-heading")
                }
            ))
        );
        assert_eq!(
            parse_atx_heading.parse_peek("## Heading with closing hashes ## {#an-id}\n"),
            Ok((
                "",
                Element::Heading {
                    level: 2,
                    content: Some("Heading with closing hashes"),
                    id: Some("an-id")
                }
            ))
        );
        assert!(
            parse_atx_heading
                .parse_peek("### No valid ID Here {#}\n")
                .is_err()
        );
        assert_eq!(
            parse_atx_heading.parse_peek("  ### Another with ident {#id-42}     \n"),
            Ok((
                "",
                Element::Heading {
                    level: 3,
                    content: Some("Another with ident"),
                    id: Some("id-42")
                }
            ))
        );
    }

    #[test]
    fn test_parse_atx_heading_edge_cases() {
        // Only hashes.
        assert_eq!(
            parse_atx_heading.parse_peek("  # \n"),
            Ok((
                "",
                Element::Heading {
                    level: 1,
                    content: None,
                    id: None
                }
            ))
        );
        // Trailing spaces after closing hashes.
        assert_eq!(
            parse_atx_heading.parse_peek("##  Content  ##   \n"),
            Ok((
                "",
                Element::Heading {
                    level: 2,
                    content: Some("Content"),
                    id: None
                }
            ))
        );
        // No newline (EOF).
        assert!(parse_atx_heading.parse_peek("### Hello").is_err());
    }
}
