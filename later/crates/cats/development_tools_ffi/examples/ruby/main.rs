// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

#[cfg(not(windows))] // TODO review Windows support for Ruby FFI examples
mod magnus;
#[cfg(not(windows))]
mod rutie;

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
    /// Runs the `magnus` example.
    #[cfg(not(windows))]
    #[command(name = "magnus")]
    Magnus,
    /// Runs the `rutie` example.
    #[cfg(not(windows))]
    #[command(name = "rutie")]
    Rutie,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            #[cfg(not(windows))]
            Commands::Magnus => {
                magnus::run();
            }
            #[cfg(not(windows))]
            Commands::Rutie => {
                rutie::run();
            }
        }
    }
}
