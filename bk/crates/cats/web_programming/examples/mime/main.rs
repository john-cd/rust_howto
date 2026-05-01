use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

mod filename;
mod request;
mod string;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "filename")]
    Filename,
    #[command(name = "request")]
    Request,
    #[command(name = "string")]
    String,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Filename => {
                filename::run();
            }
            Commands::Request => {
                request::run();
            }
            Commands::String => {
                string::run();
            }
        }
    }
}
