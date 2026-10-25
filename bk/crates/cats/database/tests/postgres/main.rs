//! Runs the `tokio-postgres` example as an integration test.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(all(feature = "postgres", test))]
use anyhow::Context as _;

#[cfg(feature = "postgres")]
mod tokio_postgres;

/// Runs the postgres example.
#[cfg(feature = "postgres")]
fn main() -> anyhow::Result<()> {
    tokio_postgres::main()?;
    Ok(())
}

/// Runs the PostgreSQL example against the configured database.
#[cfg(feature = "postgres")]
#[test]
#[ignore = "needs Postgres"]
fn tokio_postgres_round_trip_on_postgres() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .context("set DATABASE_URL to connect to PostgreSQL")?;
    tokio::runtime::Runtime::new()?
        .block_on(tokio_postgres::run_with_url(&database_url))
}

/// Does nothing when the `postgres` feature is disabled.
#[cfg(not(feature = "postgres"))]
fn main() {}
