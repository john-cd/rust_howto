//! Runs one of the `architecture` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// CQRS (Command Query Responsibility Segregation) is an architectural pattern that separates the
/// models for reading and writing data.
mod cqrs;
/// Dependency Injection (DI) is a design pattern that allows us to decouple
mod di;
/// Event-driven architecture: producers publish events to subscribers.
mod event_bus;
/// Event sourcing: store events, rebuild the state by replaying them.
mod event_sourcing;
/// Hexagonal architecture: a port (trait) with two adapters.
mod hexagonal;
/// This example demonstrates a layered architecture in Rust, separating concerns into data access,
/// business logic, and presentation layers.
mod layered_architecture;
/// Pipeline architecture: stages connected by channels, each on its own thread.
mod pipeline;
/// Example usage of the `InMemoryUserRepository`.
mod repository;
/// A state machine whose transitions are triggered by events.
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

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
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
