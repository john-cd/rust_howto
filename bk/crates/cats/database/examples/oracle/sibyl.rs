#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Connects to Oracle with Sibyl's blocking API and reads a value from `DUAL`.

/// Connects to Oracle with Sibyl's blocking API and reads a value from `DUAL`.
///
/// Set `ORACLE_CONNECT_STRING`, `ORACLE_USERNAME`, and `ORACLE_PASSWORD` before
/// running this example. The connect string is typically
/// `host:1521/FREEPDB1` for Oracle Free.
///
/// # Errors
///
/// Returns an error if required settings are missing, the client cannot load,
/// or connecting to and querying the database fails.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "sibyl")]
    {
        use anyhow::Context as _;
        use sibyl as oracle;
        let connect_string = std::env::var("ORACLE_CONNECT_STRING")
            .context("set ORACLE_CONNECT_STRING to host:port/service_name")?;
        let username = std::env::var("ORACLE_USERNAME")
            .context("set ORACLE_USERNAME to a database user")?;
        let password = std::env::var("ORACLE_PASSWORD")
            .context("set ORACLE_PASSWORD to the database user's password")?;

        let environment =
            oracle::env().context("load the Oracle client library")?;
        let session = environment
            .connect(&connect_string, &username, &password)
            .context("connect to Oracle")?;
        let statement = session
            .prepare("SELECT 'Sibyl connected' FROM dual")
            .context("prepare the connection check")?;
        let rows = statement.query(()).context("run the connection check")?;

        if let Some(row) = rows.next().context("read the connection check")? {
            let message: Option<&str> =
                row.get(0).context("read the result value")?;
            if let Some(message) = message {
                println!("{message}");
            }
        }
    }

    Ok(())
}

// ANCHOR_END: example

/// Runs the Sibyl Oracle example.
///
/// # Errors
///
/// Returns an error when client setup, connection, or query execution fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Connects to Oracle with the configured Instant Client and credentials.
    #[test]
    #[ignore = "requires Oracle Instant Client and a reachable Oracle service"]
    fn connects_to_oracle_with_sibyl() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
