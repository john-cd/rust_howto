// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Connects to SQL Server with explicit credentials and runs a parameterized
//! query.

use anyhow::Context;
use anyhow::Result;
use tiberius::AuthMethod;
use tiberius::Client;
use tiberius::Config;
use tiberius::EncryptionLevel;
use tokio_util::compat::TokioAsyncWriteCompatExt as _;

/// Connects with TLS, binds a query parameter, and checks the returned value.
#[tokio::main]
async fn main() -> Result<()> {
    // ANCHOR: connect
    let host = std::env::var("MSSQL_HOST")
        .context("set MSSQL_HOST to the SQL Server host")?;
    let user = std::env::var("MSSQL_USER")
        .context("set MSSQL_USER to the SQL login")?;
    let password = std::env::var("MSSQL_PASSWORD")
        .context("set MSSQL_PASSWORD to the SQL login password")?;
    let mut config = Config::new();
    config.host(host);
    config.port(1433);
    config.database("master");
    config.authentication(AuthMethod::sql_server(user, password));
    config.encryption(EncryptionLevel::Required);
    // Local development only: opt in for a self-signed test server.
    if std::env::var("MSSQL_TRUST_CERT").as_deref() == Ok("1") {
        config.trust_cert();
    }
    let socket = tokio::net::TcpStream::connect(config.get_addr()).await?;
    socket.set_nodelay(true)?;
    let mut client = Client::connect(config, socket.compat_write()).await?;
    // ANCHOR_END: connect

    // ANCHOR: query
    let value = 41_i32;
    let row = client
        .query("SELECT @P1 + 1 AS answer", &[&value])
        .await?
        .into_row()
        .await?
        .context("SQL Server returned no row")?;
    let answer: i32 = row
        .try_get("answer")?
        .context("the answer column was NULL")?;
    anyhow::ensure!(answer == 42, "unexpected query result: {answer}");
    println!("Query result: {answer}");
    // ANCHOR_END: query
    Ok(())
}
// ANCHOR_END: example

/// Runs the SQL Server connection and query workflow.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies an actual parameterized query against a configured SQL Server.
    #[test]
    #[ignore = "needs SQL Server"]
    fn parameterized_query_returns_the_expected_value() -> Result<()> {
        main()
    }
}
