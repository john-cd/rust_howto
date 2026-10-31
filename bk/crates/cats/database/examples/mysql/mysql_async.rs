// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Connects to MySQL or MariaDB through a pool and runs parameterized queries.
//!
//! A connection pool keeps a few database connections open and lends them out,
//! because opening a connection for each request is slow. Parameterized queries
//! send values separately from the SQL text, which prevents SQL injection.

use anyhow::Context;
use anyhow::Result;
use mysql_async::Pool;
use mysql_async::params;
use mysql_async::prelude::*;

/// Connects with a pool, binds parameters, and checks the returned values.
#[tokio::main]
async fn main() -> Result<()> {
    // ANCHOR: connect
    // For example `mysql://user:password@localhost:3306/db_name`.
    let url = std::env::var("MYSQL_URL")
        .context("set MYSQL_URL to the MySQL connection URL")?;
    let pool = Pool::new(url.as_str());
    let mut conn = pool.get_conn().await?;
    // ANCHOR_END: connect

    // ANCHOR: query
    // `?` placeholders are bound by the server, never spliced into the SQL
    // text. A tuple supplies positional parameters.
    let answer: i32 = conn
        .exec_first("SELECT ? + 1", (41_i32,))
        .await?
        .context("MySQL returned no row")?;
    anyhow::ensure!(answer == 42, "unexpected query result: {answer}");

    // `params!` binds named parameters.
    let greeting: String = conn
        .exec_first(
            "SELECT CONCAT('Hello, ', :name)",
            params! { "name" => "MySQL" },
        )
        .await?
        .context("MySQL returned no row")?;
    println!("{answer}, {greeting}");
    // ANCHOR_END: query

    // Return the connection to the pool, then close the pool's connections.
    drop(conn);
    pool.disconnect().await?;
    Ok(())
}
// ANCHOR_END: example

/// Runs the MySQL connection and query workflow.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies parameterized queries against a configured MySQL server.
    #[test]
    #[ignore = "needs MySQL or MariaDB"]
    fn parameterized_queries_return_the_expected_values() -> Result<()> {
        main()
    }
}
