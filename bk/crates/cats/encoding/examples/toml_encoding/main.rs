use clap::Parser;
use clap::Subcommand;

mod toml;
mod toml1;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "toml")]
    Toml,
    #[command(name = "toml1")]
    Toml1,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Toml => {
                toml::run();
            }
            Commands::Toml1 => {
                toml1::run();
            }
        }
    }
}
