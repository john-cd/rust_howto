// // ANCHOR: example
// use serde::Deserialize;
// use serde::Serialize;
// use surrealdb::RecordId;
// use surrealdb::Result;
// use surrealdb::Surreal;
// // For an in-memory database
// use surrealdb::engine::local::Mem;
// use tokio;
// // For a RocksDB file
// // use surrealdb::engine::local::RocksDb;

// // SurrealDB is a scalable, distributed, collaborative, document-graph
// database // for the realtime web.

// #[derive(Debug, Serialize, Deserialize)]
// struct Person {
//     name: String,
//     age: u16,
// }

// #[derive(Debug, Deserialize)]
// struct Record {
//     #[allow(dead_code)]
//     id: RecordId,
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     // Initialize the SurrealDB instance.
//     // The `surrealdb` crate can be used to start an embedded in-memory
//     // datastore, an embedded datastore persisted to disk,
//     // a browser-based embedded datastore backed by IndexedDB,
//     // or for connecting to a distributed TiKV key-value store.
//     // We added `surrealdb = { version = "2.1.4", features = [ "kv-mem" ] }`
//     // to `Cargo.toml` to get an in-memory datastore:
//     let db = Surreal::new::<Mem>(()).await?;

//     // Create database connection using RocksDB
//     // let db = Surreal::new::<RocksDb>("path/to/database-folder").await?;

//     // Select a namespace and database.
//     db.use_ns("test").use_db("test").await?;

//     // Create a record with a random ID
//     let person: Option<Person> = db.create("person").await?;

//     // Uses the content() method for creating records
//     let created: Option<Person> = db
//         .create(("person", "john"))
//         .content(Person {
//             name: "John Doe".to_string(),
//             age: 42,
//         })
//         .await?;

//     println!("Created person: {:?}", created);

//     // Select all person records.
//     let people: Vec<Person> = db.select("person").await?;
//     println!("All people: {:?}", people);

//     // Find a specific person by name using a WHERE clause.
//     // Demonstrates how to use query() with bindings to prevent SQL injection
//     // vulnerabilities. Note that the .query() method is able to hold more
//     // than one statement.
//     let specific_person = db
//         .query("SELECT * FROM person WHERE name = $name")
//         .bind(("name", "John Doe"))
//         .await?;
//     println!("Specific person: {:?}", specific_person);

//     // Update a person's age.
//     db
//         .update(("person", "John Doe".to_string())) // Assumes name is unique
// for simplicity. In real applications, use a proper ID.
//         .content(Person {
//             name: "John Doe".to_string(),
//             age: 43,
//         })
//         .await?;

//     // Delete a person.
//     let deleted: Option<Person> = db.delete(("person", "John Doe")).await?;
//     println!("Deleted person: {:?}", deleted);

//     // Check if the person exists after deletion.
//     let people_after_delete: Vec<Person> = db.select("person").await?;
//     println!("People after delete: {:?}", people_after_delete);

//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() -> Result<()> {
//     main()?;
//     Ok(())
// }
