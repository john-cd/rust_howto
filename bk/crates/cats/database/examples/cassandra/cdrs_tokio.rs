#![allow(dead_code)]
// ANCHOR: example
// This example demonstrates asynchronous Cassandra access using `cdrs-tokio`.

use std::sync::Arc;

use anyhow::Context;
use cdrs_tokio::IntoCdrsValue;
use cdrs_tokio::authenticators::StaticPasswordAuthenticatorProvider;
use cdrs_tokio::cluster::NodeTcpConfigBuilder;
use cdrs_tokio::cluster::TcpConnectionManager;
use cdrs_tokio::cluster::session::Session;
use cdrs_tokio::cluster::session::SessionBuilder;
use cdrs_tokio::cluster::session::TcpSessionBuilder;
use cdrs_tokio::frame::TryFromRow as TryFromRowTrait;
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use cdrs_tokio::query::*;
use cdrs_tokio::query_values;
use cdrs_tokio::transport::TransportTcp;
use tokio;
use uuid::Uuid;

// `cdrs_tokio` is a native async Cassandra DB client written in Rust.

type CurrentSession = Session<
    TransportTcp,
    TcpConnectionManager,
    RoundRobinLoadBalancingStrategy<TransportTcp, TcpConnectionManager>,
>;

#[derive(Clone, Debug, IntoCdrsValue, cdrs_tokio::TryFromRow, PartialEq)]
struct RowStruct {
    key: Uuid,
    name: String,
    age: i32,
}

impl RowStruct {
    fn into_query_values(self) -> QueryValues {
        // Transforms arguments to values consumed by queries
        query_values!("id" => self.key, "name" => self.name, "age" => self.age)
    }
}
  
async fn example() -> anyhow::Result<()> {
    // Load environment variables from a .env file (for secure handling of
    // credentials).
    use dotenvy::dotenv;
    dotenv().ok();

    let user = std::env::var("CASSANDRA_USER").expect("CASSANDRA_USER not set");
    let password = std::env::var("CASSANDRA_PASSWORD")
        .expect("CASSANDRA_PASSWORD not set");
    let auth = StaticPasswordAuthenticatorProvider::new(&user, &password
      
    // For tests, you may use
    // `cdrs_tokio::authenticators::NoneAuthenticatorProvider`.

    // Connect to a Cassandra cluster:
    let cluster_config = NodeTcpConfigBuilder::new()
        .with_contact_point("127.0.0.1:9042".into())
        .with_authenticator_provider(Arc::new(auth))
        .build()
        .await
        .context("failed to build Cassandra cluster config")?;
      
    // Create a CDRS session that holds a pool of connections to nodes
    // and provides an interface for interacting with the cluster:
    let session: CurrentSession = TcpSessionBuilder::new(
        RoundRobinLoadBalancingStrategy::new(),
        cluster_config,
    )
    .build()
    .await?;

    // Create keyspace and table (if they don't exist):
    session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS test_keyspace WITH replication = \
{'class': 'SimpleStrategy', 'replication_factor': 1};",
        )
        .await?;

    session
        .query(
            "CREATE TABLE IF NOT EXISTS test_keyspace.users (id UUID PRIMARY \
KEY, name TEXT, age INT);",
        )
        .await?;

    // Insert a row into the 'users' table:
    let insert_query =
        "INSERT INTO test_keyspace.users (id, name, age) VALUES (?, ?, ?);";
    let id = Uuid::new_v4();
    let name = "Alice";
    let age = 30;
    let row = RowStruct {
        key: id,
        name: name.to_string(),
        age,
    };
    session
        .query_with_values(insert_query, row.into_query_values())
        .await?;

    println!("Inserted user: {name} (ID: {id})");

    // Query the inserted row:
    let select_query =
        "SELECT id, name, age FROM test_keyspace.users WHERE name = ?;";
    let rows = session
        .query_with_values(select_query, query_values!(name))
        .await?
        .response_body()?
        .into_rows()
        .ok_or_else(|| anyhow::anyhow!("No rows in the result set"))?;

    // Display the result of the query:
    for row in rows {
        let row = <RowStruct as TryFromRowTrait>::try_from_row(row)?;
        let id = row.key;
        let name = row.name;
        let age = row.age;
        println!("Found user: {name} (ID: {id}, Age: {age})");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    example().await
}
// ANCHOR_END: example

pub async fn run() -> anyhow::Result<()> {
    example().await
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[tokio::test]
    async fn require_external_svc() -> anyhow::Result<()> {
        let _lock = ENV_MUTEX.lock().unwrap();
        let user = std::env::var("TEST_CASSANDRA_USER")
            .expect("TEST_CASSANDRA_USER must be set");
        let password = std::env::var("TEST_CASSANDRA_PASSWORD")
            .expect("TEST_CASSANDRA_PASSWORD must be set");

        unsafe {
            std::env::set_var("CASSANDRA_USER", user);
            std::env::set_var("CASSANDRA_PASSWORD", password);
        }
        run().await?;
        Ok(())
    }
    // [finish; see also https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/crud_operations.rs](https://github.com/john-cd/rust_howto/issues/1017)
    // <https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/multiple_thread.rs>
}
