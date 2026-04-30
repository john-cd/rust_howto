#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates creating database tables in PostgreSQL using the
//! `postgres` crate.

use postgres::Client;
use postgres::NoTls;

/// This function creates two tables in a PostgreSQL database: `author` and
/// `book`.
///
/// The `author` table has columns for `id`, `name`, and `country`.
/// The `book` table has columns for `id`, `title`, and `author_id` (a foreign
/// key referencing the `author` table).
fn main() -> anyhow::Result<()> {
    // Establish a connection to the PostgreSQL database.
    // The connection URL format is
    // `postgresql://<user>:<password>@<host>/<db>`.
    let url = std::env::var("PG_URL").expect("PG_URL must be set");
    let mut client = Client::connect(&url, NoTls)?;

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
    println!("Tables created!");
    Ok(())
}
// ANCHOR_END: example

pub fn run() {
    main();
}
