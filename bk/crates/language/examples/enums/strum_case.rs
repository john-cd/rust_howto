#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Parse and print config values with `serialize_all` and
//! `ascii_case_insensitive`.
//!
//! If the same enum is also serialized with `serde`, keep the two consistent:
//! `#[serde(rename_all = "snake_case")]` matches `#[strum(serialize_all =
//! "snake_case")]`. Prefer `serde` alone when the value only ever travels
//! through `serde`; use `strum` for command-line arguments, environment
//! variables and menus.

use std::str::FromStr;

use strum::Display;
use strum::EnumString;
use strum::ParseError;

/// A log level read from a configuration file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
enum Verbosity {
    Quiet,
    Normal,
    VeryVerbose,
}

/// Round trips values and shows a parse error.
fn main() {
    // `Display` writes snake_case...
    assert_eq!(Verbosity::VeryVerbose.to_string(), "very_verbose");

    // ...and parsing accepts it in any ASCII case.
    assert_eq!(
        Verbosity::from_str("very_verbose"),
        Ok(Verbosity::VeryVerbose)
    );
    assert_eq!("QUIET".parse::<Verbosity>(), Ok(Verbosity::Quiet));
    assert_eq!("Normal".parse::<Verbosity>(), Ok(Verbosity::Normal));

    // Unknown values are an error, not a default.
    let err = "loud".parse::<Verbosity>();
    assert_eq!(err, Err(ParseError::VariantNotFound));
    println!("{}", ParseError::VariantNotFound);

    // Every variant round trips.
    for v in [Verbosity::Quiet, Verbosity::Normal, Verbosity::VeryVerbose] {
        assert_eq!(v.to_string().parse::<Verbosity>(), Ok(v));
    }
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks case-insensitive parsing and serde-compatible names.
    #[test]
    fn test_main() {
        main();
    }
}
