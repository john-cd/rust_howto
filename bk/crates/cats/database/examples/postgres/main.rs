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
#[cfg(feature = "postgres")]
mod tokio_postgres;

#[cfg(feature = "postgres")]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(feature = "postgres")]
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "postgres")]
    #[command(name = "aggregate_data")]
    AggregateData,
    #[cfg(feature = "postgres")]
    #[command(name = "cornucopia")]
    Cornucopia,
    #[cfg(feature = "postgres")]
    #[command(name = "create_tables")]
    CreateTables,
    #[cfg(feature = "postgres")]
    #[command(name = "insert_query_data")]
    InsertQueryData,
    #[cfg(feature = "postgres")]
    #[command(name = "tokio_postgres")]
    TokioPostgres,
}

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
                #[cfg(feature = "postgres")]
                Commands::TokioPostgres => {
                    tokio_postgres::run()?;
                }
            }
        }
    }
    Ok(())
}

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
        let test_url = match std::env::var("TEST_PG_URL") {
            Ok(val) => val,
            Err(_) => {
                eprintln!(
                    "Skipping Postgres integration test; set TEST_PG_URL to run."
                );
                return Ok(());
            }
        };

        unsafe {
            std::env::set_var("PG_URL", test_url);
        }
        main()?;
        Ok(())
    }
}
// [review](https://github.com/john-cd/rust_howto/issues/713)
