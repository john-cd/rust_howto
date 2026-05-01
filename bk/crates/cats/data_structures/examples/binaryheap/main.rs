use clap::Parser;
use clap::Subcommand;

mod binaryheap;
mod priority_queue;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "binaryheap")]
    Binaryheap,
    #[command(name = "priority_queue")]
    PriorityQueue,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Binaryheap => {
                binaryheap::run();
            }
            Commands::PriorityQueue => {
                priority_queue::run();
            }
        }
    }
}
