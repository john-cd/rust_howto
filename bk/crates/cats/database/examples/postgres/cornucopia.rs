#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates using generated, type-safe PostgreSQL queries
//! with Cornucopia.
//!
//! Cornucopia is a command-line generator. Annotated SQL lives in
//! `cornucopia/queries/greetings.sql`, and its schema is in
//! `cornucopia/schema.sql`. The generated module is included below.
//!
//! Install Cornucopia 0.9 and `psql`, prepare a local database, and generate
//! the module from the repository root with:
//! ```text
//! cargo install --locked --version 0.9.0 cornucopia
//! $env:DATABASE_URL = "postgres://postgres:postgres@localhost/rust_howto"
//! psql $env:DATABASE_URL -f bk/crates/cats/database/examples/postgres/cornucopia/schema.sql
//! cornucopia -q bk/crates/cats/database/examples/postgres/cornucopia/queries -d bk/crates/cats/database/examples/postgres/cornucopia/generated.rs live $env:DATABASE_URL
//! rustup run nightly rustfmt --edition 2021 bk/crates/cats/database/examples/postgres/cornucopia/generated.rs
//! ```
//! Set `DATABASE_URL` to that database before running the example. The book
//! chapter describes how to start the local PostgreSQL service.

mod generated {
    include!("cornucopia/generated.rs");
}

use anyhow::Context;
use generated::queries::greetings::add_greeting;
use generated::queries::greetings::list_greetings;
use tokio_postgres::NoTls;

/// Connects to PostgreSQL, prepares the sample schema, and runs generated
/// queries.
///
/// # Errors
///
/// Returns an error if `DATABASE_URL` is missing, the database is unreachable,
/// or a schema or query operation fails.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .context("set DATABASE_URL to a reachable PostgreSQL database")?;
    let (client, connection) =
        tokio_postgres::connect(&database_url, NoTls).await?;
    let connection_task = tokio::spawn(connection);

    client
        .batch_execute(include_str!("cornucopia/schema.sql"))
        .await
        .context("create the Cornucopia sample table")?;

    add_greeting()
        .bind(&client, &"Hello from Cornucopia!")
        .await
        .context("insert the sample greeting")?;
    let greetings = list_greetings()
        .bind(&client)
        .all()
        .await
        .context("query greetings")?;

    for greeting in greetings {
        println!("{greeting}");
    }

    drop(client);
    connection_task
        .await
        .context("join the PostgreSQL connection task")??;
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
///
/// # Errors
///
/// Returns an error if PostgreSQL is unavailable or a generated query fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generated greeting queries work against a configured local database.
    #[test]
    #[ignore = "requires PostgreSQL; set DATABASE_URL and prepare a local database"]
    fn generated_greetings_run_on_postgres() -> anyhow::Result<()> {
        main()
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/708)
