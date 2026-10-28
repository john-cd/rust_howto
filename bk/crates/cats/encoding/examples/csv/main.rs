//! Runs one of the `csv` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod delimiter;
mod filter;
mod invalid;
mod read;
mod read1;
mod serde_serialize;
mod serialize;
mod transform;

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
    /// Runs the `delimiter` example.
    #[command(name = "delimiter")]
    Delimiter,
    /// Runs the `filter` example.
    #[command(name = "filter")]
    Filter,
    /// Runs the `invalid` example.
    #[command(name = "invalid")]
    Invalid,
    /// Runs the `read` example.
    #[command(name = "read")]
    Read,
    /// Runs the `read1` example.
    #[command(name = "read1")]
    Read1,
    /// Runs the `serde_serialize` example.
    #[command(name = "serde_serialize")]
    SerdeSerialize,
    /// Runs the `serialize` example.
    #[command(name = "serialize")]
    Serialize,
    /// Runs the `transform` example.
    #[command(name = "transform")]
    Transform,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Delimiter => {
                delimiter::run()?;
            }
            Commands::Filter => {
                filter::run()?;
            }
            Commands::Invalid => {
                invalid::run()?;
            }
            Commands::Read => {
                read::run()?;
            }
            Commands::Read1 => {
                read1::run()?;
            }
            Commands::SerdeSerialize => {
                serde_serialize::run()?;
            }
            Commands::Serialize => {
                serialize::run()?;
            }
            Commands::Transform => {
                transform::run()?;
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
