//! Runs one of the `game_engines` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "bevy")]
mod bevy;
#[cfg(feature = "fyrox")]
mod fyrox;
#[cfg(feature = "ggez")]
mod ggez;
mod macroquad;

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
    /// Runs the `bevy` example.
    #[command(name = "bevy")]
    #[cfg(feature = "bevy")]
    Bevy,
    /// Runs the `fyrox` example.
    #[command(name = "fyrox")]
    #[cfg(feature = "fyrox")]
    Fyrox,
    /// Runs the `ggez` example.
    #[command(name = "ggez")]
    #[cfg(feature = "ggez")]
    Ggez,
    /// Runs the `macroquad` example.
    #[command(name = "macroquad")]
    Macroquad,
}

/// Runs the example selected on the command line.
///
/// # Errors
///
/// Returns an error if the selected engine's setup or event loop fails.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dispatch(Cli::parse())
}

/// Runs the example selected in the parsed command line.
fn dispatch(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(command) = cli.command {
        match command {
            #[cfg(feature = "bevy")]
            Commands::Bevy => {
                bevy::run();
            }
            #[cfg(feature = "fyrox")]
            Commands::Fyrox => {
                fyrox::run();
            }
            #[cfg(feature = "ggez")]
            Commands::Ggez => {
                ggez::run()?;
            }
            Commands::Macroquad => {
                macroquad::run();
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

    /// Without a subcommand, nothing is run.
    #[test]
    fn no_subcommand_runs_nothing() {
        let cli =
            Cli::try_parse_from(["example"]).expect("no arguments are valid");
        assert!(cli.command.is_none());
        dispatch(cli).expect("the example should run successfully");
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
