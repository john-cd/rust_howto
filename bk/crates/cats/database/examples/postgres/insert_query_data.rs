// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates inserting rows into PostgreSQL and querying the
//! inserted data back.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use anyhow::Context as _;
use postgres::Client;
use postgres::NoTls;
use postgres::types::ToSql;

/// Represents an author with an ID, name, and country.
struct Author {
    /// The identifier.
    _id: i32,
    /// The name.
    name: String,
    /// The country.
    country: String,
}

/// Inserts sample authors into a temporary table, then queries them back.
///
/// # Errors
///
/// Returns an error if `DATABASE_URL` is missing, PostgreSQL cannot be reached,
/// or an insert or query fails.
fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .context("set DATABASE_URL to connect to PostgreSQL")?;
    let mut client = Client::connect(&database_url, NoTls)
        .context("connect to PostgreSQL using DATABASE_URL")?;

    // A temporary table keeps this example independent of `create_tables` and
    // prevents sample rows from accumulating in a shared database.
    client
        .batch_execute(
            "CREATE TEMP TABLE author (
                id SERIAL PRIMARY KEY,
                name VARCHAR NOT NULL,
                country VARCHAR NOT NULL
            )",
        )
        .context("create a temporary author table")?;

    // A BTreeMap keeps the generated bulk insert and its output deterministic.
    let mut authors = BTreeMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    // Bulk insert the authors into the database to prevent N+1 queries.
    // Build the query string and the parameters vector.
    if !authors.is_empty() {
        let mut query =
            String::from("INSERT INTO author (name, country) VALUES ");
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        for (i, (key, value)) in authors.iter().enumerate() {
            if i > 0 {
                query.push_str(", ");
            }
            // Add parameter placeholders ($1, $2), ($3, $4), etc.
            write!(query, "(${}, ${})", i * 2 + 1, i * 2 + 2)
                .context("format the bulk insert query")?;
            params.push(key);
            params.push(value);
        }

        client
            .execute(&query, &params)
            .context("insert sample authors")?;
    }

    // Query the database to retrieve all authors and print their details.
    // The query returns rows, and we iterate over them to extract author
    // information.
    for row in client
        .query("SELECT id, name, country FROM author ORDER BY name", &[])
        .context("query sample authors")?
    {
        let author = Author {
            _id: row.try_get(0)?,
            name: row.try_get(1)?,
            country: row.try_get(2)?,
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())
}
// ANCHOR_END: example

/// # Errors
///
/// Returns an error if PostgreSQL cannot be reached or the sample query fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example inserts and reads back its sample authors on PostgreSQL.
    #[test]
    #[ignore = "needs Postgres"]
    fn inserts_and_queries_authors_on_postgres() -> anyhow::Result<()> {
        main()
    }
}
