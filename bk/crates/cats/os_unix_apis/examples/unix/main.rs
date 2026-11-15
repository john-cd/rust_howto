//! Runs one of the `unix` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Simple Rust example using the `nix` crate to handle Unix-like operating
#[cfg(target_family = "unix")]
mod nix;
/// `rustix` is a library that provides a safe and idiomatic Rust interface to
#[cfg(target_family = "unix")]
mod rustix;

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
    /// Runs the `nix` example.
    #[cfg(target_family = "unix")]
    #[command(name = "nix")]
    Nix,
    /// Runs the `rustix` example.
    #[cfg(target_family = "unix")]
    #[command(name = "rustix")]
    Rustix,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            #[cfg(target_family = "unix")]
            Commands::Nix => {
                nix::run().expect("the example should run");
            }
            #[cfg(target_family = "unix")]
            Commands::Rustix => {
                rustix::run().expect("the example should run");
            }
        }
    }
}
/// Runs the example.
/// Parses the command line and runs the selected example.

fn main() {
    dispatch(Cli::parse());
}

/// Tests the `main` example and its documented behavior.
#[cfg(all(test, unix))]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["nix", "rustix"];

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
        let cli = Cli::try_parse_from(["example"]).expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli);
    }
}
