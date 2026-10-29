//! Runs one of the `ruby` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod greeting;
#[cfg(all(not(windows), feature = "ruby"))]
// TODO review Windows support for Ruby FFI examples
mod magnus;
#[cfg(feature = "ruby")]
mod rutie;

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
    /// Runs the `magnus` example.
    #[cfg(all(not(windows), feature = "ruby"))]
    #[command(name = "magnus")]
    Magnus,
    /// Runs the `rutie` example.
    #[cfg(feature = "ruby")]
    #[command(name = "rutie")]
    Rutie,
}

/// Runs the example selected on the command line.
fn main() {
    dispatch(Cli::parse());
}

/// Runs the example selected in the parsed command line.
fn dispatch(cli: Cli) {
    println!("{}", greeting::message());

    if let Some(command) = cli.command {
        match command {
            #[cfg(all(not(windows), feature = "ruby"))]
            Commands::Magnus => {
                magnus::run().expect("the Ruby example should run");
            }
            #[cfg(feature = "ruby")]
            Commands::Rutie => {
                rutie::run().expect("the Rutie example should run");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    /// The command-line definition is internally consistent.
    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Without a subcommand, nothing is run.
    #[test]
    fn no_subcommand_runs_nothing() {
        let cli =
            Cli::try_parse_from(["example"]).expect("no arguments are valid");
        assert!(cli.command.is_none());
        dispatch(cli);
    }

    /// Every declared subcommand name is accepted.
    #[test]
    fn declared_subcommands_parse() {
        let command = Cli::command();
        let names: Vec<_> = command
            .get_subcommands()
            .map(clap::Command::get_name)
            .collect();
        for name in names {
            assert!(
                Cli::try_parse_from(["example", name]).is_ok(),
                "subcommand `{name}` should parse"
            );
        }
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
