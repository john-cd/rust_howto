use clap::Parser;
use clap::Subcommand;

mod indicatif;
mod indicatif2;
mod inquire;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "indicatif")]
    Indicatif,
    #[command(name = "indicatif2")]
    Indicatif2,
    #[command(name = "inquire")]
    Inquire,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Indicatif => {
                indicatif::run();
            }
            Commands::Indicatif2 => {
                indicatif2::run();
            }
            Commands::Inquire => {
                inquire::run();
            }
        }
    }
}
