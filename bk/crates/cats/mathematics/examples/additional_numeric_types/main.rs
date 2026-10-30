//! Runs one of the `additional_numeric_types` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

use clap::Parser;
use clap::Subcommand;

mod big_integers;
mod fixed_point;
mod half_precision;
mod num_bigint;
mod num_traits;
mod ordered_float;
mod rational_numbers;
#[cfg(target_os = "linux")]
mod rug;
mod rust_decimal;
mod typenum;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `big_integers` example.
    #[command(name = "big_integers")]
    BigIntegers,
    /// Runs the `fixed_point` example.
    #[command(name = "fixed_point")]
    FixedPoint,
    /// Runs the `half_precision` example.
    #[command(name = "half_precision")]
    HalfPrecision,
    /// Runs the `num_bigint` example.
    #[command(name = "num_bigint")]
    NumBigint,
    /// Runs the `num_traits` example.
    #[command(name = "num_traits")]
    NumTraits,
    /// Runs the `ordered_float` example.
    #[command(name = "ordered_float")]
    OrderedFloat,
    /// Runs the `rational_numbers` example.
    #[command(name = "rational_numbers")]
    RationalNumbers,
    /// Runs the `rug` example.
    #[cfg(target_os = "linux")]
    #[command(name = "rug")]
    Rug,
    /// Runs the `rust_decimal` example.
    #[command(name = "rust_decimal")]
    RustDecimal,
    /// Runs the `typenum` example.
    #[command(name = "typenum")]
    Typenum,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::BigIntegers => {
                big_integers::run();
            }
            Commands::FixedPoint => {
                fixed_point::run();
            }
            Commands::HalfPrecision => {
                half_precision::run();
            }
            Commands::NumBigint => {
                num_bigint::run();
            }
            Commands::NumTraits => {
                num_traits::run();
            }
            Commands::OrderedFloat => {
                ordered_float::run();
            }
            Commands::RationalNumbers => {
                rational_numbers::run();
            }
            #[cfg(target_os = "linux")]
            Commands::Rug => {
                rug::run();
            }
            Commands::RustDecimal => {
                rust_decimal::run();
            }
            Commands::Typenum => {
                typenum::run();
            }
        }
    }
}

/// Parses the command line and runs the selected example.
fn main() {
    dispatch(Cli::parse());
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "big_integers",
        "fixed_point",
        "half_precision",
        "num_bigint",
        "num_traits",
        "ordered_float",
        "rational_numbers",
        #[cfg(target_os = "linux")]
        "rug",
        "rust_decimal",
        "typenum",
    ];

    /// The command-line definition is internally consistent.
    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() {
        for name in SUBCOMMANDS {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
        }
    }

    /// Subcommand names are unique.
    #[test]
    fn test_subcommand_names_are_unique() {
        let mut sorted = SUBCOMMANDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SUBCOMMANDS.len());
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }

    /// Running without a subcommand selects nothing and succeeds.
    #[test]
    fn test_no_subcommand_runs_nothing() {
        let cli = Cli::try_parse_from(["example"])
            .expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli);
    }
}
