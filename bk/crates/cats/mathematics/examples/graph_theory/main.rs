//! Runs a graph theory example.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod cycle_detection;
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

fn main() {
    dispatch(Cli::parse());
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each example is reachable through its own subcommand and runs.
    #[test]
    fn test_every_subcommand_parses_and_runs() {
        for name in ["cycle-detection", "topological-levels"] {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
            dispatch(cli);
        }
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }

    /// Running without a subcommand selects nothing and succeeds.
    #[test]
    fn test_no_subcommand_runs_nothing() {
        let cli = Cli::try_parse_from(["example"]).expect("should parse");
        assert!(cli.command.is_none());
        dispatch(cli);
    }
}
