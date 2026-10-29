//! Runs one of the graph algorithm examples, chosen on the command line.
//!
//! Reviewed 10-27-2026.

use clap::Parser;
use clap::Subcommand;

mod pathfinding_astar;
mod petgraph_astar;
mod petgraph_bfs;
mod petgraph_dfs;
mod petgraph_dijkstra;
mod petgraph_mst;
mod petgraph_toposort;

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
    /// Runs the `pathfinding_astar` example.
    #[command(name = "pathfinding_astar")]
    PathfindingAstar,
    /// Runs the `petgraph_astar` example.
    #[command(name = "petgraph_astar")]
    Astar,
    /// Runs the `petgraph_bfs` example.
    #[command(name = "petgraph_bfs")]
    Bfs,
    /// Runs the `petgraph_dfs` example.
    #[command(name = "petgraph_dfs")]
    Dfs,
    /// Runs the `petgraph_dijkstra` example.
    #[command(name = "petgraph_dijkstra")]
    Dijkstra,
    /// Runs the `petgraph_toposort` example.
    #[command(name = "petgraph_toposort")]
    Toposort,
    /// Runs the `petgraph_mst` example.
    #[command(name = "petgraph_mst")]
    Mst,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::PathfindingAstar => pathfinding_astar::run(),
            Commands::Astar => {
                petgraph_astar::run();
            }
            Commands::Bfs => {
                petgraph_bfs::run();
            }
            Commands::Dfs => {
                petgraph_dfs::run();
            }
            Commands::Dijkstra => {
                petgraph_dijkstra::run();
            }
            Commands::Toposort => {
                petgraph_toposort::run();
            }
            Commands::Mst => petgraph_mst::run(),
        }
    }
}

/// Parses the command line and runs the selected example.
fn main() {
    dispatch(Cli::parse());
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["pathfinding_astar", "petgraph_astar", "petgraph_bfs", "petgraph_dfs", "petgraph_dijkstra", "petgraph_toposort", "petgraph_mst"];

    /// The command-line definition is internally consistent.
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
