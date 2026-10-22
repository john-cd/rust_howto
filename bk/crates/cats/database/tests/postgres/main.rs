//! Runs the `tokio-postgres` example as an integration test.
//!
//! TODO Reviewed ??-??-2026

#[cfg(feature = "postgres")]
mod tokio_postgres;

/// Runs the example; needs a PostgreSQL server.
#[cfg(feature = "postgres")]
fn main() -> anyhow::Result<()> {
    tokio_postgres::main()?;
    Ok(())
}

/// Runs the example against the server at `TEST_PG_URL`, skipping if it is
/// unset.
#[cfg(feature = "postgres")]
#[test]
fn require_external_svc() -> anyhow::Result<()> {
    let test_url = match std::env::var("TEST_PG_URL") {
        Ok(url) => url,
        Err(_) if std::env::var_os("REQUIRE_PG_SERVICE").is_some() => {
            anyhow::bail!(
                "REQUIRE_PG_SERVICE is set but TEST_PG_URL is missing"
            );
        }
        Err(_) => {
            eprintln!(
                "Skipping PostgreSQL integration test; set TEST_PG_URL to run it."
            );
            return Ok(());
        }
    };
    tokio::runtime::Runtime::new()?
        .block_on(tokio_postgres::run_with_url(&test_url))
}

/// Does nothing when the `postgres` feature is disabled.
#[cfg(not(feature = "postgres"))]
fn main() {}
