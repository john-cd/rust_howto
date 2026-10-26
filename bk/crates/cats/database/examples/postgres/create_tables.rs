#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates creating database tables in PostgreSQL using the
//! `postgres` crate.

use anyhow::Context as _;
use postgres::Client;
use postgres::NoTls;

/// This function creates two tables in a PostgreSQL database: `author` and
/// `book`.
///
/// The `author` table has columns for `id`, `name`, and `country`.
/// The `book` table has columns for `id`, `title`, and `author_id` (a foreign
/// key referencing the `author` table).
///
/// # Errors
///
/// Returns an error if `DATABASE_URL` is missing, PostgreSQL cannot be reached,
/// or either table cannot be created.
fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .context("set DATABASE_URL to connect to PostgreSQL")?;
    let mut client = Client::connect(&database_url, NoTls)
        .context("connect to PostgreSQL using DATABASE_URL")?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS author (
            -- Primary key for the author table.
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book  (
            -- Primary key for the book table.
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            -- Foreign key referencing the author table.
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ",
    )?;
    println!("Created the author and book tables.");
    Ok(())
}
// ANCHOR_END: example

/// # Errors
///
/// Returns an error if PostgreSQL cannot be reached or the tables cannot be
/// created.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example creates its tables in the configured PostgreSQL database.
    #[test]
    #[ignore = "needs Postgres"]
    fn creates_tables_on_postgres() -> anyhow::Result<()> {
        main()
    }
}
