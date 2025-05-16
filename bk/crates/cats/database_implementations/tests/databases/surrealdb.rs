#![cfg(feature = "surrealdb")]
// ANCHOR: example
#![allow(clippy::result_large_err)]
use serde::Deserialize;
use serde::Serialize;
use surrealdb::Result;
// Database client instance for embedded or remote databases:
use surrealdb::Surreal;
// For an in-memory database, use:
use surrealdb::engine::local::Mem;
// For a RocksDB file, use:
// use surrealdb::engine::local::RocksDb;

// The document to store in the database.
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String, // or: Cow<'static, str>
    age: u16,
}

/// SurrealDB is a scalable, distributed, collaborative,
/// document-graph database for the realtime web.
///
/// The `surrealdb` crate can be used to start an embedded in-memory
/// datastore, an embedded datastore persisted to disk,
/// a browser-based embedded datastore backed by IndexedDB,
/// or for connecting to a distributed TiKV key-value store.
///
/// Add `surrealdb = { version = "2.1.4", features = [ "kv-mem" ] }`
/// to `Cargo.toml` for an in-memory datastore.
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize a SurrealDB in-memory instance.
    let db = Surreal::new::<Mem>(()).await?;

    // Alternatively, connect to a local endpoint:
    // let db = Surreal::new::<Ws>("localhost:8000").await?;
    // Or create database connection using RocksDB:
    // let db = Surreal::new::<RocksDb>("path/to/database-folder").await?;

    // Select a namespace and database.
    db.use_ns("test_namespace").use_db("test_db").await?;

    // Create a record with a specific ID.
    let created: Option<Person> = db
        .create(("person", "John Doe"))
        .content(Person {
            name: "J. Doe".to_string(),
            age: 42,
        })
        .await?;

    println!("Created person: {:?}", created);

    // Select all person records.
    let people: Vec<Person> = db.select("person").await?;
    println!("All people: {:?}", people);

    // Find a specific person by name using a WHERE clause.
    // Demonstrates how to use `query()` with bindings to prevent SQL injection
    // vulnerabilities. Note that the `query()` method is able to hold more
    // than one statement.
    let mut result = db
        .query("SELECT * FROM person WHERE name = $name")
        .bind(("name", "J. Doe"))
        .await?;

    let people: Vec<Person> = result.take(0)?;
    println!("Specific person: {:?}", people);

    // Update a person's age.
    let _updated: Option<Person> = db.update(("person", "John Doe"))
    // Assumes that the name is unique for simplicity. In real applications, use a proper ID.
        .content(Person {
            name: "J. Doe".to_string(),
            age: 43,
        })
        .await?;

    // Delete a person.
    let deleted: Option<Person> = db.delete(("person", "John Doe")).await?;
    println!("Deleted person: {:?}", deleted);

    // Check if the person exists after deletion.
    let people_after_delete: Vec<Person> = db.select("person").await?;
    println!("People after delete: {:?}", people_after_delete);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
// [review https://surrealdb.com/docs/ NOW](https://github.com/john-cd/rust_howto/issues/1148)
