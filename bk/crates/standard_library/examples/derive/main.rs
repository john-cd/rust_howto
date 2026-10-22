// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod derive;
mod derive_more;

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
    /// Runs the `derive` example.
    #[command(name = "derive")]
    Derive,
    /// Runs the `derive_more` example.
    #[command(name = "derive_more")]
    DeriveMore,
}

/// Runs the example selected on the command line.
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
