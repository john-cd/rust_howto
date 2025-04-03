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
//! Add you database's configuration to your `.env` file, for example:
//! PG__HOST=pg.example.com
//! PG__USER=john_doe
//! PG__PASSWORD=topsecret
//! PG__DBNAME=example
//! PG__POOL__MAX_SIZE=16
//! PG__POOL__TIMEOUTS__WAIT__SECS=5
//! PG__POOL__TIMEOUTS__WAIT__NANOS=0

use deadpool_postgres::Runtime;
use dotenvy::dotenv;
use tokio_postgres::NoTls;

/// Configuration for the application.
#[derive(Debug, serde::Deserialize)]
struct Config {
    pub pg: deadpool_postgres::Config,
}

impl Config {
    // With the `serde` feature enabled, we can read the configuration using the
    // `config` crate.
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Loads the `.env` file from the current directory or parents.
    dotenv().ok();
    // Hydrate the configuration from environment variables.
    let cfg = Config::from_env()?;
    // Creates a new `Pool` using the `deadpool_postgres::Config`.
    let pool = cfg.pg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    // Retrieves an Object from this Pool or waits for one to become available
    let client = pool.get().await?;
    // Like `tokio_postgres::Client::prepare()`, but uses an existing
    // `Statement` from the `StatementCache` if possible
    let stmt = client.prepare_cached("SELECT version()").await?;
    let rows = client.query(&stmt, &[]).await?;

    for row in rows {
        let version: &str = row.get(0);
        println!("PostgreSQL version: {}", version);
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [finish; need heavy test](https://github.com/john-cd/rust_howto/issues/46)
