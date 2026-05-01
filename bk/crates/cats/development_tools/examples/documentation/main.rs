//! Documentation tools examples.

use clap::Parser;
use clap::Subcommand;

mod rustdoc;
mod rustdoc2;
mod rustdoc3;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "rustdoc")]
    Rustdoc,
    #[command(name = "rustdoc2")]
    Rustdoc2,
    #[command(name = "rustdoc3")]
    Rustdoc3,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Rustdoc => {
                rustdoc::run();
            }
            Commands::Rustdoc2 => {
                rustdoc2::run();
            }
            Commands::Rustdoc3 => {
                rustdoc3::run();
            }
        }
    }
}
