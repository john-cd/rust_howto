#![allow(dead_code)]
#![cfg(feature = "mongodb")]
// ANCHOR: example
use std::env;

use dotenvy::dotenv;
use mongodb::Client;
use mongodb::bson::doc;
use serde::Deserialize;
use serde::Serialize;

/// Represents a user with a name and age.
#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file, if present.
    dotenv().ok();

    // Retrieve the MongoDB connection URI from the environment variable.
    // Example: mongodb://user:password@server:27017/
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
    collection.insert_one(&user).await?;

    let filter = doc! { "name": "Alice" };
    if let Some(user) = collection.find_one(filter).await? {
        println!("Found user: {user:?}");
    } else {
        println!("User not found");
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    unsafe {
        // Set the MONGO_URI environment variable to connect to the MongoDB
        // service. Refer to the compose*.yaml files for the service
        // configuration.
        env::set_var(
            "MONGO_URI",
            "mongodb://mongoadmin:mysecretpassword@rust_howto_dev-mongodb-1:27017/",
        );
    }
    main()?;
    Ok(())
}
