use clap::Parser;
use clap::Subcommand;

mod nix;
mod rustix;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "nix")]
    Nix,
    #[command(name = "rustix")]
    Rustix,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Nix => {
                let _ = nix::run();
            }
            Commands::Rustix => {
                let _ = rustix::run();
            }
        }
    }
}
