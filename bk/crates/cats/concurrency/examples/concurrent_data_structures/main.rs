use clap::Parser;
use clap::Subcommand;

mod crossbeam_queue;
mod dashmap;
mod flurry;
mod papaya;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "crossbeam_queue")]
    CrossbeamQueue,
    #[command(name = "dashmap")]
    Dashmap,
    #[command(name = "flurry")]
    Flurry,
    #[command(name = "papaya")]
    Papaya,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CrossbeamQueue => {
                let _ = crossbeam_queue::run();
            }
            Commands::Dashmap => {
                let _ = dashmap::run();
            }
            Commands::Flurry => {
                let _ = flurry::run();
            }
            Commands::Papaya => {
                let _ = papaya::run();
            }
        }
    }
}
