#![allow(dead_code)]
// ANCHOR: example
use std::env;

// Import diesel.
use diesel::prelude::*;
// Import the oracle connection type.
use diesel_oci::OciConnection;
// Import dotenvy to load environment variables from a .env file.
// This is useful for managing configuration settings, especially
// sensitive information like database credentials.
use dotenvy::dotenv;

// ANCHOR: example
// `diesel_oci` is a Diesel backend and connection implementation for Oracle
// databases.

// In your `Cargo.toml`, add the following dependencies:
// [dependencies]
// diesel = { version = "2.3.3", features = [ ] }
// diesel-oci = "0.4.0"
// dotenvy = "0.15.7"
// tokio = { version = "1", features = ["full"] }

diesel::table! {
    // Import sql_types from diesel.
    use diesel::sql_types::*;
    // use diesel_full_text_search::*;

    /// The table containing all users
    users {
        /// The user's unique id
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}
// See: https://docs.diesel.rs/2.2.x/diesel/macro.table.html
// The `diesel::table!` macro is used to define a table schema in Diesel.

// Define a struct to hold the query results.
#[derive(QueryableByName, Queryable, PartialEq, Debug)]
#[diesel(table_name = users)]
struct User {
    username: String,
    password: String,
}
// `#[derive(Queryable)]` allows Diesel to map database rows to this struct.

fn main() -> anyhow::Result<()> {
    // Load environment variables (for secure handling of credentials)
    dotenv().ok();

    // Fetch connection details from environment variables
    let db_url = env::var("ORACLE_DB_URL").expect("ORACLE_DB_URL not set");
    let username =
        env::var("ORACLE_DB_USERNAME").expect("ORACLE_DB_USERNAME not set");
    let password =
        env::var("ORACLE_DB_PASSWORD").expect("ORACLE_DB_PASSWORD not set");

    // Set up a connection to Oracle DB using Diesel and diesel_oci
    let mut connection: OciConnection =
        establish_connection(&db_url, &username, &password)?;

    // Query the database (fetching users as an example)
    let results = diesel::sql_query("SELECT * FROM users WHERE ROWNUM <= 5")
        .load::<User>(&mut connection)?;

    // Print the results
    for user in results {
        println!("Username: {}, Password: {}", user.username, user.password);
    }

    // Use the connection similary to any other diesel connection
    let _res = users::table.load::<(i32, String, String)>(&mut connection)?;

    Ok(())
}

/// Establishes a connection to the Oracle database.
fn establish_connection(
    db_url: &str,
    username: &str,
    password: &str,
) -> Result<OciConnection, ConnectionError> {
    // Example: "oracle://user:secret@127.0.0.1/MY_DB".
    // The connection string format is:
    // "oracle://<username>:<password>@<db_url>"
    let connection_string = format!("oracle://{username}:{password}@{db_url}");

    // Create and return the connection
    OciConnection::establish(&connection_string)
}
// ANCHOR_END: example

#[test]
#[ignore = "requires external oracle db and oracle client library"]
fn require_external_svc() -> anyhow::Result<()> {
    let _lock = super::ENV_MUTEX.lock().unwrap();
    let username = std::env::var("TEST_ORACLE_DB_USERNAME")
        .expect("TEST_ORACLE_DB_USERNAME must be set");
    let password = std::env::var("TEST_ORACLE_DB_PASSWORD")
        .expect("TEST_ORACLE_DB_PASSWORD must be set");
    let db_url = std::env::var("TEST_ORACLE_DB_URL")
        .expect("TEST_ORACLE_DB_URL must be set");

    unsafe {
        env::set_var("ORACLE_DB_USERNAME", username);
        env::set_var("ORACLE_DB_PASSWORD", password);
        env::set_var("ORACLE_DB_URL", db_url);
    }
    main()?;
    Ok(())
}

// TODO review
// Troubleshooting "DPI-1047: Cannot locate a 64-bit Oracle Client library"
// This error occurs when the Oracle Instant Client is not installed or not
// in the system's library path.
//
// To resolve this on Linux:
// 1. Install `libaio1`: `sudo apt-get install libaio1`
// 2. Download and extract the Oracle Instant Client.
// 3. Set the `LD_LIBRARY_PATH` environment variable to the directory
//    containing the Instant Client libraries:
//    `export LD_LIBRARY_PATH=/path/to/instantclient:$LD_LIBRARY_PATH`
// 4. Alternatively, add the path to `/etc/ld.so.conf.d/` and run `ldconfig`.

// The simplest Oracle Client is the free Oracle Instant Client.
// Only the "Basic" or "Basic Light" package is required.
// <https://www.oracle.com/database/technologies/instant-client.html>
// <https://github.com/oracle/docker-images/tree/main/OracleInstantClient>

// review install steps in <https://odpi-c.readthedocs.io/en/latest/user_guide/introduction.html>
// Oracle Database Programming Interface for C (ODPI-C) is an open source
// library of C code that simplifies the use of common Oracle Call Interface
// (OCI) features for Oracle Database drivers and user applications.
// Install:
// sudo apt-get update
// sudo apt-get -y install odpic-dev
// sudo apt-get -y install odpic-doc

// To install Oracle Database on Ubuntu, first download the database software
// from the official Oracle website, then install OpenJDK 11 using the command
// `sudo apt install openjdk-11-jdk`. After that, extract the downloaded
// software and run the setup script to complete the installation.

// See also <https://github.com/oracle/docker-images/tree/main/OracleDatabase>
