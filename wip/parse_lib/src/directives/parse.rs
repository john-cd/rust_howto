#![allow(dead_code)]
#![allow(unused_imports)]

use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::tag;
use nom::bytes::take_until;
use nom::character::char;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::value;

use super::model::*;

// Matches the fixed prefix "{{" and optional spaces.
fn parse_prefix(input: &str) -> IResult<&str, ()> {
    let (input, _) = (tag("{{"), space0).parse(input)?;
    Ok((input, ()))
}

//
fn optional_colon(input: &str) -> IResult<&str, ()> {
    value((), opt((space0, char(':')))).parse(input)
}

//
fn get_value(input: &str) -> IResult<&str, &str> {
    map(
        (
            space1::<&str, _>, // Matches at least one space after the keyword
            take_until("}}"),  // Takes all characters until the "}}" sequence is found
            tag("}}"),         // Consumes the closing "}}"
        ),
        |(_, value, _)| value.trim(),
    )
    .parse(input)
}

// Parses the keyword part of the directive (e.g., "crate", "docs").
// It uses `alt` to try matching against a list of known keywords.
fn kinds(input: &str) -> IResult<&str, &str> {
    alt((
        tag("cat"),
        tag("crate"),
        tag("docs"),
        tag("github"),
        tag("lib.rs"),
        tag("crates.io"),
        tag("web"),
    ))
    .parse(input)
}

// TODO
fn get_link_kind(input: &str) -> LinkKind {
    match input {
        "cat" => LinkKind::Category,
        "crate" => LinkKind::Crate,
        "docs" => LinkKind::Docs,
        "github" => LinkKind::GithubRepo,
        "lib.rs" => LinkKind::LibRs,
        "crates.io" => LinkKind::CratesIo,
        "web" => LinkKind::Web,
        _ => todo!(), // TODO
    }
}

// Parses
// - the type sigil # or ! or nothing,
// - the keyword afterwards.

/// Parses a complete directive line, extracting the directive type / kind and its value (e.g., crate name).
///
/// The expected structure is:
/// "{{" + zero or more spaces + Directive Type Sigil + DirectiveKind + (zero or more spaces + :) + 1 or more spaces + Value + zero or more spaces + "}}"
///
/// The `value` part can contain spaces and can be optionally followed
/// by whitespace before the final "}}",
fn parse_directive(input: &str) -> IResult<&str, Directive> {
    let (input, _) = parse_prefix(input)?;

    let insert_link = (kinds, optional_colon, get_value);

    let insert_badge = (char('!'), space0, kinds, optional_colon, get_value);

    let insert_crate_block = (char('#'), space0, tag("crate"), get_value);

    let insert_example_block_prefix = (char('#'), space0, tag("crate"), get_value);

    let mut directives = alt((
        map(insert_link, |(kind, _, value)| Directive::Link {
            kind: get_link_kind(kind),
            name: value,
        }),
        map(insert_badge, |(_, _, kind, _, value)| Directive::Badge {
            kind: get_link_kind(kind),
            name: value,
        }),
        map(insert_crate_block, |(_, _, _, value)| {
            Directive::CrateBlock { crate_name: value }
        }),
        map(insert_example_block_prefix, |(_, _, _, value)| {
            Directive::ExampleBlock { name: value }
        }),
    ));

    directives.parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_crate_badge_directives() {
        let examples = [
            "{{!crate my_awesome_crate}}",
            "{{!docs my_crate}}",
            "{{!github  my_crate}}",
            "{{!lib.rs another_lib_rs_link}}",
            "{{!crates.io some_crate_name}}",
            "{{!web project_website.com}}",
            // Example with trailing whitespace
            // TODO
            // Example with leading and trailing whitespace
            // TODO
            "{{!crate   }}", // Example with an empty value (after trimming)
        ];

        for (i, &input) in examples.iter().enumerate() {
            println!("--- Example {} ---", i + 1);
            println!("Input: \"{input}\"");
            match parse_directive(input) {
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
        match parse_directive(malformed_input) {
            Ok((_, parsed_link)) => {
                println!("Unexpected success: {parsed_link:?}");
            }
            Err(e) => {
                eprintln!("Correctly failed to parse: {e:?}");
            }
        }
    }
}

// TODO review mdbook-scrub test book for examples of directives to test

// {{#example}}
// {{#example }}
// {{#example some_example}}
// {{#example some_example }}
// {{# example some_example}}
// {{ #example some_example}}

// Crate Blocks

// {{#crate crt}}
// {{ # crate crt}}
// {{#crate crt }}
// {{#crate: crt}}
// {{#crate : crt}}
// {{#crate: x_y-z}}

// Crate Blocks with Additional Categories

// {{#crate: crt cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}
