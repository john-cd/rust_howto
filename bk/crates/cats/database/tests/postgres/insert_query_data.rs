// ANCHOR: example
use std::collections::HashMap;

use postgres::Client;
use postgres::Error;
use postgres::NoTls;

/// Represents an author with an ID, name, and country.
struct Author {
    _id: i32,
    name: String,
    country: String,
}

pub fn main() -> Result<(), Error> {
    // Connect to the PostgreSQL database using the provided connection string.
    // The connection URL is formatted as
    // postgresql://<user>:<password>@<host>/<db>,
    // for example postgresql://postgres:postgres@localhost/library
    let mut client = Client::connect(
        "postgresql://postgres:mysecretpassword@rust_howto_dev-postgres-1/library",
        NoTls,
    )?;

    // Create a HashMap to store author names and their respective countries.
    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    // Iterate over the authors HashMap and insert each author into the
    // database. For each author, create an Author struct and execute an SQL
    // INSERT query. The query uses parameterized values ($1, $2) to prevent
    // SQL injection.
    for (key, value) in &authors {
        let author = Author {
            _id: 0,
            name: key.to_string(),
            country: value.to_string(),
        };

        client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
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
