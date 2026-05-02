use clap::Parser;
use clap::Subcommand;

mod magnus;
mod rutie;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "magnus")]
    Magnus,
    #[command(name = "rutie")]
    Rutie,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Magnus => {
                magnus::run();
            }
            Commands::Rutie => {
                rutie::run();
            }
        }
    }
}
