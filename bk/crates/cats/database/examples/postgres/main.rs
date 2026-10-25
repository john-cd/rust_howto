//! Runs one of the `postgres` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(feature = "postgres")]
use clap::Parser;
#[cfg(feature = "postgres")]
use clap::Subcommand;

#[cfg(feature = "postgres")]
mod aggregate_data;
#[cfg(feature = "postgres")]
mod cornucopia;
#[cfg(feature = "postgres")]
mod create_tables;
#[cfg(feature = "postgres")]
mod insert_query_data;

/// Command-line arguments.
#[cfg(feature = "postgres")]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Commands,
}

/// The examples that can be run.
#[cfg(feature = "postgres")]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `aggregate_data` example.
    #[cfg(feature = "postgres")]
    #[command(name = "aggregate_data")]
    AggregateData,
    /// Runs the `cornucopia` example.
    #[cfg(feature = "postgres")]
    #[command(name = "cornucopia")]
    Cornucopia,
    /// Runs the `create_tables` example.
    #[cfg(feature = "postgres")]
    #[command(name = "create_tables")]
    CreateTables,
    /// Runs the `insert_query_data` example.
    #[cfg(feature = "postgres")]
    #[command(name = "insert_query_data")]
    InsertQueryData,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "postgres")]
    {
        let cli = Cli::parse();

        match cli.command {
            Commands::AggregateData => {
                aggregate_data::run()?;
            }
            Commands::Cornucopia => {
                cornucopia::run()?;
            }
            Commands::CreateTables => {
                create_tables::run()?;
            }
            Commands::InsertQueryData => {
                insert_query_data::run().map_err(anyhow::Error::msg)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[cfg(feature = "postgres")]
mod tests {
    use super::*;

    /// Rejects invocations that do not select a PostgreSQL example.
    #[test]
    fn requires_a_subcommand() {
        assert!(Cli::try_parse_from(["postgres"]).is_err());
    }
}
// [review](https://github.com/john-cd/rust_howto/issues/713)
