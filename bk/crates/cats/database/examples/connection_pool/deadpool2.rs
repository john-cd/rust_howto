#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! This example demonstrates how to use `deadpool-postgres` to manage a pool of
//! PostgreSQL connections.
//! `deadpool` is an async pool for connections and objects of any type.
//! `deadpool_postgres` implements a `deadpool` manager for `tokio-postgres`
//! and also provides a statement cache by wrapping `tokio_postgres::Client` and
//! `tokio_postgres::Transaction`.
//!
//! Add to your `Config.toml`:
//! deadpool = "0.12.1" # or latest version
//! deadpool-postgres = { version = "0.14.1", features = ["serde"] }
//!
//! Add the database's configuration to your `.env` file, for example:
//! `PG__HOST=pg.example.com`
//! `PG__USER=john_doe`
//! `PG__PASSWORD=topsecret`
//! `PG__DBNAME=example`
//! `PG__POOL__MAX_SIZE=16`
//! `PG__POOL__TIMEOUTS__WAIT__SECS=5`
//! `PG__POOL__TIMEOUTS__WAIT__NANOS=0`

use deadpool_postgres::Runtime;
use dotenvy::dotenv;
use tokio_postgres::NoTls;

/// Configuration for the application.
#[derive(Debug, serde::Deserialize)]
struct Config {
    /// The settings of the PostgreSQL connection pool.
    pub pg: deadpool_postgres::Config,
}

impl Config {
    // With the `serde` feature enabled, we can read the configuration using the
    // `config` crate.
    /// Reads the configuration from the environment variables.
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

/// Loads the PostgreSQL settings and creates a Tokio-backed connection pool.
///
/// # Errors
///
/// Returns an error if configuration cannot be read or the pool cannot be
/// built.
fn create_pool() -> anyhow::Result<deadpool_postgres::Pool> {
    // Loads the `.env` file from the current directory or parents.
    dotenv().ok();
    // Hydrate the configuration from environment variables.
    let cfg = Config::from_env()?;
    // Creates a new `Pool` using the `deadpool_postgres::Config`.
    Ok(cfg.pg.create_pool(Some(Runtime::Tokio1), NoTls)?)
}

/// Creates a PostgreSQL connection pool and queries the database.
///
/// # Errors
///
/// Returns an error if configuration, pool checkout, or the version query
/// fails.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = create_pool()?;
    // Retrieves an Object from this Pool or waits for one to become available:
    let client = pool.get().await?;
    // Like `tokio_postgres::Client::prepare()`, but uses an existing
    // `Statement` from the `StatementCache` if possible:
    let stmt = client.prepare_cached("SELECT version()").await?;
    let rows = client.query(&stmt, &[]).await?;

    for row in rows {
        let version: &str = row.get(0);
        println!("PostgreSQL version: {version}");
    }

    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
///
/// # Errors
///
/// Returns an error if the PostgreSQL connection or query fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use tokio::task::JoinSet;

    use super::*;

    /// Exercises pool checkout and query behavior with many concurrent clients.
    #[tokio::test]
    #[ignore = "requires a reachable PostgreSQL service and PG__* environment variables"]
    async fn serves_concurrent_queries_through_the_pool() -> anyhow::Result<()>
    {
        const CONCURRENT_QUERIES: usize = 64;

        let pool = create_pool()?;
        let mut tasks = JoinSet::new();

        for request_id in 0..CONCURRENT_QUERIES as i32 {
            let pool = pool.clone();
            tasks.spawn(async move {
                let client = pool.get().await?;
                let row = client
                    .query_one(
                        "SELECT pg_sleep(0.01), $1::integer",
                        &[&request_id],
                    )
                    .await?;
                Ok::<i32, anyhow::Error>(row.get(1))
            });
        }

        let mut completed_ids = Vec::with_capacity(CONCURRENT_QUERIES);
        while let Some(result) = tasks.join_next().await {
            completed_ids.push(result??);
        }
        completed_ids.sort_unstable();

        assert_eq!(
            completed_ids,
            (0..CONCURRENT_QUERIES as i32).collect::<Vec<_>>()
        );
        Ok(())
    }
}
// [finish; need heavy test](https://github.com/john-cd/rust_howto/issues/46)
