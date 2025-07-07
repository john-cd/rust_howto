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

use winnow::prelude::*;
use winnow::Parser;
use winnow::Result;
use winnow::ascii::space0;
use winnow::ascii::space1;
use winnow::combinator::alt;
use winnow::combinator::delimited;
use winnow::combinator::opt;
use winnow::combinator::separated;
use winnow::token::literal;
use winnow::token::one_of;
use winnow::token::take_till;
use winnow::token::take_until;

// TODO
// use winnow::error::ParseError;
// use winnow::Input;
// use winnow::AsChar;
//
// pub fn wrap<'a, I, E: ParseError<I>, F>(
//     parser: F,
// ) -> impl Parser<I, Output = <F as Parser<I>>::Output, Error = E>
// where
//     I: Input + winnow::Compare<&'a str>,
//     <I as Input>::Item: AsChar,
//     F: Parser<I, Error = E>,
//     {
//         delimited(("{{", space0),
//         parser,
//         (space0, "}}"),
//         )
//     }

/// Matches the fixed prefix "{{" and optional spaces.
fn parse_prefix<'s>(input: &mut &'s str) -> Result<()> {
    ("{{", space0).value(()).parse_next(input)
}

/// Parses the keyword part of the directive (e.g., "crate", "docs").
/// It uses `alt` to try matching against a list of known keywords.
fn parse_kinds<'s>(input: &mut &'s str) -> Result<&'s str> {
    alt((
        "cat",
        "crate",
        "docs",
        "github",
        "lib.rs",
        "crates.io",
        "web",
    ))
    .parse_next(input)
}

/// Matches an optional colon, optionally preceded by spaces.
fn parse_optional_colon<'s>(input: &mut &'s str) -> Result<()> {
    opt((space0, ":")).value(()).parse_next(input)
}

/// Parses text before }} after at least one space.
/// Returns trimmed text. Errors if it is empty.
fn parse_value<'s>(input: &mut &'s str) -> Result<&'s str> {
    delimited(
        space1,                // Matches at least one space.
        take_until(1.., "}}"), // Takes all characters until the "}}" sequence is found.
        "}}",         // Consumes the closing "}}".
    )
    .map(|value: &str| value.trim())
    .verify(|s: &str| !s.is_empty()) // `value` (after trim) can't be empty.
    .parse_next(input)
}

