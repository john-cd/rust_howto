use clap::Parser;
use clap::Subcommand;

mod async1;
mod async2;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "async1")]
    Async1,
    #[command(name = "async2")]
    Async2,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Async1 => {
                async1::run();
            }
            Commands::Async2 => {
                async2::run();
            }
        }
    }
}
