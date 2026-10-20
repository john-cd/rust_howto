use clap::Parser;
use clap::Subcommand;

#[cfg(target_family = "unix")]
mod nix;
#[cfg(target_family = "unix")]
mod rustix;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(target_family = "unix")]
    #[command(name = "nix")]
    Nix,
    #[cfg(target_family = "unix")]
    #[command(name = "rustix")]
    Rustix,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(target_family = "unix")]
            Commands::Nix => {
                nix::run();
            }
            #[cfg(target_family = "unix")]
            Commands::Rustix => {
                rustix::run();
            }
        }
    }
}
