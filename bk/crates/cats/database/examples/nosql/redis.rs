#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Setting and getting a value in Redis.

// This example demonstrates connecting to Redis and executing commands using
// the `redis` crate.
use std::env;

use anyhow::Result;
use redis::Commands as _;
use redis::Connection;

/// Establishes a connection to a Redis server.
///
/// This function reads environment variables to determine the Redis server's
/// hostname, password, and whether to use a secure connection (TLS). It then
/// constructs a connection URL and attempts to connect to the server.
///
/// # Returns
///
/// A `Result` containing a `Connection` to the Redis server on success, or an
/// error if the connection fails.
fn connect() -> Result<Connection> {
    let redis_host_name = env::var("REDIS_HOSTNAME")
        .unwrap_or_else(|_| "127.0.0.1:6379".to_owned());
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();
    // Does Redis server need a secure connection?
    let uri_scheme = if tls_enabled(env::var("IS_TLS").ok().as_deref()) {
        "rediss"
    } else {
        "redis"
    };
    let redis_conn_url =
        format!("{uri_scheme}://:{redis_password}@{redis_host_name}");
    // `open` does not actually open a connection yet, but it does perform some
    // basic checks on the URL that might make the operation fail.
    Ok(redis::Client::open(redis_conn_url)?.get_connection()?)
}

/// Enables TLS only for an explicit true value such as `IS_TLS=true`.
fn tls_enabled(value: Option<&str>) -> bool {
    value.is_some_and(|value| value.eq_ignore_ascii_case("true"))
}

/// Fetches an integer value from Redis.
///
/// This function connects to a Redis server, sets a key-value pair ("`my_key`",
/// 42), and then retrieves the value associated with "`my_key`".
///
/// # Returns
///
/// A `Result` containing the integer value retrieved from Redis on success, or
/// an error if the operation fails.
fn fetch_an_integer() -> Result<isize> {
    let mut con = connect()?;
    // Throw away the result, just make sure it does not fail.
    let _: () = con.set("my_key", 42)?;
    // Read back the key and return it. Because the return value
    // from the function is a result for integer, this will automatically
    // convert into one.
    Ok(con.get("my_key")?)
}

/// Prints the integer fetched from Redis.
fn main() -> Result<()> {
    let my_int = fetch_an_integer()?;
    println!("{my_int}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that TLS is enabled only for an explicit `true`.
    #[test]
    fn tls_requires_an_explicit_true_value() {
        assert!(!tls_enabled(None));
        assert!(!tls_enabled(Some("false")));
        assert!(!tls_enabled(Some("0")));
        assert!(tls_enabled(Some("true")));
        assert!(tls_enabled(Some("TRUE")));
    }

    /// Verifies that the Redis example stores and reads the expected value.
    #[test]
    #[ignore = "needs a reachable Redis server"]
    fn require_external_svc() -> Result<()> {
        let value = fetch_an_integer()?;
        anyhow::ensure!(value == 42, "Redis returned {value}, expected 42");
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
// review?
