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
                let _ = petgraph_astar::run();
            }
            Commands::PetgraphBfs => {
                let _ = petgraph_bfs::run();
            }
            Commands::PetgraphDfs => {
                let _ = petgraph_dfs::run();
            }
            Commands::PetgraphDijkstra => {
                let _ = petgraph_dijkstra::run();
            }
            Commands::PetgraphToposort => {
                let _ = petgraph_toposort::run();
            }
        }
    }
}
