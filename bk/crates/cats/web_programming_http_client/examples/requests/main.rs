use clap::Parser;
use clap::Subcommand;

mod get;
mod get1;
mod header;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "get")]
    Get,
    #[command(name = "get1")]
    Get1,
    #[command(name = "header")]
    Header,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Get => {
                let _ = get::run();
            }
            Commands::Get1 => {
                let _ = get1::run();
            }
            Commands::Header => {
                let _ = header::run();
            }
        }
    }
}
