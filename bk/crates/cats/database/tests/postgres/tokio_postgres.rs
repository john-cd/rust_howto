#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Connecting to PostgreSQL with `tokio-postgres`.

use anyhow::Context as _;
use anyhow::ensure;
use tokio_postgres::Client;

/// In this example, we:
///
/// - Connect to a PostgreSQL database using `tokio_postgres::connect`.
/// - Spawn the connection on a separate task using `tokio::spawn`.
/// - Create a temporary table for this connection.
/// - Insert, query, update, and delete one row.
/// - Wait for the connection task to finish after closing the client.
pub async fn run() -> anyhow::Result<()> {
    let url = std::env::var("PG_URL").context("PG_URL must be set")?;
    run_with_url(&url).await
}

/// Runs the example against the database at `url`.
pub(crate) async fn run_with_url(url: &str) -> anyhow::Result<()> {
    // The libpq-style connection strings consist of space-separated
    // key-value pairs: <https://docs.rs/tokio-postgres/latest/tokio_postgres/config/struct.Config.html>.
    let (client, connection) =
        tokio_postgres::connect(url, tokio_postgres::NoTls).await?;

    // The connection object performs the actual communication with the
    // database.
    let connection_task = tokio::spawn(connection);
    let result = run_queries(&client).await;
    drop(client);

    // Dropping the last client closes the connection. Wait for its driver so
    // the example does not leave a detached task behind.
    match connection_task.await? {
        Ok(()) => {}
        Err(error) if error.is_closed() => {}
        Err(error) => return Err(error.into()),
    }

    result
}

/// Creates a temporary table, then inserts, queries, updates and deletes a row.
async fn run_queries(client: &Client) -> anyhow::Result<()> {
    // A temporary table keeps repeated runs isolated and is removed when the
    // connection closes.
    client
        .batch_execute(
            "CREATE TEMP TABLE person (
                id SERIAL PRIMARY KEY,
                name VARCHAR NOT NULL,
                data BYTEA
            )",
        )
        .await?;

    let id: i32 = client
        .query_one(
            "INSERT INTO person (name, data) VALUES ($1, $2) RETURNING id",
            &[&"John Doe", &Some(b"example data".to_vec())],
        )
        .await?
        .get(0);

    let row = client
        .query_one("SELECT name, data FROM person WHERE id = $1", &[&id])
        .await?;
    let name: String = row.get(0);
    let data: Option<Vec<u8>> = row.get(1);
    ensure!(name == "John Doe", "inserted name was not preserved");
    ensure!(
        data.as_deref() == Some(&b"example data"[..]),
        "inserted byte data was not preserved"
    );
    println!("Found person: {name} with id: {id}");
    if let Some(data) = data {
        println!("Data: {:?}", String::from_utf8(data));
    }

    // Calling a method like `Client::query` has no effect until its future is
    // polled. Requests execute in the order their futures are first polled.
    let updated = client
        .execute(
            "UPDATE person SET name = $1 WHERE id = $2",
            &[&"Jane Doe", &id],
        )
        .await?;
    ensure!(
        updated == 1,
        "expected one row to be updated, got {updated}"
    );
    let updated_name: String = client
        .query_one("SELECT name FROM person WHERE id = $1", &[&id])
        .await?
        .get(0);
    ensure!(updated_name == "Jane Doe", "updated name was not persisted");

    let deleted = client
        .execute("DELETE FROM person WHERE id = $1", &[&id])
        .await?;
    ensure!(
        deleted == 1,
        "expected one row to be deleted, got {deleted}"
    );
    ensure!(
        client
            .query_opt("SELECT id FROM person WHERE id = $1", &[&id])
            .await?
            .is_none(),
        "deleted row is still present"
    );

    Ok(())
}

/// Runs the tokio postgres example.
pub fn main() -> anyhow::Result<()> {
    tokio::runtime::Runtime::new()?.block_on(run())
}
// ANCHOR_END: example
