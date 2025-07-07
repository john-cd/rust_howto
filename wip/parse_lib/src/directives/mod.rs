#![allow(dead_code)]
#![allow(unused_imports)]
//! Parse various directives within text / Markdown:
//!
//! {{cat xyz}}
//! {{!cat xyz}}
//! {{#crate my_crate}}
//! {{#crate my_crate cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}
//! {{#example some_example}}
//! ...
//!
//! See `directive_model.rs`.

use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::tag;
use nom::bytes::take_till;
use nom::bytes::take_until;
use nom::character::char;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::value;
use nom::combinator::verify;
use nom::multi::separated_list1;
use nom::sequence::delimited;

// TODO
// use nom::error::ParseError;
// use nom::Input;
// use nom::AsChar;
//
// pub fn wrap<'a, I, E: ParseError<I>, F>(
//     parser: F,
// ) -> impl Parser<I, Output = <F as Parser<I>>::Output, Error = E>
// where
//     I: Input + nom::Compare<&'a str>,
//     <I as Input>::Item: AsChar,
//     F: Parser<I, Error = E>,
//     {
//         delimited((tag("{{"), space0),
//         parser,
//         (space0, tag("}}")),
//         )
//     }

/// Matches the fixed prefix "{{" and optional spaces.
fn parse_prefix(input: &str) -> IResult<&str, ()> {
    let (input, _) = (tag("{{"), space0).parse(input)?;
    Ok((input, ()))
}

