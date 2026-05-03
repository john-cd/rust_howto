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
    Astar,
    #[command(name = "petgraph_bfs")]
    Bfs,
    #[command(name = "petgraph_dfs")]
    Dfs,
    #[command(name = "petgraph_dijkstra")]
    Dijkstra,
    #[command(name = "petgraph_toposort")]
    Toposort,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
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
        }
    }
}
