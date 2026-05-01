use clap::Parser;
use clap::Subcommand;

mod diff;
mod similar;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "diff")]
    Diff,
    #[command(name = "similar")]
    Similar,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Diff => {
                let _ = diff::run();
            }
            Commands::Similar => {
                let _ = similar::run();
            }
        }
    }
}
