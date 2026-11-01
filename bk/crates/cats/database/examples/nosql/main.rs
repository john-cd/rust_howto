//! Runs one of the `nosql` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(any(feature = "mongodb", feature = "redis"))]
use clap::Parser;
#[cfg(any(feature = "mongodb", feature = "redis"))]
use clap::Subcommand;

#[cfg(feature = "mongodb")]
mod mongodb;
#[cfg(feature = "redis")]
mod redis;

/// Command-line arguments.
#[cfg(any(feature = "mongodb", feature = "redis"))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(any(feature = "mongodb", feature = "redis"))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `mongodb` example.
    #[cfg(feature = "mongodb")]
    #[command(name = "mongodb")]
    Mongodb,
    /// Runs the `redis` example.
    #[cfg(feature = "redis")]
    #[command(name = "redis")]
    Redis,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    #[cfg(any(feature = "mongodb", feature = "redis"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "mongodb")]
                Commands::Mongodb => {
                    mongodb::run()?;
                }
                #[cfg(feature = "redis")]
                Commands::Redis => {
                    redis::run()?;
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
    #[cfg(not(any(feature = "mongodb", feature = "redis")))]
    #[test]
    fn test_main_is_a_no_op_without_features() {
        assert!(main().is_ok());
    }

    #[cfg(feature = "mongodb")]
    #[test]
    fn test_mongodb_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "mongodb"]).is_ok());
    }

    #[cfg(feature = "redis")]
    #[test]
    fn test_redis_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "redis"]).is_ok());
    }

    /// Unknown subcommands are rejected.
    #[cfg(any(feature = "mongodb", feature = "redis"))]
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
