//! Runs one of the `read_write` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod memmap2;
mod read_file;
mod same_file;

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
    /// Runs the `memmap2` example.
    #[command(name = "memmap2")]
    Memmap2,
    /// Runs the `read_file` example.
    #[command(name = "read_file")]
    ReadFile,
    /// Runs the `same_file` example.
    #[command(name = "same_file")]
    SameFile,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Memmap2 => {
                memmap2::run()?;
            }
            Commands::ReadFile => {
                read_file::run()?;
            }
            Commands::SameFile => {
                same_file::run()?;
            }
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
