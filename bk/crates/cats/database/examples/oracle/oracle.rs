#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Connect to Oracle, query user tables with bind variables, and print rows.
//!
//! Set `ORACLE_USER`, `ORACLE_PASSWORD`, and `ORACLE_CONNECT_STRING` in the
//! environment. The optional integration test also requires an Oracle
//! database and Oracle Instant Client; the chapter documents how to run it.

use std::env;

use anyhow::Context as _;
use oracle::Connection;

/// Connects to Oracle and prints up to five tables matching `R%`.
fn main() -> anyhow::Result<()> {
    let username = env::var("ORACLE_USER")
        .context("set ORACLE_USER to an Oracle database user")?;
    let password = env::var("ORACLE_PASSWORD")
        .context("set ORACLE_PASSWORD to the user's password")?;
    let connect_string = env::var("ORACLE_CONNECT_STRING")
        .context("set ORACLE_CONNECT_STRING to host:port/service_name")?;

    let connection =
        Connection::connect(&username, &password, &connect_string)?;
    let mut statement = connection
        .statement(
            "SELECT table_name FROM user_tables \
             WHERE table_name LIKE :1 AND ROWNUM <= :2",
        )
        .build()?;
    let table_name_pattern = "R%";
    let row_limit = 5;
    let rows = statement.query(&[&table_name_pattern, &row_limit])?;

    for row_result in rows {
        let row = row_result?;
        let table_name: String = row.get("table_name")?;
        println!("Table: {table_name}");
    }

    connection.close()?;
    Ok(())
}

// ANCHOR_END: example

/// Runs the example and reports missing configuration or Oracle errors.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Connects to Oracle and exercises the parameterized table query.
    #[test]
    #[ignore = "requires Oracle Free, Instant Client, and ORACLE_* settings"]
    fn require_external_svc() -> anyhow::Result<()> {
        main()
    }
}
