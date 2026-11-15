//! Runs one of the `bitfield` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// `bitflags` generate types for C-style flags with ergonomic APIs.
mod bitflags;

/// Add the following to your `Cargo.toml`: ```toml [dependencies] bitvec = "1.1.1" ```
mod bitvec;

/// A fixed-size bit set with `fixedbitset`: a visited set for a graph walk.
mod fixedbitset;

/// Example of using the `flagset` crate to define and use bit flags.
mod flagset;

/// Compressed bitmaps with the `roaring` crate.
mod roaring;

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
    /// Runs the `bitflags` example.
    #[command(name = "bitflags")]
    Bitflags,

    /// Runs the `bitvec` example.
    #[command(name = "bitvec")]
    Bitvec,

    /// Runs the `flagset` example.
    #[command(name = "flagset")]
    Flagset,

    /// Runs the `roaring` example.
    #[command(name = "roaring")]
    Roaring,

    /// Runs the `fixedbitset` example.
    #[command(name = "fixedbitset")]
    Fixedbitset,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::Bitflags => {
                bitflags::run();
            }
            Commands::Bitvec => {
                bitvec::run();
            }
            Commands::Flagset => {
                flagset::run();
            }
            Commands::Roaring => {
                roaring::run();
            }
            Commands::Fixedbitset => {
                fixedbitset::run();
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
    const SUBCOMMANDS: &[&str] = &["bitflags", "bitvec", "flagset", "roaring", "fixedbitset"];

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
