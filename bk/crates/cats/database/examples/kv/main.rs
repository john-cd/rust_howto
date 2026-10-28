//! Runs one of the `kv` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(any(feature = "lmdb", feature = "rocksdb"))]
use clap::Parser;
#[cfg(any(feature = "lmdb", feature = "rocksdb"))]
use clap::Subcommand;

#[cfg(feature = "lmdb")]
mod heed;
#[cfg(feature = "rocksdb")]
mod rocksdb;

/// Command-line arguments.
#[cfg(any(feature = "lmdb", feature = "rocksdb"))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(any(feature = "lmdb", feature = "rocksdb"))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `heed` example.
    #[cfg(feature = "lmdb")]
    #[command(name = "heed")]
    Heed,
    /// Runs the `rocksdb` example.
    #[cfg(feature = "rocksdb")]
    #[command(name = "rocksdb")]
    Rocksdb,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    #[cfg(any(feature = "lmdb", feature = "rocksdb"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "lmdb")]
                Commands::Heed => {
                    heed::run()?;
                }
                #[cfg(feature = "rocksdb")]
                Commands::Rocksdb => {
                    rocksdb::run()?;
                }
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
