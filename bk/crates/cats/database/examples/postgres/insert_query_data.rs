#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates inserting rows into PostgreSQL and querying the
//! inserted data back.
use std::collections::HashMap;
use std::fmt::Write;

use postgres::Client;
use postgres::Error;
use postgres::NoTls;
use postgres::types::ToSql;

/// Represents an author with an ID, name, and country.
struct Author {
    _id: i32,
    name: String,
    country: String,
}

fn main() -> Result<(), Error> {
    // Connect to the PostgreSQL database using the provided connection string.
    // The connection URL is formatted as
    // postgresql://<user>:<password>@<host>/<db>,
    // for example postgresql://postgres:postgres@localhost/library
    let url = std::env::var("PG_URL").expect("PG_URL must be set");
    let mut client = Client::connect(&url, NoTls)?;

    // Create a HashMap to store author names and their respective countries.
    let mut authors = HashMap::new();
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
            let _ = write!(query, "(${}, ${})", i * 2 + 1, i * 2 + 2);
            params.push(key);
            params.push(value);
        }

        client.execute(&query, &params[..])?;
    }

    // Query the database to retrieve all authors and print their details.
    // The query returns rows, and we iterate over them to extract author
    // information.
    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<(), Error> {
    main()
}
