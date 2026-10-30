//! Runs one of the `retained_mode_gui` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#![allow(unexpected_cfgs)]

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "floem")]
mod floem;
#[cfg(feature = "iced")]
mod iced;
#[cfg(feature = "slint")]
mod slint;
#[cfg(feature = "xilem")]
mod xilem;

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
    /// Runs the `floem` example.
    #[cfg(feature = "floem")]
    #[command(name = "floem")]
    Floem,
    /// Runs the `iced` example.
    #[cfg(feature = "iced")]
    #[command(name = "iced")]
    Iced,
    /// Runs the `slint` example.
    #[cfg(feature = "slint")]
    #[command(name = "slint")]
    Slint,
    /// Runs the `xilem` example.
    #[cfg(feature = "xilem")]
    #[command(name = "xilem")]
    Xilem,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "floem")]
            Commands::Floem => {
                floem::run();
            }
            #[cfg(feature = "iced")]
            Commands::Iced => {
                iced::run()?;
            }
            #[cfg(feature = "slint")]
            Commands::Slint => {
                slint::run()?;
            }
            #[cfg(feature = "xilem")]
            Commands::Xilem => {
                xilem::run()?;
            }
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

    /// Running without a subcommand is accepted and runs nothing.
    #[test]
    fn missing_subcommand_is_accepted() {
        assert!(Cli::try_parse_from(["example"]).is_ok());
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
