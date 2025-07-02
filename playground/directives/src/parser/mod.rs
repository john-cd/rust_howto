#![allow(dead_code)]
#![allow(unused_imports)]

use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::space1;
use nom::combinator::map;

// TODO revise the AST for directives
// review mdbook-scrub test book for directive examples
#[derive(Debug, PartialEq)]
pub enum DirectiveType {
    // Directive to insert:
    /// Markdown link e.g., `[text][label]`⮳.
    Link(DirectiveKind),
    /// Badge e.g., `[![some_crate][c~some_crate~docs~badge]][c~some_crate~docs]`.
    Badge(DirectiveKind),
    /// Crate block e.g., multiple badges for a given crate: {{#crate crt}}.
    CrateBlock,
    /// Example directive: {{#example some_example}}.
    ExampleBlock,
}

/// Represents the kind of directive being parsed.
/// Link or Badge only.
#[derive(Debug, PartialEq)]
pub enum DirectiveKind {
    Category,
    Crate, // Link or badge to internal crate page.
    Docs,  // link or badge to docs.rs.
    Github,
    LibRs,
    CratesIo,
    Web,
}

// /// Represents a parsed directive, containing its type, kind, and the extracted value.
// #[derive(Debug, PartialEq)]
// pub struct ParsedDirective<'a> {
//     pub directive_type: DirectiveType,
//     pub directive_kind: DirectiveKind,
//     pub value: &'a str,
// }

// /// Parses the keyword part of the directive (e.g., "crate", "docs").
// /// It uses `alt` to try matching against a list of known keywords.
// fn parse_directive_kind(input: &str) -> IResult<&str, DirectiveKind> {
//     alt((
//         map(tag("crate"), |_| DirectiveKind::Crate),
//         map(tag("docs"), |_| DirectiveKind::Docs),
//         map(tag("github"), |_| DirectiveKind::Github),
//         map(tag("lib.rs"), |_| DirectiveKind::LibRs),
//         map(tag("crates.io"), |_| DirectiveKind::CratesIo),
//         map(tag("web"), |_| DirectiveKind::Web),
//     ))
//     .parse(input)
// }

// //
// fn parse_directive_type(input: &str) -> IResult<&str, DirectiveType> {
//     alt((
//         map(tag("!"), |_| DirectiveType::Link),
//         map(tag("#"), |_| DirectiveType::CrateBlock),
//     ))
//     .parse(input)
// }

// /// Parses a complete directive line, extracting the directive type / kind and its value (e.g., crate name).
// ///
// /// The expected structure is:
// /// "{{" + zero or more spaces + DirectiveType + DirectiveKind + (zero or more spaces + :) + 1 or more spaces + Value + zero or more spaces + "}}"
// ///
// /// The `value` part can contain spaces and can be optionally followed
// /// by whitespace before the final "}}",
// fn parse_link_line(input: &str) -> IResult<&str, ParsedDirective> {
//     map(
//         (
//             tag("{{"),            // Matches the fixed prefix "{{"
//             parse_directive_type, // Parses the type sigil # or ! or nothing
//             parse_directive_kind, // Parses the keyword: docs, github, etc.
//             space1,               // Matches at least one space after the keyword
//             take_until("}}"),     // Takes all characters until the "}}" sequence is found
//             tag("}}"),            // Consumes the closing "}}"
//         ),
//         // This closure is executed on successful parsing to transform the raw output
//         // into our `ParsedDirective` struct.
//         |(_, directive_type, directive_kind, _, value, _): (
//             &str,
//             DirectiveType,
//             DirectiveKind,
//             &str,
//             &str,
//             &str,
//         )| {
//             let trimmed_value = value.trim();

//             ParsedDirective {
//                 directive_type,
//                 directive_kind,
//                 value: trimmed_value,
//             }
//         },
//     )
//     .parse(input)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_crate_badge_directives() {
//         let examples = [
//             "{{!crate my_awesome_crate}}",
//             "{{!docs my_crate}}",
//             "{{!github  my_crate}}",
//             "{{!lib.rs another_lib_rs_link}}",
//             "{{!crates.io some_crate_name}}",
//             "{{!web project_website.com}}",
//             // Example with trailing whitespace
//             // TODO
//             // Example with leading and trailing whitespace
//             // TODO
//             "{{!crate   }}",          // Example with an empty value (after trimming)
//         ];

//         for (i, &input) in examples.iter().enumerate() {
//             println!("--- Example {} ---", i + 1);
//             println!("Input: \"{input}\"");
//             match parse_link_line(input) {
//                 Ok((remaining, parsed_link)) => {
//                     println!("Success! Parsed Link: {parsed_link:?}");
//                     // In this case, `remaining` should be an empty string if the whole line was parsed.
//                     if !remaining.is_empty() {
//                         println!("Remaining input: \"{remaining}\"");
//                     }
//                 }
//                 Err(e) => {
//                     eprintln!("Error parsing input: {e:?}");
//                 }
//             }
//             println!();
//         }

//         // Example of a malformed string
//         println!("--- Malformed Example ---");
//         let malformed_input = "{{!crate_missing_space_xyz}}";
//         println!("Input: \"{malformed_input}\"");
//         match parse_link_line(malformed_input) {
//             Ok((_, parsed_link)) => {
//                 println!("Unexpected success: {parsed_link:?}");
//             }
//             Err(e) => {
//                 eprintln!("Correctly failed to parse: {e:?}");
//             }
//         }
//     }
// }

// // TODO review mdbook-scrub test book for examples of directives to test

// // {{#example}}
// // {{#example }}
// // {{#example some_example}}
// // {{#example some_example }}
// // {{# example some_example}}
// // {{ #example some_example}}

// // Crate Blocks

// // {{#crate crt}}
// // {{ # crate crt}}
// // {{#crate crt }}
// // {{#crate: crt}}
// // {{#crate : crt}}
// // {{#crate: x_y-z}}

// // Crate Blocks with Additional Categories

// // {{#crate: crt cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}
