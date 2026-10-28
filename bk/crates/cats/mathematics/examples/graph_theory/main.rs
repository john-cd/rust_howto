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
fn main() {
    match Cli::parse().command {
        Some(Commands::CycleDetection) => cycle_detection::run(),
        Some(Commands::TopologicalLevels) => topological_levels::run(),
        None => {}
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
