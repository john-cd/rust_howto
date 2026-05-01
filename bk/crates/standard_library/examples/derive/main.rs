use clap::Parser;
use clap::Subcommand;

mod derive;
mod derive_more;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "derive")]
    Derive,
    #[command(name = "derive_more")]
    DeriveMore,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Derive => {
                derive::run();
            }
            Commands::DeriveMore => {
                derive_more::run();
            }
        }
    }
}
