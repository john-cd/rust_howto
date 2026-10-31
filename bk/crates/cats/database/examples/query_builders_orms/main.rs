//! Runs one of the `query_builders_orms` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(any(feature = "diesel", feature = "sea_orm", feature = "sqlx"))]
use clap::Parser;
#[cfg(any(feature = "diesel", feature = "sea_orm", feature = "sqlx"))]
use clap::Subcommand;

#[cfg(feature = "diesel")]
mod diesel1;
#[cfg(feature = "sea_orm")]
mod sea_orm;
#[cfg(feature = "sea_orm")]
mod seaography;
#[cfg(feature = "sqlx")]
mod sqlx;

/// Command-line arguments.
#[cfg(any(feature = "diesel", feature = "sea_orm", feature = "sqlx"))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(any(feature = "diesel", feature = "sea_orm", feature = "sqlx"))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `diesel1` example.
    #[cfg(feature = "diesel")]
    #[command(name = "diesel1")]
    Diesel1,
    /// Runs the `sea_orm` example.
    #[cfg(feature = "sea_orm")]
    #[command(name = "sea_orm")]
    SeaOrm,
    /// Runs the `seaography` example.
    #[cfg(feature = "sea_orm")]
    #[command(name = "seaography")]
    Seaography,
    /// Runs the `sqlx` example.
    #[cfg(feature = "sqlx")]
    #[command(name = "sqlx")]
    Sqlx,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    #[cfg(any(feature = "diesel", feature = "sea_orm", feature = "sqlx"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "diesel")]
                Commands::Diesel1 => {
                    diesel1::run()?;
                }
                #[cfg(feature = "sea_orm")]
                Commands::SeaOrm => {
                    sea_orm::run().map_err(anyhow::Error::msg)?;
                }
                #[cfg(feature = "sea_orm")]
                Commands::Seaography => {
                    seaography::run()?;
                }
                #[cfg(feature = "sqlx")]
                Commands::Sqlx => {
                    sqlx::run()?;
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
        feature = "diesel",
        feature = "sea_orm",
        feature = "sqlx"
    )))]
    #[test]
    fn test_main_is_a_no_op_without_features() {
        assert!(main().is_ok());
    }

    #[cfg(feature = "diesel")]
    #[test]
    fn test_diesel1_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "diesel1"]).is_ok());
    }

    #[cfg(feature = "sea_orm")]
    #[test]
    fn test_sea_orm_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "sea_orm"]).is_ok());
    }

    #[cfg(feature = "sea_orm")]
    #[test]
    fn test_seaography_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "seaography"]).is_ok());
    }

    #[cfg(feature = "sqlx")]
    #[test]
    fn test_sqlx_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "sqlx"]).is_ok());
    }

    /// Unknown subcommands are rejected.
    #[cfg(any(feature = "diesel", feature = "sea_orm", feature = "sqlx"))]
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
