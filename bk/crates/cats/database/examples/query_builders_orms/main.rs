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

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
