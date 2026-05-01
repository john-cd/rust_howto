use clap::Parser;
use clap::Subcommand;

mod hyper;
mod ureq;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "hyper")]
    Hyper,
    #[command(name = "ureq")]
    Ureq,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Hyper => {
                let _ = hyper::run();
            }
            Commands::Ureq => {
                let _ = ureq::run();
            }
        }
    }
}
