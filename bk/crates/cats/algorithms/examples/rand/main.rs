//! Runs one of the `rand` examples, chosen on the command line.
//!
//! Reviewed 10-27-2026.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Fast pseudo-random number generation with `fastrand`.
mod fastrand;

/// In `Cargo.toml`, add: ```toml [dependencies] rand = "0.10.3" or latest.
mod rand1;

/// Generate a random string of 20 alphanumeric characters.
mod rand_alphanumeric;

/// Demonstrates how to implement a custom distribution for the `rand` crate.
mod rand_custom;

/// This example demonstrates how to sample from a normal distribution.
mod rand_distr;

/// Generates a random password from a custom set of characters with `rand`.
mod rand_password;

/// Demonstrates generating random numbers within a specified range.
mod rand_range;

/// Generates random numbers in ranges with the `rand` crate.
mod rand_range_uniform;

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

    /// Runs the `fastrand` example.
    #[command(name = "fastrand")]
    Fastrand,

    /// Runs the `rand1` example.
    #[command(name = "rand")]
    Rand,

    /// Runs the `rand_alphanumeric` example.
    #[command(name = "rand_alphanumeric")]
    RandAlphanumeric,

    /// Runs the `rand_custom` example.
    #[command(name = "rand_custom")]
    RandCustom,

    /// Runs the `rand_distr` example.
    #[command(name = "rand_distr")]
    RandDistr,

    /// Runs the `rand_password` example.
    #[command(name = "rand_password")]
    RandPassword,

    /// Runs the `rand_range` example.
    #[command(name = "rand_range")]
    RandRange,

    /// Runs the `rand_range_uniform` example.
    #[command(name = "rand_range_uniform")]
    RandRangeUniform,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(command) = cli.command {
        match command {
            Commands::Fastrand => {
                fastrand::run();
            }
            Commands::Rand => {
                rand1::run();
            }
            Commands::RandAlphanumeric => {
                rand_alphanumeric::run();
            }
            Commands::RandCustom => {
                rand_custom::run();
            }
            Commands::RandDistr => {
                rand_distr::run()?;
            }
            Commands::RandPassword => {
                rand_password::run();
            }
            Commands::RandRange => {
                rand_range::run();
            }
            Commands::RandRangeUniform => {
                rand_range_uniform::run();
            }
        }
    }

    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "fastrand",
        "rand",
        "rand_alphanumeric",
        "rand_custom",
        "rand_distr",
        "rand_password",
        "rand_range",
        "rand_range_uniform",
    ];

    /// Confirms that the command and its arguments are valid.
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
}
