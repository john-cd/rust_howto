//! Runs one of the `fuzzing` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// `afl` (American Fuzzy Lop) is a popular fuzz testing tool, and `afl` is a Rust crate that
/// provides support for integrating AFL with Rust programs.
#[cfg(all(not(windows), any(fuzzing, test)))]
mod afl;

/// `bolero` is a fuzzing frontend for Rust that supports multiple engines like libFuzzer, AFL, and
/// Honggfuzz.
mod bolero_example;

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
    /// Runs the `afl` example.
    #[cfg(all(not(windows), fuzzing))]
    #[command(name = "afl")]
    Afl,

    /// Runs the `bolero_example` example.
    #[command(name = "bolero_example")]
    BoleroExample,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            #[cfg(all(not(windows), fuzzing))]
            Commands::Afl => {
                afl::run();
            }
            Commands::BoleroExample => {
                bolero_example::run();
            }
        }
    }
}

/// Parses the command line and runs the selected example.
fn main() {
    dispatch(Cli::parse());
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Both example subcommands are available in AFL-instrumented builds.
    #[cfg(all(not(windows), fuzzing))]
    const SUBCOMMANDS: &[&str] = &["afl", "bolero_example"];

    /// Ordinary builds expose the portable example.
    #[cfg(not(all(not(windows), fuzzing)))]
    const SUBCOMMANDS: &[&str] = &["bolero_example"];

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
