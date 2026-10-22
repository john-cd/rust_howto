// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod arrayvec;
mod smallvec;
mod tinyvec;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `arrayvec` example.
    #[command(name = "arrayvec")]
    Arrayvec,
    /// Runs the `smallvec` example.
    #[command(name = "smallvec")]
    Smallvec,
    /// Runs the `tinyvec` example.
    #[command(name = "tinyvec")]
    Tinyvec,
}

/// Runs the example selected on the command line.
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
