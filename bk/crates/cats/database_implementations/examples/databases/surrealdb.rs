#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Use SurrealDB's embedded in-memory engine for a self-contained CRUD example.

#![allow(
    clippy::result_large_err,
    reason = "Keep the structured error type provided by the SurrealDB SDK."
)]

use surrealdb::Result;
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use surrealdb::types::SurrealValue;

/// A person, stored in the database.
#[derive(Debug, SurrealValue)]
struct Person {
    /// The name of the person.
    name: String, // or: Cow<'static, str>
    /// The age of the person.
    age: u16,
}

/// Creates, reads, updates, and deletes one person in an in-memory database.
///
/// The `database_implementations` package enables the `kv-mem` feature. For a
/// persistent embedded store, enable another engine feature in `Cargo.toml`.
#[tokio::main]
async fn main() -> Result<()> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("test_namespace").use_db("test_db").await?;

    let created: Option<Person> = db
        .create(("person", "John Doe"))
        .content(Person {
            name: "J. Doe".to_owned(),
            age: 42,
        })
        .await?;

    println!("Created person: {created:?}");

    let people: Vec<Person> = db.select("person").await?;
    println!("All people: {people:?}");

    // Bind the value instead of interpolating it into the query string.
    let mut result = db
        .query("SELECT * FROM person WHERE name = $name")
        .bind(("name", "J. Doe"))
        .await?;

    let people: Vec<Person> = result.take(0)?;
    println!("Specific person: {people:?}");

    let _updated: Option<Person> = db
        .update(("person", "John Doe"))
        .content(Person {
            name: "J. Doe".to_owned(),
            age: 43,
        })
        .await?;

    let deleted: Option<Person> = db.delete(("person", "John Doe")).await?;
    println!("Deleted person: {deleted:?}");

    let people_after_delete: Vec<Person> = db.select("person").await?;
    println!("People after delete: {people_after_delete:?}");

    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
///
/// # Errors
///
/// Returns an error if the in-memory engine or any CRUD operation fails.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs CRUD operations against an isolated in-memory database.
    #[test]
    fn uses_the_embedded_memory_engine() -> Result<()> {
        main()?;
        Ok(())
    }
}
