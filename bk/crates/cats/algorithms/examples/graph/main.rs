//! Runs one of the graph algorithm examples, chosen on the command line.
//!
//! Reviewed 10-27-2026.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Implicit grid pathfinding using the A* algorithm and the `pathfinding` crate.
mod pathfinding_astar;
/// Finds a shortest path on a weighted graph with A* and Manhattan distance.
mod petgraph_astar;
/// Visits graph nodes level by level from the start node.
mod petgraph_bfs;
/// Explores a graph depth first to illustrate branch traversal order.
mod petgraph_dfs;
/// Computes single-source shortest paths in a weighted directed graph.
mod petgraph_dijkstra;
/// Builds a minimum spanning tree of a weighted undirected graph with Kruskal's algorithm.
mod petgraph_mst;
/// Orders tasks so each dependency precedes the task that needs it.
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
    /// Finds a route through a grid with A* pathfinding.
    #[command(name = "pathfinding_astar")]
    PathfindingAstar,

    /// Finds a low-cost route through a weighted graph with A*.
    #[command(name = "petgraph_astar")]
    Astar,

    /// Traverses an unweighted graph level by level with BFS.
    #[command(name = "petgraph_bfs")]
    Bfs,

    /// Traverses a graph depth first with DFS.
    #[command(name = "petgraph_dfs")]
    Dfs,

    /// Finds shortest paths from one source in a weighted graph.
    #[command(name = "petgraph_dijkstra")]
    Dijkstra,

    /// Orders task dependencies with a topological sort.
    #[command(name = "petgraph_toposort")]
    Toposort,

    /// Builds a minimum spanning tree with Kruskal's algorithm.
    #[command(name = "petgraph_mst")]
    Mst,
}

/// Invokes the graph module selected by the subcommand.
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

/// Parses the graph subcommand and starts its matching example.
fn main() {
    dispatch(Cli::parse());
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "pathfinding_astar",
        "petgraph_astar",
        "petgraph_bfs",
        "petgraph_dfs",
        "petgraph_dijkstra",
        "petgraph_toposort",
        "petgraph_mst",
    ];

    /// Confirms that the command and its arguments are valid.
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
}
