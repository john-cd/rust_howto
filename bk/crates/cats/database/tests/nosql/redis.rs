// ANCHOR: example
use std::env;

use anyhow::Context;
use anyhow::Result;
use redis::Commands;
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
        .context("missing environment variable REDIS_HOSTNAME")?;
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();
    // Does Redis server need a secure connection?
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };
    let redis_conn_url =
        format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    // `open` does not actually open a connection yet, but it does perform some
    // basic checks on the URL that might make the operation fail.
    Ok(redis::Client::open(redis_conn_url)?.get_connection()?)
}

/// Fetches an integer value from Redis.
///
/// This function connects to a Redis server, sets a key-value pair ("my_key",
/// 42), and then retrieves the value associated with "my_key".
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

fn main() -> Result<()> {
    let my_int = fetch_an_integer()?;
    println!("{}", my_int);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() -> Result<()> {
    unsafe {
        // container name = $COMPOSE_PROJECT_NAME + service name + number
        env::set_var("REDIS_HOSTNAME", "rust_howto_dev-redis-1");
    }
    main()?;
    Ok(())
}
// [review NOW](https://github.com/john-cd/rust_howto/issues/1161)?
