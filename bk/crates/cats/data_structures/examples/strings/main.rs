//! Runs one of the `strings` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod bytes;
mod string_format;
mod strings;

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
    /// Runs the `bytes` example.
    #[command(name = "bytes")]
    Bytes,
    /// Runs the `string_format` example.
    #[command(name = "string_format")]
    StringFormat,
    /// Runs the `strings` example.
    #[command(name = "strings")]
    Strings,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bytes => {
                bytes::run();
            }
            Commands::StringFormat => {
                string_format::run();
            }
            Commands::Strings => {
                strings::run();
            }
        }
    }
}
