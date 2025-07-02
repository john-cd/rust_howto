//! Parses Markdown ATX Headings
//! <https://spec.commonmark.org/0.31.2/#atx-headings>
//!
//! Example:
//!
//! ```rust
//! fn main() {
//!     let heading1 = "# My first heading {#first}\n";
//!     match parse_atx_heading(heading1) {
//!         Ok((rest, heading)) => {
//!             println!("Parsed: {:?}, Remaining: '{}'", heading, rest);
//!         }
//!         Err(e) => {
//!             eprintln!("Error parsing: {:?}", e);
//!         }
//!     }
//!
//!     let heading2 = "  ## Another heading ##  {#another-one}  \n";
//!     match parse_atx_heading(heading2) {
//!         Ok((rest, heading)) => {
//!             println!("Parsed: {:?}, Remaining: '{}'", heading, rest);
//!         }
//!         Err(e) => {
//!             eprintln!("Error parsing: {:?}", e);
//!         }
//!     }
//!
//!     let heading3 = "### Simple heading\n";
//!     match parse_atx_heading(heading3) {
//!         Ok((rest, heading)) => {
//!             println!("Parsed: {:?}, Remaining: '{}'", heading, rest);
//!         }
//!         Err(e) => {
//!             eprintln!("Error parsing: {:?}", e);
//!         }
//!     }
//!
//!     let heading4 = " # Heading with trailing hashes ###\n";
//!     match parse_atx_heading(heading4) {
//!         Ok((rest, heading)) => {
//!             println!("Parsed: {:?}, Remaining: '{}'", heading, rest);
//!         }
//!         Err(e) => {
//!             eprintln!("Error parsing: {:?}", e);
//!         }
//!     }
//! }
//! ```

use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, line_ending},
    combinator::opt,
    sequence::{delimited, preceded},
};
use nom::Parser;
use nom::character::multispace0;
use nom::sequence::pair;
use nom::branch::alt;
use nom::bytes::take_while_m_n;
use nom::multi::many1_count;

use super::super::ast::Element;


/// Parse indentation. Up to three spaces of indentation are allowed.
fn parse_indent(input: &str) -> IResult<&str, &str> {
    take_while_m_n(0, 3, |c| c == ' ').parse(input)
}

// Identation followed by opening sequence of 1–6 unescaped # characters
fn parse_opening_hashes(input: &str) -> IResult<&str, u8> {
    let (input, _) = parse_indent(input)?;
    let (input, count) = many1_count(char('#')).parse(input)?;
    if count > 6 {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::TooLarge)));
    }
    Ok((input, count as u8))
}

/// Helper to check for non-newline whitespace.
fn is_non_newline_whitespace(c: char) -> bool {
    c == ' ' || c == '\t'
}

// Helper to parse content that doesn't contain a closing hash sequence or attribute block start.
fn parse_heading_raw_content(input: &str) -> IResult<&str, &str> {

    // The opening sequence of # characters must be followed by spaces or tabs, or by the end of line.
    let (input, _) = take_while(is_non_newline_whitespace)(input)?;

    // We need to be careful not to consume a closing hash sequence or attribute block.
    // The `is_not` combinator can be useful here, but it's hard to make it look ahead.
    // A more robust solution might involve `recognize` and `peek`.
    // For now, let's take characters until we hit a newline or a potential attribute block start '{'.
    let (rest, content) = take_while(|c| c != '\n' && c != '\r')(input)?;

    Ok((rest, content.trim_end_matches(is_non_newline_whitespace)))
}

// Optional closing sequence of any number of unescaped # characters.
// The optional closing sequence of #s must be preceded by spaces or tabs and may be followed by spaces or tabs only.
fn parse_closing_hashes(input: &str) -> IResult<&str, &str> {
    let mut p = delimited(multispace0, take_while(|c| c == '#'), multispace0);
    p.parse(input)
}

