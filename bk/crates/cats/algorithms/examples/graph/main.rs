use clap::Parser;
use clap::Subcommand;

mod petgraph_astar;
mod petgraph_bfs;
mod petgraph_dfs;
mod petgraph_dijkstra;
mod petgraph_toposort;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "petgraph_astar")]
    PetgraphAstar,
    #[command(name = "petgraph_bfs")]
    PetgraphBfs,
    #[command(name = "petgraph_dfs")]
    PetgraphDfs,
    #[command(name = "petgraph_dijkstra")]
    PetgraphDijkstra,
    #[command(name = "petgraph_toposort")]
    PetgraphToposort,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::PetgraphAstar => {
                petgraph_astar::run();
            }
            Commands::PetgraphBfs => {
                petgraph_bfs::run();
            }
            Commands::PetgraphDfs => {
                petgraph_dfs::run();
            }
            Commands::PetgraphDijkstra => {
                petgraph_dijkstra::run();
            }
            Commands::PetgraphToposort => {
                petgraph_toposort::run();
            }
        }
    }
}
