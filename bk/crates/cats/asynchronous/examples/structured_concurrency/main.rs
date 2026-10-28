//! Runs one of the `structured_concurrency` examples, chosen on the command
//! line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod cancellation_tree;
mod joinset_pool;
mod supervisor;
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
fn main() {
    let cli = Cli::parse();

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
