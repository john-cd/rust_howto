//! Runs a graph theory example.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Detect dependency cycles in a directed graph.
mod cycle_detection;
/// Group tasks into levels that can run in parallel.
mod topological_levels;

/// Command-line arguments for the graph theory examples.
#[derive(Parser)]
struct Cli {
    /// The graph example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Graph examples available from this executable.
#[derive(Subcommand)]
enum Commands {
    /// Reports strongly connected components that contain a cycle.
    CycleDetection,
    /// Evaluates DAG tasks in parallel, one ready level at a time.
    TopologicalLevels,
}

/// Runs the selected graph theory example.
fn dispatch(cli: Cli) {
    match cli.command {
        Some(Commands::CycleDetection) => cycle_detection::run(),
        Some(Commands::TopologicalLevels) => topological_levels::run(),
        None => {}
    }
}
/// Runs the example.
fn main() {
    dispatch(Cli::parse());
}

/// Each example is reachable through its own subcommand and runs.
#[cfg(test)]
mod tests {
    use super::*;

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
