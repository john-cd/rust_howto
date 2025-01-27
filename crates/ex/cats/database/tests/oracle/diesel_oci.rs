// // ANCHOR: example
// // ANCHOR_END: example

// use diesel::prelude::*;
// use diesel_oci::OciConnection;
// use dotenvy::dotenv;
// use std::env;

// // Diesel backend and connection implementation for Oracle databases

// // In your `Cargo.toml`, add the following dependencies:
// // [dependencies]
// // diesel = { version = "2.2.6", features = [ ] }
// // diesel-oci = "0.3.0"
// // dotenvy = "0.15.0"
// // tokio = { version = "1", features = ["full"] }

// diesel::table! {
//     use diesel::sql_types::*;
//     // use diesel_full_text_search::*;

//     /// The table containing all users
//     users {
//         /// The user's unique id
//         id -> Integer,
//         username -> Text,
//         password -> Text,
//     }
// }
// // See: https://docs.diesel.rs/2.2.x/diesel/macro.table.html

// // Define a struct to hold the query results
// #[derive(Queryable, PartialEq, Debug)]
// struct User {
//     username: String,
//     password: String,
// }

// fn main() -> anyhow::Result<()> {
//     // Load environment variables (for secure handling of credentials)
//     dotenv().ok();

//     // Fetch connection details from environment variables
//     let db_url = env::var("ORACLE_DB_URL").expect("ORACLE_DB_URL not set");
//     let username = env::var("ORACLE_DB_USERNAME").expect("ORACLE_DB_USERNAME
// not set");     let password =
// env::var("ORACLE_DB_PASSWORD").expect("ORACLE_DB_PASSWORD not set");

//     // Set up a connection to Oracle DB using Diesel and diesel_oci
//     let mut connection: OciConnection = establish_connection(&db_url,
// &username, &password)?;

//     // Query the database (fetching users as an example)
//     let results = diesel::sql_query("SELECT * FROM users WHERE ROWNUM <= 5")
//         .load::<User>(&mut connection)?;

//     // Print the results
//     for user in results {
//         println!("Username: {}, Password: {}", user.username, user.password);
//     }

//     Ok(())
// }

// /// Establishes a connection to the Oracle database
// fn establish_connection(db_url: &str, username: &str, password: &str) ->
// Result<OciConnection, ConnectionError> {     // Example:
// "oracle://user:secret@127.0.0.1/MY_DB"     let connection_string =
// format!("oracle://{}:{}@{}", username, password, db_url);

//     // Create and return the connection
//     OciConnection::establish(&connection_string)
// }

// #[test]
// fn require_external_svc() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // TODO P2 finish example; need heavy test
