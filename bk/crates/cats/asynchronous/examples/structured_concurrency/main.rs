//! Runs one of the `structured_concurrency` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Builds or traverses structured concurrency graph data while preserving relationships needed by
/// downstream analysis. Workspace and rendering code use this module to obtain a consistent view of
/// the graph.
mod cancellation_tree;
/// A bounded worker pool with `JoinSet`: keep at most `LIMIT` tasks running, refill as tasks
/// finish, collect results in completion order, and tell a
mod joinset_pool;
/// A supervisor that restarts a failing task with capped exponential backoff
mod supervisor;
/// Choosing a Tokio channel: `watch` keeps only the latest value (config, state), `broadcast`
/// delivers every event but receivers can lag, and `mpsc` is a work queue where each message goes
/// to one consumer.
mod watch_latest;

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
    /// Runs the `cancellation_tree` example.
    #[command(name = "cancellation_tree")]
    CancellationTree,
    /// Runs the `joinset_pool` example.
    #[command(name = "joinset_pool")]
    JoinsetPool,
    /// Runs the `supervisor` example.
    #[command(name = "supervisor")]
    Supervisor,
    /// Runs the `watch_latest` example.
    #[command(name = "watch_latest")]
    WatchLatest,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::CancellationTree => {
                cancellation_tree::run();
            }
            Commands::JoinsetPool => {
                joinset_pool::run();
            }
            Commands::Supervisor => {
                supervisor::run();
            }
            Commands::WatchLatest => {
                watch_latest::run();
            }
        }
    }
}
/// Runs the example.
/// Parses the command line and runs the selected example.
fn main() {
    dispatch(Cli::parse());
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "cancellation_tree",
        "joinset_pool",
        "supervisor",
        "watch_latest",
    ];

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
