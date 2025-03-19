// ANCHOR: example

// In this example, we
// - connect to a PostgreSQL database using `tokio_postgres::connect`,
// - spawn the connection on a separate task using `tokio::spawn`,
// - create a table if it does not already exist,
// - insert a new row into the table,
// - query the table for rows and print the results,
// - update a row in the table,
// - delete a row from the table,

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
    // Connect to the database.
    //  The libpq-style connection strings consist of space-separated
    // key-value pairs: https://docs.rs/tokio-postgres/latest/tokio_postgres/config/struct.Config.html
    let (client, connection) = tokio_postgres::connect(
        "host=rust_howto_dev-postgres-1 user=postgres password=mysecretpassword dbname=library",
        tokio_postgres::NoTls,
    )
    .await?;

    // Spawn the connection on a separate task.
    // The connection object performs the actual communication with the
    // database.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Create a table
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS person (
                id SERIAL PRIMARY KEY,
                name VARCHAR NOT NULL,
                data BYTEA
            )",
            &[],
        )
        .await?;

    // Insert a new row
    client
        .execute(
            "INSERT INTO person (name, data) VALUES ($1, $2)",
            &[&"John Doe", &Some(b"example data".to_vec())],
        )
        .await?;

    // Query the rows
    let mut id: i32 = 0;
    for row in client
        .query("SELECT id, name, data FROM person", &[])
        .await?
    {
        id = row.get(0);
        let name: &str = row.get(1);
        let data: Option<Vec<u8>> = row.get(2);

        println!("Found person: {} with id: {}", name, id);
        if let Some(data) = data {
            println!("Data: {:?}", String::from_utf8(data));
        }
    }
    // Note: calling a method like Client::query on its own does nothing.
    // The associated request is not sent to the database until the future
    // returned by the method is first polled (awaited). Requests are
    // executed in the order that they are first polled, not in the order that
    // their futures are created.

    // Update a row
    client
        .execute(
            "UPDATE person SET name = $1 WHERE id = $2",
            &[&"Jane Doe", &id],
        )
        .await?;

    // Delete a row
    client
        .execute("DELETE FROM person WHERE id = $1", &[&id])
        .await?;

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [WIP finish SOON](https://github.com/john-cd/rust_howto/issues/719) need heay test
