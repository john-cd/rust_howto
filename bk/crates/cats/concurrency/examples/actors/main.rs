// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod actix;
mod actors;
mod ractor;
#[cfg(not(windows))]
mod riker;
#[cfg(not(windows))]
mod stakker;

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
    /// Runs the `actix` example.
    #[command(name = "actix")]
    Actix,
    /// Runs the `actors` example.
    #[command(name = "actors")]
    Actors,
    /// Runs the `ractor` example.
    #[command(name = "ractor")]
    Ractor,
    /// Runs the `riker` example.
    #[cfg(not(windows))] // TODO review if this should be gated on a feature instead of the OS
    #[command(name = "riker")]
    Riker,
    /// Runs the `stakker` example.
    #[cfg(not(windows))] // TODO review if this should be gated on a feature instead of the OS
    #[command(name = "stakker")]
    Stakker,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Actix => {
                actix::run();
            }
            Commands::Actors => {
                actors::run();
            }
            Commands::Ractor => {
                ractor::run();
            }
            #[cfg(not(windows))]
            Commands::Riker => {
                riker::run();
            }
            #[cfg(not(windows))]
            Commands::Stakker => {
                stakker::run();
            }
        }
    }
}
