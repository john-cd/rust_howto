#![allow(dead_code)]
// ANCHOR: example
use std::env;

use dotenvy::dotenv;
use oracle::Connection;
use oracle::Error;

/// Rust bindings to ODPI-C.
///
/// This example demonstrates how to connect to an Oracle database,
/// execute a query, and iterate over the results.
///
/// Add to your `.env` file:
/// ORACLE_DB_USERNAME=your_username
/// ORACLE_DB_PASSWORD=your_password
/// ORACLE_DB_URL=your_db_host:port/your_service_name
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from a .env file (for secure handling of
    // credentials).
    dotenv().ok();

    // Fetch the connection details from environment variables.
    let username =
        env::var("ORACLE_DB_USERNAME").expect("ORACLE_DB_USERNAME not set");
    let password =
        env::var("ORACLE_DB_PASSWORD").expect("ORACLE_DB_PASSWORD not set");
    let db_url = env::var("ORACLE_DB_URL").expect("ORACLE_DB_URL not set");

    // Connect to the Oracle database.
    let conn = Connection::connect(&username, &password, &db_url)?;

    // Query the database (example: retrieve the database name;
    // user_tables is a common system table in Oracle).
    let sql = "SELECT * FROM user_tables WHERE ROWNUM <= 5";
    let mut stmt = conn.statement(sql).build()?;

    // Execute the query and fetch results.
    let rows = stmt.query(&[])?;

    // Iterate over the rows and print the results.
    for row in rows {
        let row = row?;
        // Print each column's value
        // (assuming the result has one column, adjust as needed).
        println!("Table: {}", row.get::<_, String>(0)?);
    }

    conn.close()?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    unsafe {
        // Refer to the compose*.yaml files.
        std::env::set_var("ORACLE_DB_USERNAME", "sysdba");
        std::env::set_var("ORACLE_DB_PASSWORD", "Oracle_123");
        std::env::set_var(
            "ORACLE_DB_URL",
            "rust_howto_dev-oracle-1:1521/ORCLCDB",
        );
    }
    main()?;
    Ok(())
}
// [finish; need to fix heavy test](https://github.com/john-cd/rust_howto/issues/1021)
// <https://odpi-c.readthedocs.io/en/latest/user_guide/installation.html>
