//! Runs one of the `ruby` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod greeting;
#[cfg(all(not(windows), feature = "ruby"))]
// TODO review Windows support for Ruby FFI examples
mod magnus;
#[cfg(all(not(windows), feature = "ruby"))]
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
    #[cfg(all(not(windows), feature = "ruby"))]
    #[command(name = "magnus")]
    Magnus,
    /// Runs the `rutie` example.
    #[cfg(all(not(windows), feature = "ruby"))]
    #[command(name = "rutie")]
    Rutie,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();
    println!("{}", greeting::message());

    if let Some(command) = cli.command {
        match command {
            #[cfg(all(not(windows), feature = "ruby"))]
            Commands::Magnus => {
                magnus::run().expect("the Ruby example should run");
            }
            #[cfg(all(not(windows), feature = "ruby"))]
            Commands::Rutie => {
                rutie::run();
            }
        }
    }
}
