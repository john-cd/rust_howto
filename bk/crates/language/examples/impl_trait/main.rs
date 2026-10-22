// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod impl_trait;
mod rpit;

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
    /// Runs the `impl_trait` example.
    #[command(name = "impl_trait")]
    ImplTrait,
    /// Runs the `rpit` example.
    #[command(name = "rpit")]
    Rpit,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ImplTrait => {
                impl_trait::run();
            }
            Commands::Rpit => {
                rpit::run();
            }
        }
    }
}
