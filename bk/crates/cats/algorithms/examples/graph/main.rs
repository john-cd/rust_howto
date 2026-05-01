use clap::Parser;

mod petgraph_astar;
mod petgraph_bfs;
mod petgraph_dfs;
mod petgraph_dijkstra;
mod petgraph_toposort;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    example: String,
}

fn main() {
    let cli = Cli::parse();
    match cli.example.as_str() {
        "astar" => petgraph_astar::run(),
        "bfs" => petgraph_bfs::run(),
        "dfs" => petgraph_dfs::run(),
        "dijkstra" => petgraph_dijkstra::run(),
        "toposort" => petgraph_toposort::run(),
        _ => eprintln!("Unknown example. Available: astar, bfs, dfs, dijkstra, toposort"),
    }
}
