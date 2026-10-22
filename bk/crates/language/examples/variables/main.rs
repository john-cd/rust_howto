//! Runs one of the `variables` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod destructuring;
mod shadowing;
mod vars;
mod vars2;

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
    /// Runs the `destructuring` example.
    #[command(name = "destructuring")]
    Destructuring,
    /// Runs the `shadowing` example.
    #[command(name = "shadowing")]
    Shadowing,
    /// Runs the `vars` example.
    #[command(name = "vars")]
    Vars,
    /// Runs the `vars2` example.
    #[command(name = "vars2")]
    Vars2,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Destructuring => {
                destructuring::run();
            }
            Commands::Shadowing => {
                shadowing::run();
            }
            Commands::Vars => {
                vars::run();
            }
            Commands::Vars2 => {
                vars2::run();
            }
        }
    }
}
