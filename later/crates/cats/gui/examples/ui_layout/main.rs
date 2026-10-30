//! Runs one of the `ui_layout` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod morphorm;
mod taffy;

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
    /// Runs the `morphorm` example.
    #[command(name = "morphorm")]
    Morphorm,
    /// Runs the `taffy` example.
    #[command(name = "taffy")]
    Taffy,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Morphorm => morphorm::run(),
            Commands::Taffy => taffy::run()?,
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
