// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use `deadpool-postgres` to manage a pool of
//! PostgreSQL connections.
//! `deadpool` is an async pool for connections and objects of any type.
//! `deadpool_postgres` implements a `deadpool` manager for `tokio-postgres`
//! and also provides a statement cache by wrapping `tokio_postgres::Client` and
//! `tokio_postgres::Transaction`.
//! Use `pool.status()` to inspect the current size and checkout activity.
//!
//! Add to your `Config.toml`:
//! deadpool = "0.12.1" # or latest version
//! deadpool-postgres = { version = "0.14.1", features = ["serde"] }
//!
//! Add the database URL to your `.env` file, for example:
//! `DATABASE_URL=postgres://john_doe:topsecret@pg.example.com/example`
//! Or set the connection fields individually with `PG__HOST`, `PG__USER`,
//! `PG__PASSWORD`, and `PG__DBNAME`.
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
    #[serde(default)]
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
/// built. When `max_size` is set, it overrides the environment configuration.
fn create_pool_with_max_size(
    max_size: Option<usize>,
) -> anyhow::Result<deadpool_postgres::Pool> {
    // Loads the `.env` file from the current directory or parents.
    dotenv().ok();
    // Hydrate pool and connection options from environment variables.
    let mut cfg = Config::from_env()?;
    // A URL is convenient for local development. Explicit PG__* fields also
    // apply; keep them consistent with the URL when both are configured.
    if let Ok(database_url) = std::env::var("DATABASE_URL") {
        cfg.pg.url = Some(database_url);
    }
    if let Some(max_size) = max_size {
        let mut pool_config = cfg.pg.get_pool_config();
        pool_config.max_size = max_size;
        cfg.pg.pool = Some(pool_config);
    }
    // Creates a new `Pool` using the `deadpool_postgres::Config`.
    Ok(cfg.pg.create_pool(Some(Runtime::Tokio1), NoTls)?)
}

/// Creates a PostgreSQL connection pool using its configured maximum size.
///
/// # Errors
///
/// Returns an error if environment configuration cannot be read or the pool
/// cannot be built.
fn create_pool() -> anyhow::Result<deadpool_postgres::Pool> {
    create_pool_with_max_size(None)
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
    let status = pool.status();
    println!(
        "Pool status: {} available, {} allocated, {} maximum, {} waiting",
        status.available, status.size, status.max_size, status.waiting
    );
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

/// # Errors
///
/// Returns an error if the PostgreSQL connection or query fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use tokio::task::JoinSet;

    use super::*;

    /// Exercises pool checkout and query behavior with many concurrent clients.
    #[tokio::test]
    #[ignore = "requires a reachable PostgreSQL service and PG__* environment variables"]
    async fn serves_concurrent_queries_through_the_pool() -> anyhow::Result<()>
    {
        /// Maximum number of PostgreSQL connections in the test pool.
        const MAX_POOL_SIZE: usize = 8;
        /// Number of requests submitted to the pool.
        const CONCURRENT_QUERIES: usize = 64;

        let pool = create_pool_with_max_size(Some(MAX_POOL_SIZE))?;
        assert_eq!(pool.status().max_size, MAX_POOL_SIZE);

        let active_checkouts = Arc::new(AtomicUsize::new(0));
        let peak_checkouts = Arc::new(AtomicUsize::new(0));
        let mut tasks = JoinSet::new();

        for query_index in 0..CONCURRENT_QUERIES {
            let request_id = i32::try_from(query_index)?;
            let pool = pool.clone();
            let active_checkouts = Arc::clone(&active_checkouts);
            let peak_checkouts = Arc::clone(&peak_checkouts);
            tasks.spawn(async move {
                let client = pool.get().await?;
                let active =
                    active_checkouts.fetch_add(1, Ordering::SeqCst) + 1;
                peak_checkouts.fetch_max(active, Ordering::SeqCst);

                let query_result = client
                    .query_one(
                        "SELECT pg_sleep(0.01), $1::integer",
                        &[&request_id],
                    )
                    .await;
                drop(client);
                active_checkouts.fetch_sub(1, Ordering::SeqCst);

                let row = query_result?;
                Ok::<i32, anyhow::Error>(row.get(1))
            });
        }

        let mut completed_ids = Vec::with_capacity(CONCURRENT_QUERIES);
        while let Some(result) = tasks.join_next().await {
            completed_ids.push(result??);
        }
        completed_ids.sort_unstable();

        let expected_ids = (0..CONCURRENT_QUERIES)
            .map(i32::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(completed_ids, expected_ids);
        assert_eq!(active_checkouts.load(Ordering::SeqCst), 0);
        assert!(
            peak_checkouts.load(Ordering::SeqCst) <= MAX_POOL_SIZE,
            "the pool allowed more simultaneous checkouts than its configured maximum"
        );
        Ok(())
    }

    #[test]
    #[ignore = "needs a running PostgreSQL server"]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
// [finish; need heavy test](https://github.com/john-cd/rust_howto/issues/46)
