#![allow(dead_code)]

// Import necessary nom parser combinators
use nom::Parser;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::space1,
    combinator::map,
};

//
#[derive(Debug, PartialEq)]
pub enum DirectiveType {
    // TODO
    /// A single Markdown link e.g., `[text][label]`⮳.
    Link,
    /// Badge e.g., `[![some_crate][c~some_crate~docs~badge]][c~some_crate~docs]`.
    Badge,
    /// Block e.g., multiple badges for a given crate.
    Block,
}

/// Represents the kind of directive being parsed.
#[derive(Debug, PartialEq)]
pub enum DirectiveKind {
    Crate,
    Docs,
    Github,
    LibRs,
    CratesIo,
    Web,
    Example, // TODO
}

/// Represents a parsed directive, containing its type, kind, and the extracted value.
#[derive(Debug, PartialEq)]
pub struct ParsedDirective<'a> {
    pub directive_type: DirectiveType,
    pub directive_kind: DirectiveKind,
    pub value: &'a str,
}

/// Parses the keyword part of the link (e.g., "crate", "docs").
/// It uses `alt` to try matching against a list of known keywords.
fn parse_directive_kind(input: &str) -> IResult<&str, DirectiveKind> {
    alt((
        map(tag("crate"), |_| DirectiveKind::Crate),
        map(tag("docs"), |_| DirectiveKind::Docs),
        map(tag("github"), |_| DirectiveKind::Github),
        map(tag("lib.rs"), |_| DirectiveKind::LibRs),
        map(tag("crates.io"), |_| DirectiveKind::CratesIo),
        map(tag("web"), |_| DirectiveKind::Web),
    ))
    .parse(input)
}

//
fn parse_directive_type(input: &str) -> IResult<&str, DirectiveType> {
    alt((
        map(tag("!"), |_| DirectiveType::Link),
        map(tag("#"), |_| DirectiveType::Block),
    ))
    .parse(input)
}

/// Parses a complete link line, extracting the link type and its crate.
///
/// The structure expected is:
/// "{{" + DirectiveType + DirectiveKind + 1 or more spaces + Value + zero or more spaces + "}}"
///
/// The `value` part can contain spaces and can be optionally followed
/// by whitespace before the final "}}". This parser extracts the raw content
/// between the keyword and "}}" and then trims it.
fn parse_link_line(input: &str) -> IResult<&str, ParsedDirective> {
    map(
        (
            tag("{{"),            // Matches the fixed prefix "{{"
            parse_directive_type, // Parses the type sigil # or ! or nothing
            parse_directive_kind, // Parses the keyword: docs, github, etc.
            space1,               // Matches at least one space after the keyword
            take_until("}}"),     // Takes all characters until the "}}" sequence is found
            tag("}}"),            // Consumes the closing "}}"
        ),
        // This closure is executed on successful parsing to transform the raw output
        // into our `ParsedDirective` struct.
        |(_, directive_type, directive_kind, _, value, _): (
            &str,
            DirectiveType,
            DirectiveKind,
            &str,
            &str,
            &str,
        )| {
            let trimmed_value = value.trim();

            ParsedDirective {
                directive_type,
                directive_kind,
                value: trimmed_value,
            }
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_link_line() {
        let examples = [
            "{{!crate my_awesome_crate}}",
            "{{!docs my_docs_link  }}", // Example with trailing whitespace
            "{{!github  my/repo }}",    // Example with leading and trailing whitespace
            "{{!lib.rs another_lib_rs_link}}",
            "{{!crates.io some_crate_name}}",
            "{{!web project_website.com}}",
            "{{!crate empty_value}}", // Example with a value that is just "empty_value"
            "{{!crate   }}",          // Example with an empty value (after trimming)
        ];

        for (i, &input) in examples.iter().enumerate() {
            println!("--- Example {} ---", i + 1);
            println!("Input: \"{input}\"");
            match parse_link_line(input) {
                Ok((remaining, parsed_link)) => {
                    println!("Success! Parsed Link: {parsed_link:?}");
                    // In this case, `remaining` should be an empty string if the whole line was parsed.
                    if !remaining.is_empty() {
                        println!("Remaining input: \"{remaining}\"");
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing input: {e:?}");
                }
            }
            println!();
        }

        // Example of a malformed string
        println!("--- Malformed Example ---");
        let malformed_input = "{{!crate_missing_space_xyz}}";
        println!("Input: \"{malformed_input}\"");
        match parse_link_line(malformed_input) {
            Ok((_, parsed_link)) => {
                println!("Unexpected success: {parsed_link:?}");
            }
            Err(e) => {
                eprintln!("Correctly failed to parse: {e:?}");
            }
        }
    }
}

// TODO
// {{#example}}
// {{#example }}
// {{#example some_example}}
// {{#example some_example }}
// {{# example some_example}}
// {{ #example some_example}}