/// Parses the keyword part of the directive (e.g., "crate", "docs").
/// It uses `alt` to try matching against a list of known keywords.
fn parse_kinds(input: &str) -> IResult<&str, &str> {
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

/// Matches an optional colon, optionally preceded by spaces.
fn parse_optional_colon(input: &str) -> IResult<&str, ()> {
    value((), opt((space0, char(':')))).parse(input)
}

/// Parses text before }} after at least one space.
/// Returns trimmed text. Errors if it is empty.
fn parse_value(input: &str) -> IResult<&str, &str> {
    verify(
        map(
            delimited(
                space1::<&str, _>, // Matches at least one space.
                take_until("}}"),  // Takes all characters until the "}}" sequence is found.
                tag("}}"),         // Consumes the closing "}}".
            ),
            |value| value.trim(),
        ),
        |s: &str| !s.is_empty(), // `value` (after trim) can't be empty.
    )
    .parse(input)
}

/// Parses text before }} after at least one space.
/// Split the text at whitespaces.
fn parse_values(input: &str) -> IResult<&str, Vec<&str>> {
    let word = take_till(|c: char| c.is_whitespace() || c == '}');
    delimited(
        space1,                        // Starts with a space.
        separated_list1(space0, word), // List of at least one word separated by space or tab.
        tag("}}"),                     // Ends with "}}".
    )
    .parse(input)
}

use directive_lib::DestinationKind;

/// Convert keywords to `DestinationKind` enum.
fn to_destination_kind(input: &str) -> DestinationKind {
    use directive_lib::DestinationKind::*;
    match input {
        "cat" => Category,
        "crate" => Crate,
        "docs" => Docs,
        "github" => GithubRepo,
        "lib.rs" => LibRs,
        "crates.io" => CratesIo,
        "web" => Web,
        _ => todo!(), // TODO
    }
}

use directive_lib::Directive;

/// Parses a complete directive line, extracting the directive type () / kind and its value (e.g., crate name).
///
/// The expected structure is:
/// "{{" + zero or more spaces + Sigil (# or ! or nothing) + DestinationKind + (zero or more spaces + :) + 1 or more spaces + Value(s) + zero or more spaces + "}}"
///
/// The `value` part can contain spaces and can be optionally followed
/// by whitespace before the final "}}",
pub fn parse_directive(input: &str) -> IResult<&str, Directive> {
    let (input, _) = parse_prefix(input)?;

    let insert_link = (parse_kinds, parse_optional_colon, parse_value);

    let insert_badge = (
        char('!'),
        space0,
        parse_kinds,
        parse_optional_colon,
        parse_value,
    );

    let insert_crate_block = (
        char('#'),
        space0,
        tag("crate"),
        parse_optional_colon,
        parse_values,
    );

    let insert_example_block_prefix = (
        char('#'),
        space0,
        tag("example"),
        parse_optional_colon,
        parse_value,
    );

    let mut directives = alt((
        map(insert_link, |(kind, _, value)| Directive::Link {
            kind: to_destination_kind(kind),
            name: value,
        }),
        map(insert_badge, |(_, _, kind, _, value)| Directive::Badge {
            kind: to_destination_kind(kind),
            name: value,
        }),
        map(insert_crate_block, |(_, _, _, _, value)| {
            Directive::CrateBlock {
                crate_name: value
                    .get(0)
                    .expect("There must be at least one word because of `separated_list1`."),
                additional_categories: value.get(1..).unwrap_or(&[]).to_vec(),
            }
        }),
        map(insert_example_block_prefix, |(_, _, _, _, value)| {
            Directive::ExampleBlock { name: value }
        }),
    ));

    directives.parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // * Category links:
    //
    // {{cat xyz}}
    //
    // Variants:
    //
    // {{cat xyz  }}
    // {{  cat xyz}}
    // {{cat: xyz}}
    // {{cat  :  xyz}}
    // {{cat x-y_z::a-b_c }}
    #[test]
    fn test_parse_directive_category_link() {
        let examples = [
            "{{cat xyz}}",
            "{{cat xyz  }}",
            "{{  cat xyz}}",
            "{{cat: xyz}}",
            "{{cat  :  xyz}}",
        ];
        for &input in examples.iter() {
            let parsed = parse_directive(input);
            let expected = Ok((
                "",
                Directive::Link {
                    kind: DestinationKind::Category,
                    name: "xyz",
                },
            ));
            assert_eq!(parsed, expected);
        }

        let parsed = parse_directive("{{cat x-y_z::a-b_c }}");
        let expected = Ok((
            "",
            Directive::Link {
                kind: DestinationKind::Category,
                name: "x-y_z::a-b_c",
            },
        ));
        assert_eq!(parsed, expected);

        let parsed = parse_directive("{{cat}}");
        assert!(parsed.is_err());

        let parsed = parse_directive("{{cat }}");
        assert!(parsed.is_err());

        let parsed = parse_directive("{{cat_missing_space}}");
        assert!(parsed.is_err());
    }

    // ----------------------------

    // * Category badges:
    //
    // {{!cat mathematics}}
    // {{!cat mathematics }}
    // {{ !cat mathematics}}
    // {{!cat: mathematics}}
    // {{!cat : mathematics}}
    // {{!cat no_std}}
    #[test]
    fn test_parse_directive_category_badge() {
        let examples = [
            "{{!cat xyz}}",
            "{{!cat xyz  }}",
            "{{ !cat xyz}}",
            "{{!cat: xyz}}",
            "{{!cat  :  xyz}}",
        ];
        for &input in examples.iter() {
            let parsed = parse_directive(input);
            let expected = Ok((
                "",
                Directive::Badge {
                    kind: DestinationKind::Category,
                    name: "xyz",
                },
            ));
            assert_eq!(parsed, expected);
        }

        let parsed = parse_directive("{{!cat x-y_z::a-b_c }}");
        let expected = Ok((
            "",
            Directive::Badge {
                kind: DestinationKind::Category,
                name: "x-y_z::a-b_c",
            },
        ));
        assert_eq!(parsed, expected);

        let parsed = parse_directive("{{!cat}}");
        assert!(parsed.is_err());

        let parsed = parse_directive("{{!cat }}");
        assert!(parsed.is_err());

        let parsed = parse_directive("{{!cat_no_spaces}}");
        assert!(parsed.is_err());
    }

    // ----------------------------

    // * Crate Links
    //
    // - internal page: {{crate xyz}}
    // - `docs.rs` link: {{docs xyz}}
    // - Github link: {{github xyz}}
    // - `lib.rs` link: {{lib.rs xyz}}
    // - `crates.io` link: {{crates.io xyz}}
    // - Website for the crate: {{web xyz}}
    #[test]
    fn test_parse_directive_crate_links() {
        let examples = [
            "{{crate xyz}}",
            "{{docs xyz}}",
            "{{github xyz}}",
            "{{lib.rs xyz}}",
            "{{crates.io xyz}}",
            "{{web xyz}}",
        ];

        let expected_kinds = [
            DestinationKind::Crate,
            DestinationKind::Docs,
            DestinationKind::GithubRepo,
            DestinationKind::LibRs,
            DestinationKind::CratesIo,
            DestinationKind::Web,
        ];

        for (i, &input) in examples.iter().enumerate() {
            let parsed = parse_directive(input);
            let expected = Ok((
                "",
                Directive::Link {
                    kind: expected_kinds[i].clone(),
                    name: "xyz",
                },
            ));
            assert_eq!(parsed, expected);
        }

        assert!(parse_directive("{{docs}}").is_err());

        assert!(parse_directive("{{lib.rs }}").is_err());

        assert!(parse_directive("{{crates.io_missing_space}}").is_err());
    }

    // ----------------------------

    // * Crate badges:
    // Internal crate page: {{!crate xyz}}
    //   - {{!crate xyz}}
    //   - {{!crate xyz }}
    //   - {{ ! crate xyz }}
    //   - {{!crate: x_y-z}}
    //   - {{!crate : x_y-z}}
    // {{!docs xyz}}
    // {{!github xyz}}
    // {{!lib.rs xyz}}
    // {{!crates.io xyz}}
    // {{!web xyz}}
    #[test]
    fn test_parse_directive_crate_badges() {
        let examples = [
            "{{!crate xyz}}",
            "{{!docs xyz}}",
            "{{!github xyz}}",
            "{{!lib.rs xyz}}",
            "{{!crates.io xyz}}",
            "{{!web xyz}}",
        ];

        let expected_kinds = [
            DestinationKind::Crate,
            DestinationKind::Docs,
            DestinationKind::GithubRepo,
            DestinationKind::LibRs,
            DestinationKind::CratesIo,
            DestinationKind::Web,
        ];

        for (i, &input) in examples.iter().enumerate() {
            let parsed = parse_directive(input);
            let expected = Ok((
                "",
                Directive::Badge {
                    kind: expected_kinds[i].clone(),
                    name: "xyz",
                },
            ));
            assert_eq!(parsed, expected);
        }

        assert!(parse_directive("{{!crate}}").is_err());

        assert!(parse_directive("{{!crate }}").is_err());

        assert!(parse_directive("{{!crate_missing_space}}").is_err());
    }

    // ----------------------------

    // * Crate blocks
    //
    // The following variations should be accepted:
    //
    // {{#crate crt}}
    // {{ # crate crt}}
    // {{#crate crt }}
    // {{#crate: crt}}
    // {{#crate : crt}}
    // {{#crate x_y-z}}
    //
    // * Crate Blocks with Additional Categories
    //
    // {{#crate: crt cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}

    #[test]
    fn test_parse_directive_crate_block() {
        let examples = [
            "{{#crate crt}}",
            "{{ # crate crt}}",
            "{{#crate crt }}",
            "{{#crate: crt}}",
            "{{#crate : crt}}",
        ];
        for &input in examples.iter() {
            let parsed = parse_directive(input);
            let expected = Ok((
                "",
                Directive::CrateBlock {
                    crate_name: "crt",
                    additional_categories: vec![],
                },
            ));
            assert_eq!(parsed, expected);
        }

        let parsed = parse_directive("{{#crate x_y-z}}");
        let expected = Ok((
            "",
            Directive::CrateBlock {
                crate_name: "x_y-z",
                additional_categories: vec![],
            },
        ));
        assert_eq!(parsed, expected);

        let parsed = parse_directive("{{#crate: crt cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}");
        let expected = Ok((
            "",
            Directive::CrateBlock {
                crate_name: "crt",
                additional_categories: vec!["cat1", "cat-2", "cat-2-2", "cat3::sub-cat-3"],
            },
        ));
        assert_eq!(parsed, expected);

        let parsed = parse_directive("{{#crate}}");
        assert!(parsed.is_err());

        let parsed = parse_directive("{{#crate }}");
        assert!(parsed.is_err());

        let parsed = parse_directive("{{#crate_no_spaces}}");
        assert!(parsed.is_err());
    }

    // ----------------------------

    // Example blocks:
    //
    // The following variations should be accepted:
    //
    // {{#example some_example}}
    // {{#example some_example }}
    // {{# example some_example}}
    // {{ #example some_example}}
    //
    // The following are incomplete directives:
    //
    // {{#example}}
    // {{#example }}
    // {{#example_no_space}}

    #[test]
    fn test_parse_directive_example_block() {
        let examples = [
            "{{#example some_example}}",
            "{{#example some_example }}",
            "{{# example some_example}}",
            "{{ #example some_example}}",
        ];
        for &input in examples.iter() {
            let parsed = parse_directive(input);
            let expected = Ok((
                "",
                Directive::ExampleBlock {
                    name: "some_example",
                },
            ));
            assert_eq!(parsed, expected);
        }

        assert!(parse_directive("{{#example}}").is_err());

        assert!(parse_directive("{{#example }}").is_err());

        assert!(parse_directive("{{#example_no_space}}").is_err());
    }
}