/// Parses text before }} after at least one space.
/// Split the text at whitespaces.
fn parse_values<'s>(input: &mut &'s str) -> Result<Vec<&'s str>> {
    let word = take_till(1.., |c: char| c.is_whitespace() || c == '}');
    delimited(
        space1,                       // Starts with a space.
        separated(1.., word, space0), // List of at least one word separated by space or tab.
        "}}",                // Ends with "}}".
    )
    .parse_next(input)
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
pub fn parse_directive<'s>(input: &mut &'s str) -> Result<Directive<'s>> {
    let insert_link = (parse_prefix, parse_kinds, parse_optional_colon, parse_value);

    let insert_badge = (
        parse_prefix,
        "!",
        space0,
        parse_kinds,
        parse_optional_colon,
        parse_value,
    );

    let insert_crate_block = (
        parse_prefix,
        "#",
        space0,
        "crate",
        parse_optional_colon,
        parse_values,
    );

    let insert_example_block_prefix = (
        parse_prefix,
        "#",
        space0,
        "example",
        parse_optional_colon,
        parse_value,
    );

    let mut directives = alt((
        insert_link.map(|(_, kind, _, value)| Directive::Link {
            kind: to_destination_kind(kind),
            name: value,
        }),
        insert_badge.map(|(_, _, _, kind, _, value)| Directive::Badge {
            kind: to_destination_kind(kind),
            name: value,
        }),
        insert_crate_block.map(|(_, _, _, _, _, value)| Directive::CrateBlock {
            crate_name: value
                .get(0)
                .expect("There must be at least one word because of `separated_list1`."),
            additional_categories: value.get(1..).unwrap_or(&[]).to_vec(),
        }),
        insert_example_block_prefix
            .map(|(_, _, _, _, _, value)| Directive::ExampleBlock { name: value }),
    ));

    directives.parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO fix failing unit tests below
    // fn insert_crate_block<'s>(input: &mut &'s str) -> Result< (char, &'s str, &'s str, (), Vec<&'s str>)> {
    //     (
    //         "#",
    //         space0,
    //         "crate",
    //         parse_optional_colon,
    //         parse_values,
    //     )
    //         .parse_next(input)
    // }
    //
    // #[test]
    // fn test() {
    //     assert_eq!(
    //         insert_crate_block.parse_peek("{{#crate xyz}}"),
    //         Ok(("", ('#', "", "crate", (), vec!["xyz"])))
    //     );
    // }

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
        for input in examples.into_iter() {
            let parsed = parse_directive.parse_peek(&mut input);
            let expected = Ok((
                "",
                Directive::Link {
                    kind: DestinationKind::Category,
                    name: "xyz",
                },
            ));
            assert_eq!(parsed, expected);
        }

        let parsed = parse_directive.parse_peek(&mut "{{cat x-y_z::a-b_c }}");
        let expected = Ok((
            "",
            Directive::Link {
                kind: DestinationKind::Category,
                name: "x-y_z::a-b_c",
            },
        ));
        assert_eq!(parsed, expected);

        let parsed = parse_directive(&mut "{{cat}}");
        assert!(parsed.is_err());

        let parsed = parse_directive(&mut "{{cat }}");
        assert!(parsed.is_err());

        let parsed = parse_directive(&mut "{{cat_missing_space}}");
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
        for mut input in examples.into_iter() {
            let parsed = parse_directive(&mut input);
            let expected = Ok(Directive::Badge {
                kind: DestinationKind::Category,
                name: "xyz",
            });
            assert_eq!(parsed, expected);
        }

        let parsed = parse_directive(&mut "{{!cat x-y_z::a-b_c }}");
        let expected = Ok(Directive::Badge {
            kind: DestinationKind::Category,
            name: "x-y_z::a-b_c",
        });
        assert_eq!(parsed, expected);

        let parsed = parse_directive(&mut "{{!cat}}");
        assert!(parsed.is_err());

        let parsed = parse_directive(&mut "{{!cat }}");
        assert!(parsed.is_err());

        let parsed = parse_directive(&mut "{{!cat_no_spaces}}");
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

        for (i, mut input) in examples.into_iter().enumerate() {
            let parsed = parse_directive(&mut input);
            let expected = Ok(Directive::Link {
                kind: expected_kinds[i].clone(),
                name: "xyz",
            });
            assert_eq!(parsed, expected);
        }

        assert!(parse_directive(&mut "{{docs}}").is_err());

        assert!(parse_directive(&mut "{{lib.rs }}").is_err());

        assert!(parse_directive(&mut "{{crates.io_missing_space}}").is_err());
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

        for (i, mut input) in examples.into_iter().enumerate() {
            let parsed = parse_directive(&mut input);
            let expected = Ok(Directive::Badge {
                kind: expected_kinds[i].clone(),
                name: "xyz",
            });
            assert_eq!(parsed, expected);
        }

        assert!(parse_directive(&mut "{{!crate}}").is_err());

        assert!(parse_directive(&mut "{{!crate }}").is_err());

        assert!(parse_directive(&mut "{{!crate_missing_space}}").is_err());
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
        for mut input in examples.into_iter() {
            let parsed = parse_directive(&mut input);
            let expected = Ok(Directive::CrateBlock {
                crate_name: "crt",
                additional_categories: vec![],
            });
            assert_eq!(parsed, expected);
        }

        let parsed = parse_directive(&mut "{{#crate x_y-z}}");
        let expected = Ok(Directive::CrateBlock {
            crate_name: "x_y-z",
            additional_categories: vec![],
        });
        assert_eq!(parsed, expected);

        let parsed = parse_directive(&mut "{{#crate: crt cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}");
        let expected = Ok(Directive::CrateBlock {
            crate_name: "crt",
            additional_categories: vec!["cat1", "cat-2", "cat-2-2", "cat3::sub-cat-3"],
        });
        assert_eq!(parsed, expected);

        let parsed = parse_directive(&mut "{{#crate}}");
        assert!(parsed.is_err());

        let parsed = parse_directive(&mut "{{#crate }}");
        assert!(parsed.is_err());

        let parsed = parse_directive(&mut "{{#crate_no_spaces}}");
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
        for mut input in examples.into_iter() {
            let parsed = parse_directive(&mut input);
            let expected = Ok(Directive::ExampleBlock {
                name: "some_example",
            });
            assert_eq!(parsed, expected);
        }

        assert!(parse_directive(&mut "{{#example}}").is_err());

        assert!(parse_directive(&mut "{{#example }}").is_err());

        assert!(parse_directive(&mut "{{#example_no_space}}").is_err());
    }
}
