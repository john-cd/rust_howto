//! Runs one of the graph algorithm examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

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
fn main() {
    let cli = Cli::parse();

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
