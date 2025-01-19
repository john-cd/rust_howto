// ANCHOR: example
use std::env;

use dotenvy::dotenv;
use mongodb::Client;
use mongodb::bson::doc;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file, if present
    dotenv().ok();

    // Example: mongodb://root:passwd@server:27017/
    let mongo_uri = env::var("MONGO_URI")?;
    let client = Client::with_uri_str(&mongo_uri).await?;
    let db = client.database("test_db");
    let collection = db.collection::<User>("users");

    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    collection.insert_one(user).await?;

    let filter = doc! { "name": "Alice" };
    if let Some(user) = collection.find_one(filter).await? {
        println!("Found user: {:?}", user);
    } else {
        println!("User not found");
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() {
    unsafe {
        // Refer to the compose*.yaml files
        env::set_var(
            "MONGO_URI",
            "mongodb://mongoadmin:mysecretpassword@rust_howto_dev-mongodb-1:27017/",
        );
    }
    main().unwrap();
}
