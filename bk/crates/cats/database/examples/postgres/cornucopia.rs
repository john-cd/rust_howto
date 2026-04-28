#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates type-safe query generation for PostgreSQL
//! with Cornucopia.
//!
//! Cornucopia is a Rust library for managing database connections and
//! executing queries. It provides type-safe SQL in Rust.
//! <https://cornucopia-rs.netlify.app/>
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! tokio = { version = "1", features = ["full"] }
//! cornucopia = "0.3.0"
//! cornucopia_async = "0.3.0"
//! postgres = { version = "0.19", features = ["tokio-runtime"] }
//! ```

use tokio_postgres::Error;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=your_user dbname=your_db",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {e}");
        }
    });

    // In a real Cornucopia workflow, generated SQL bindings replace raw
    // query text with type-safe Rust functions.
    let rows = client.query("SELECT message FROM greetings", &[]).await?;

    for row in rows {
        let message: &str = row.get(0);
        println!("{message}");
    }

    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn require_external_svc() {
        main().unwrap();
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/708)
