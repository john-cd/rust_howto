#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Inserting and querying documents in MongoDB.

// This example demonstrates connecting to MongoDB and performing basic CRUD
// operations with the `mongodb` crate.
use std::env;

use dotenvy::dotenv;
use mongodb::Client;
use mongodb::bson::doc;
use serde::Deserialize;
use serde::Serialize;

/// Represents a user with a name and age.
#[derive(Debug, Serialize, Deserialize)]
struct User {
    /// Name of the user.
    name: String,
    /// Age of the user, in years.
    age: i32,
}

/// Connects to MongoDB and inserts a user into a collection.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file, if present.
    dotenv().ok();

    // Retrieve the MongoDB connection URI from the environment variable.
    // Example: `mongodb://user:password@server:27017/`.
    let mongo_uri = env::var("MONGO_URI")?;
    // Create a new MongoDB client.
    let client = Client::with_uri_str(&mongo_uri).await?;
    // Get a handle to the "test_db" database.
    let db = client.database("test_db");
    // Get a handle to the "users" collection, expecting documents of type
    // `User`.
    let collection = db.collection::<User>("users");

    // Create a new user.
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    // Insert the user into the collection.
    let inserted = collection.insert_one(&user).await?;

    // Query by the generated identifier so repeated runs find this insert.
    let filter = doc! { "_id": inserted.inserted_id };
    let found =
        collection.find_one(filter.clone()).await?.ok_or_else(|| {
            anyhow::anyhow!("inserted MongoDB user was not found")
        })?;
    anyhow::ensure!(
        found.name == "Alice" && found.age == 30,
        "inserted MongoDB user did not match the expected values"
    );
    println!("Found user: {found:?}");

    // Remove only the document created by this example.
    collection.delete_one(filter).await?;

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example against a MongoDB deployment configured by `MONGO_URI`.
    #[test]
    #[ignore = "needs MongoDB reachable through MONGO_URI"]
    fn require_external_svc() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
