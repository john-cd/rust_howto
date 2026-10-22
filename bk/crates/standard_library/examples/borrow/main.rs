// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod borrow;
mod borrow2;
mod borrow3;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `borrow` example.
    #[command(name = "borrow")]
    Borrow,
    /// Runs the `borrow2` example.
    #[command(name = "borrow2")]
    Borrow2,
    /// Runs the `borrow3` example.
    #[command(name = "borrow3")]
    Borrow3,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Borrow => {
                borrow::run();
            }
            Commands::Borrow2 => {
                borrow2::run();
            }
            Commands::Borrow3 => {
                borrow3::run();
            }
        }
    }
}
