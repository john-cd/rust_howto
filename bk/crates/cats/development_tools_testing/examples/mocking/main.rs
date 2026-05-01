use clap::Parser;
use clap::Subcommand;

mod faux;
mod mockall;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "faux")]
    Faux,
    #[command(name = "mockall")]
    Mockall,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Faux => {
                faux::run();
            }
            Commands::Mockall => {
                mockall::run();
            }
        }
    }
}
