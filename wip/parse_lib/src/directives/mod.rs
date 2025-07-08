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

use winnow::Parser;
use winnow::Result;
use winnow::ascii::space0;
use winnow::ascii::space1;
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::fail;
use winnow::combinator::opt;
use winnow::combinator::preceded;
use winnow::combinator::repeat;
use winnow::combinator::repeat_till;
use winnow::combinator::separated;
use winnow::combinator::seq;
use winnow::combinator::terminated;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;
use winnow::prelude::*;
use winnow::token::literal;
use winnow::token::none_of;
use winnow::token::one_of;
use winnow::token::take_till;
use winnow::token::take_until;

/// Matches the fixed prefix "{{" and optional spaces.
fn parse_prefix<'s>(input: &mut &'s str) -> Result<()> {
    terminated("{{", space0)
        .void()
        .context(Label("directive prefix"))
        .context(Expected(Description("{{ and optional whitespace")))
        .parse_next(input)
}

/// Parses the keyword part of the directive (e.g., "crate", "docs").
/// It uses `alt` to try matching against a list of known keywords.
fn parse_kinds<'s>(input: &mut &'s str) -> Result<&'s str> {
    alt((
        "cat",
        "crates.io", // Must be before "crate".
        "crate",
        "docs",
        "github",
        "lib.rs",
        "web",
        fail.context(Label("directive keyword"))
            .context(Expected(StringLiteral("cat")))
            .context(Expected(StringLiteral("crate")))
            .context(Expected(StringLiteral("docs")))
            .context(Expected(StringLiteral("github")))
            .context(Expected(StringLiteral("lib.rs")))
            .context(Expected(StringLiteral("crates.io")))
            .context(Expected(StringLiteral("web"))),
    ))
    .parse_next(input)
}

/// Matches an optional colon, optionally preceded by spaces.
fn parse_optional_colon<'s>(input: &mut &'s str) -> Result<()> {
    opt(preceded(space0, ":"))
        .void()
        .context(Label("optional colon"))
        .context(Expected(Description(
            "colon preceded by optional whitespace",
        )))
        .parse_next(input)
}

/// Parses text before }} after at least one space.
/// Returns trimmed text. Errors if it is empty.
fn parse_value<'s>(input: &mut &'s str) -> Result<&'s str> {
    delimited(
        space1,                // Matches at least one space.
        take_until(1.., "}}"), // Takes all characters until the "}}" sequence is found.
        "}}",                  // Consumes the closing "}}".
    )
    .map(|value: &str| value.trim())
    .verify(|s: &str| !s.is_empty()) // `value` (after trim) can't be empty.
    .context(Label("value"))
    .context(Expected(Description(
        "text followed by }}. Should not be empty or only whitespace.",
    )))
    .parse_next(input)
}

/// Parses text before }} after at least one space.
/// Split the text at whitespaces.
fn parse_values<'s>(input: &mut &'s str) -> Result<Vec<&'s str>> {
    let word = take_till(1.., [' ', '\t', '}'])
        .context(Label("word"))
        .context(Expected(Description(
            "characters before a space or tab or }",
        )));

    let words = separated(1.., word, alt((" ", "\t")))
        .context(Label("words"))
        .context(Expected(Description(
            "one or more words separated by a space or tab",
        )));

    delimited(
        space1, // Starts with a space.
        words,
        preceded(space0, "}}"),
    )
    .context(Label("values"))
    .context(Expected(Description(
        "whitespace-separated words, followed by }}",
    )))
    .parse_next(input)
}

use directive_lib::DestinationKind;

/// Convert keywords to `DestinationKind` enum.
fn to_destination_kind(input: &str) -> DestinationKind {
    use directive_lib::DestinationKind::*;
    match input {
        "cat" => Category,
        "crates.io" => CratesIo,
        "crate" => Crate,
        "docs" => Docs,
        "github" => GithubRepo,
        "lib.rs" => LibRs,
        "web" => Web,
        _ => unreachable!(),
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
    let insert_link = seq!(_: parse_prefix, parse_kinds, _: parse_optional_colon, parse_value);

    let insert_badge = seq!(
        _: (parse_prefix, "!", space0),
        parse_kinds,
        _: parse_optional_colon,
        parse_value,
    );

    let insert_crate_block = seq!(
        _: (parse_prefix, "#", space0, "crate", parse_optional_colon),
        parse_values,
    );

    let insert_example_block = seq!(
        _: (parse_prefix, "#", space0, "example", parse_optional_colon),
        parse_value,
    );

    let mut directives = alt((
        insert_link
            .map(|(kind, value)| Directive::Link {
                kind: to_destination_kind(kind),
                name: value,
            })
            .context(Label("link directive"))
            .context(Expected(Description(
                "{{cat|crate|docs|github|lib.rs|crates.io|web xyz}}",
            ))),
        insert_badge
            .map(|(kind, value)| Directive::Badge {
                kind: to_destination_kind(kind),
                name: value,
            })
            .context(Label("badge directive"))
            .context(Expected(Description(
                "{{ ! cat|crate|docs|github|lib.rs|crates.io|web xyz}}",
            ))),
        insert_crate_block
            .map(|(values,)| Directive::CrateBlock {
                crate_name: values
                    .get(0)
                    .expect("There must be at least one word because of `separated_list1`."),
                additional_categories: values.get(1..).unwrap_or(&[]).to_vec(),
            })
            .context(Label("crate block"))
            .context(Expected(Description("{{#crate xyz}}"))),
        insert_example_block
            .map(|(value,)| Directive::ExampleBlock { name: value })
            .context(Label("example block"))
            .context(Expected(Description("{{#example xyz}}"))),
        fail.context(Label("directive"))
            .context(Expected(Description(
                "one of a crate link, a !crate badge, a #crate block or an #example block",
            ))),
    ));

    directives.parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // // Debugging:
    // // Run tests with `--features winnow/debug` to display the whole parser trace, e.g.
    // // `cargo nextest run --features winnow/debug insert_crate_block`
    // // and write smaller test parsers e.g.
    //
    // fn insert_crate_block<'s>(input: &mut &'s str) -> Result<Vec<&'s str>> {
    //     let mut p = seq!(
    //         _: (parse_prefix, "#", space0, "crate", parse_optional_colon),
    //         parse_values,
    //     ).map(|(values,)| values);
    //
    //     p.parse_next(input)
    // }
    //
    // #[test]
    // fn test_insert_crate_block() {
    //     assert_eq!(
    //         insert_crate_block.parse_peek("{{ # crate : xyz }}"),
    //         Ok(("", vec!["xyz"])));
    // }

    // ----------------------

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
        for mut input in examples.into_iter() {
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
