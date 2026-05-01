use clap::{Parser, Subcommand};

mod cosmic_text;
mod parley;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "cosmic_text")]
    CosmicText,
    #[command(name = "parley")]
    Parley,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CosmicText => {
                let _ = cosmic_text::run();
            }
            Commands::Parley => {
                let _ = parley::run();
            }
        }
    }
}
