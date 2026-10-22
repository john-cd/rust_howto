// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod enums;
mod strum;

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
    /// Runs the `enums` example.
    #[command(name = "enums")]
    Enums,
    /// Runs the `strum` example.
    #[command(name = "strum")]
    Strum,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Enums => {
                enums::run();
            }
            Commands::Strum => {
                strum::run();
            }
        }
    }
}
