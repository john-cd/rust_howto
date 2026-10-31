//! Runs one of the `databases` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(any(feature = "sled", feature = "surrealdb", feature = "redb"))]
use clap::Parser;
#[cfg(any(feature = "sled", feature = "surrealdb", feature = "redb"))]
use clap::Subcommand;

#[cfg(feature = "redb")]
mod redb;
#[cfg(feature = "sled")]
mod sled;
#[cfg(feature = "surrealdb")]
mod surrealdb;

/// Command-line arguments.
#[cfg(any(feature = "sled", feature = "surrealdb", feature = "redb"))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(any(feature = "sled", feature = "surrealdb", feature = "redb"))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `redb` example.
    #[cfg(feature = "redb")]
    #[command(name = "redb")]
    Redb,
    /// Runs the `sled` example.
    #[cfg(feature = "sled")]
    #[command(name = "sled")]
    Sled,
    /// Runs the `surrealdb` example.
    #[cfg(feature = "surrealdb")]
    #[command(name = "surrealdb")]
    Surrealdb,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    #[cfg(any(feature = "sled", feature = "surrealdb", feature = "redb"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "redb")]
                Commands::Redb => {
                    redb::run()?;
                }
                #[cfg(feature = "sled")]
                Commands::Sled => {
                    sled::run()?;
                }
                #[cfg(feature = "surrealdb")]
                Commands::Surrealdb => {
                    surrealdb::run()?;
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
    #[cfg(not(any(
        feature = "sled",
        feature = "surrealdb",
        feature = "redb"
    )))]
    #[test]
    fn test_main_is_a_no_op_without_features() {
        assert!(main().is_ok());
    }

    #[cfg(feature = "redb")]
    #[test]
    fn test_redb_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "redb"]).is_ok());
    }

    #[cfg(feature = "sled")]
    #[test]
    fn test_sled_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "sled"]).is_ok());
    }

    #[cfg(feature = "surrealdb")]
    #[test]
    fn test_surrealdb_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "surrealdb"]).is_ok());
    }

    /// Unknown subcommands are rejected.
    #[cfg(any(feature = "sled", feature = "surrealdb", feature = "redb"))]
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
