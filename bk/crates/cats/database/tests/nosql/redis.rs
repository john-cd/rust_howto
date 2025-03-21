// ANCHOR: example
use std::env;

use anyhow::Context;
use anyhow::Result;
use redis::Commands;
use redis::Connection;

fn connect() -> Result<Connection> {
    let redis_host_name = env::var("REDIS_HOSTNAME")
        .context("missing environment variable REDIS_HOSTNAME")?;
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();
    // Does Redis server need a secure connection?
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };
    // The URL format is
    // protocol=<protocol>]] For example, "redis://127.0.0.1/"
    let redis_conn_url =
        format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    // `open` does not actually open a connection yet, but it does perform some
    // basic checks on the URL that might make the operation fail.
    Ok(redis::Client::open(redis_conn_url)?.get_connection()?)
}

fn fetch_an_integer() -> Result<isize> {
    let mut con = connect()?;
    // Throw away the result, just make sure it does not fail
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
// TODO WIP review NOW
// redis://[<username>][:<password>@]<hostname>[:port][/[<db>][?
