//! Runs one of the `postgres` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

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
    command: Option<Commands>,
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

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "postgres")]
                Commands::AggregateData => {
                    aggregate_data::run().map_err(anyhow::Error::msg)?;
                }
                #[cfg(feature = "postgres")]
                Commands::Cornucopia => {
                    cornucopia::run().map_err(anyhow::Error::msg)?;
                }
                #[cfg(feature = "postgres")]
                Commands::CreateTables => {
                    create_tables::run()?;
                }
                #[cfg(feature = "postgres")]
                Commands::InsertQueryData => {
                    insert_query_data::run().map_err(anyhow::Error::msg)?;
                }
            }
        }
    }
    Ok(())
}

/// Serializes the tests, which change the environment variables.
#[cfg(feature = "postgres")]
#[allow(dead_code)]
pub(crate) static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
#[cfg(feature = "postgres")]
mod tests {
    use super::*;

    #[test]
    fn require_external_svc() -> anyhow::Result<()> {
        let _lock = ENV_MUTEX.lock().unwrap();
        let test_url = if let Ok(val) = std::env::var("TEST_PG_URL") { val } else {
            eprintln!(
                "Skipping Postgres integration test; set TEST_PG_URL to run."
            );
            return Ok(());
        };

        unsafe {
            std::env::set_var("PG_URL", test_url);
        }
        main()?;
        Ok(())
    }
}
// [review](https://github.com/john-cd/rust_howto/issues/713)
