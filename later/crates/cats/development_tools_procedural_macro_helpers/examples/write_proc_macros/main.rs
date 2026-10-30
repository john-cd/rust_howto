//! Demonstrates how to use `darling`, `proc-macro2`, `syn`, and `quote`
//! outside of a `proc-macro` crate.
//!
//! This example parses a simple struct syntax tree, generates a `Debug`
//! implementation, and prints the resulting token stream.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod darling;
mod paste;
mod proc_macro2;
mod quote;
mod syn;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Commands,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `darling` example.
    #[command(name = "darling")]
    Darling,
    /// Runs the `paste` example.
    #[command(name = "paste")]
    Paste,
    /// Runs the `proc_macro2` example.
    #[command(name = "proc_macro2")]
    ProcMacro2,
    /// Runs the `quote` example.
    #[command(name = "quote")]
    Quote,
    /// Runs the `syn` example.
    #[command(name = "syn")]
    Syn,
}

/// Runs the example selected on the command line.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dispatch(Cli::parse())
}

/// Runs the example selected in the parsed command line.
fn dispatch(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Darling => {
            darling::run()?;
        }
        Commands::Paste => {
            paste::run();
        }
        Commands::ProcMacro2 => {
            proc_macro2::run()?;
        }
        Commands::Quote => {
            quote::run()?;
        }
        Commands::Syn => {
            syn::run()?;
        }
    }
    Ok(())
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

    /// A subcommand is required.
    #[test]
    fn missing_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example"]).is_err());
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
