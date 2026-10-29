//! Runs one of the `architecture` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod cqrs;
mod di;
mod event_bus;
mod event_sourcing;
mod hexagonal;
mod layered_architecture;
mod pipeline;
mod repository;
mod state_machine;

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
    /// Runs the `cqrs` example.
    #[command(name = "cqrs")]
    Cqrs,
    /// Runs the `di` example.
    #[command(name = "di")]
    Di,
    /// Runs the `event_bus` example.
    #[command(name = "event_bus")]
    EventBus,
    /// Runs the `event_sourcing` example.
    #[command(name = "event_sourcing")]
    EventSourcing,
    /// Runs the `hexagonal` example.
    #[command(name = "hexagonal")]
    Hexagonal,
    /// Runs the `pipeline` example.
    #[command(name = "pipeline")]
    Pipeline,
    /// Runs the `layered_architecture` example.
    #[command(name = "layered_architecture")]
    LayeredArchitecture,
    /// Runs the `repository` example.
    #[command(name = "repository")]
    Repository,
    /// Runs the `state_machine` example.
    #[command(name = "state_machine")]
    StateMachine,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

/// Runs the example selected in the parsed command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Cqrs => {
                cqrs::run()?;
            }
            Commands::Di => {
                di::run();
            }
            Commands::EventBus => {
                event_bus::run();
            }
            Commands::EventSourcing => {
                event_sourcing::run();
            }
            Commands::Hexagonal => {
                hexagonal::run();
            }
            Commands::Pipeline => {
                pipeline::run();
            }
            Commands::LayeredArchitecture => {
                layered_architecture::run()?;
            }
            Commands::Repository => {
                repository::run();
            }
            Commands::StateMachine => {
                state_machine::run();
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
