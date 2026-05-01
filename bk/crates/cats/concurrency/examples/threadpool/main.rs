use clap::Parser;
use clap::Subcommand;

mod threadpool_fractal;
mod threadpool_walk;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "threadpool_fractal")]
    ThreadpoolFractal,
    #[command(name = "threadpool_walk")]
    ThreadpoolWalk,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ThreadpoolFractal => {
                let _ = threadpool_fractal::run();
            }
            Commands::ThreadpoolWalk => {
                let _ = threadpool_walk::run();
            }
        }
    }
}
