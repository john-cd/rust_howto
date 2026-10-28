//! Runs one of the `unique_ids` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod short_uuid;
mod ulid;
mod uuid;

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
    /// Runs the `short_uuid` example.
    #[command(name = "short_uuid")]
    ShortUuid,
    /// Runs the `ulid` example.
    #[command(name = "ulid")]
    Ulid,
    /// Runs the `uuid` example.
    #[command(name = "uuid")]
    Uuid,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ShortUuid => {
                if let Err(e) = short_uuid::run() {
                    eprintln!("{e}");
                }
            }
            Commands::Ulid => {
                ulid::run();
            }
            Commands::Uuid => {
                uuid::run();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
