use clap::Parser;
use clap::Subcommand;

mod arrayvec;
mod smallvec;
mod tinyvec;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "arrayvec")]
    Arrayvec,
    #[command(name = "smallvec")]
    Smallvec,
    #[command(name = "tinyvec")]
    Tinyvec,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Arrayvec => {
                arrayvec::run();
            }
            Commands::Smallvec => {
                smallvec::run();
            }
            Commands::Tinyvec => {
                tinyvec::run();
            }
        }
    }
}
