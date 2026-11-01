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

    /// Without any example feature, `main` has nothing to run and succeeds.
    #[cfg(not(any(feature = "lmdb", feature = "rocksdb")))]
    #[test]
    fn test_main_is_a_no_op_without_features() {
        assert!(main().is_ok());
    }

    #[cfg(feature = "lmdb")]
    #[test]
    fn test_heed_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "heed"]).is_ok());
    }

    #[cfg(feature = "rocksdb")]
    #[test]
    fn test_rocksdb_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "rocksdb"]).is_ok());
    }

    /// Unknown subcommands are rejected.
    #[cfg(any(feature = "lmdb", feature = "rocksdb"))]
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