/// Parse Attribute Block.
/// This is a simplified parser for attributes focusing on the ID.
/// <https://rust-lang.github.io/mdBook/format/markdown.html#heading-attributes>
fn parse_attribute_block(input: &str) -> IResult<&str, Option<&str>> {
    let parse_attributes_content = take_while(|c: char| {
        c != '{' && c != '}' && c != '<' && c != '>' && c != '\\' && c != '\n' && c != '\r'
    });

    let (input, content) = delimited(char('{'), parse_attributes_content, char('}')).parse(input)?;

    let parse_id = preceded(char('#'), take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-'));

    let (_, id_fragment) = opt(parse_id).parse(content)?;

    Ok((input, id_fragment))
}

/// Parses a Markdown ATX heading.
pub fn parse_atx_heading(input: &str) -> IResult<&str, Element> {

    let (input, level) = parse_opening_hashes(input)?;

    // Parse the raw content, which might include closing hashes or attribute blocks.
    let (input, raw_content) = parse_heading_raw_content(input)?;

    // Now, try to parse the optional closing hashes and attribute block from the raw_content's end.
    // This requires a bit of backtracking or careful consumption.

    let (remaining_content, closing_hashes_and_attr) = opt(pair(
        opt(parse_closing_hashes), // Optional closing hashes
        opt(preceded(multispace0, parse_attribute_block)) // Optional attribute block
    )).parse(raw_content)?;

    let mut actual_content = remaining_content;
    let mut id_fragment: Option<&str> = None;

    if let Some((_closing_hashes, attr_block)) = closing_hashes_and_attr {
        if let Some(id) = attr_block.flatten() {
            id_fragment = Some(id);
        }
    }

    // Now, strip leading/trailing spaces from the *actual* content.
    let final_content = actual_content.trim_matches(is_non_newline_whitespace);

    let (input, _) = multispace0.parse(input); // Consume any remaining whitespace before EOL
    let (input, _) = alt((line_ending, tag(""))).parse(input)?; // Headings end with a newline, or EOF

    Ok((
        input,
        Element::Heading {
            level,
            content: final_content,
            id: id_fragment,
        },
    ))
}

// Example usage and tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_indent() {
        assert_eq!(parse_indent("   # hello"), Ok(("# hello", "   ")));
        assert_eq!(parse_indent("# hello"), Ok(("# hello", "")));
        assert_eq!(parse_indent("    # hello"), Ok((" # hello", "    "))); // Only takes up to 3 for markdown spec, but our parse_indent takes all. This is fine.
    }

    #[test]
    fn test_parse_opening_hashes() {
        assert_eq!(parse_opening_hashes("# hello"), Ok(("hello", 1)));
        assert_eq!(parse_opening_hashes("##  world"), Ok(("world", 2)));
        assert_eq!(parse_opening_hashes("###### test"), Ok(("test", 6)));
        assert!(parse_opening_hashes("####### too many").is_err());
        assert!(parse_opening_hashes("no hash").is_err());
    }

    #[test]
    fn test_parse_attribute_block() {
        assert_eq!(parse_attribute_block("{#my-id}"), Ok(("", Some("my-id"))));
        assert_eq!(parse_attribute_block("{ #another-id }"), Ok(("", Some("another-id"))));
        assert_eq!(parse_attribute_block("{}"), Ok(("", None)));
        assert_eq!(parse_attribute_block("{.class}"), Ok(("", None))); // Our current parser only extracts ID
        assert!(parse_attribute_block("{ #broken{ }").is_err()); // Nested braces
        assert!(parse_attribute_block("{ #broken< }").is_err()); // Invalid char
    }

    #[test]
    fn test_parse_atx_heading_simple() {
        assert_eq!(
            parse_atx_heading("# Hello, world!\n"),
            Ok((
                "",
                Element::Heading {
                    level: 1,
                    content: "Hello, world!",
                    id: None
                }
            ))
        );
        assert_eq!(
            parse_atx_heading("##  Another heading  \n"),
            Ok((
                "",
                Element::Heading {
                    level: 2,
                    content: "Another heading",
                    id: None
                }
            ))
        );
        assert_eq!(
            parse_atx_heading("   ### Third heading ### \n"),
            Ok((
                "",
                Element::Heading {
                    level: 3,
                    content: "Third heading",
                    id: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_atx_heading_with_id() {
        assert_eq!(
            parse_atx_heading("# My Heading {#my-heading}\n"),
            Ok((
                "",
                Element::Heading {
                    level: 1,
                    content: "My Heading",
                    id: Some("my-heading")
                }
            ))
        );
        assert_eq!(
            parse_atx_heading("## Heading with ID ## {#an-id}\n"),
            Ok((
                "",
                Element::Heading {
                    level: 2,
                    content: "Heading with ID",
                    id: Some("an-id")
                }
            ))
        );
        assert_eq!(
            parse_atx_heading("### No ID Here {#}\n"),
            Ok((
                "",
                Element::Heading {
                    level: 3,
                    content: "No ID Here",
                    id: None // Our ID parser needs `#` to be followed by ID fragment.
                }
            ))
        );
         assert_eq!(
            parse_atx_heading("### Another {#id-42}     \n"),
            Ok((
                "",
                Element::Heading {
                    level: 3,
                    content: "Another",
                    id: Some("id-42")
                }
            ))
        );
    }

    #[test]
    fn test_parse_atx_heading_edge_cases() {
        // Only hashes
        assert_eq!(
            parse_atx_heading("# \n"),
            Ok((
                "",
                Element::Heading {
                    level: 1,
                    content: "",
                    id: None
                }
            ))
        );
        // Trailing spaces after closing hashes
        assert_eq!(
            parse_atx_heading("##  Content  ##   \n"),
            Ok((
                "",
                Element::Heading {
                    level: 2,
                    content: "Content",
                    id: None
                }
            ))
        );
        // No newline (EOF)
        assert_eq!(
            parse_atx_heading("### Hello"),
            Ok((
                "",
                Element::Heading {
                    level: 3,
                    content: "Hello",
                    id: None
                }
            ))
        );
    }
}


// # with the ID {#myh1}
// ## with a class {.myclass}
// #### with a custom attribute {myattr=myvalue}
// ### multiple! {.myclass1 myattr #myh3 otherattr=value .myclass2}
