#![allow(dead_code)]
// ANCHOR: example
// This example demonstrates asynchronous Cassandra access using `cdrs-tokio`.
use std::sync::Arc;

use anyhow::Context;
use cdrs_tokio::authenticators::StaticPasswordAuthenticatorProvider;
use cdrs_tokio::cluster::NodeTcpConfigBuilder;
use cdrs_tokio::cluster::TcpConnectionManager;
use cdrs_tokio::cluster::session::Session;
use cdrs_tokio::cluster::session::TcpSessionBuilder;
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use cdrs_tokio::query::*;
use cdrs_tokio::query_values;
use cdrs_tokio::transport::TransportTcp;
use uuid::Uuid;

// `cdrs_tokio` is a native async Cassandra DB client written in Rust.

type CurrentSession = Session<
    TransportTcp,
    TcpConnectionManager,
    RoundRobinLoadBalancingStrategy<TransportTcp, TcpConnectionManager>,
>;

#[derive(Clone, Debug, TryFromRow, IntoCdrsValue, PartialEq)]
struct RowStruct {
    key: Uuid,
    name: String,
    age: i32,
}

impl RowStruct {
    fn into_query_values(self) -> QueryValues {
        query_values!("key" => self.key, "name" => self.name, "age" => self.age)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let user = "user";
    let password = "password";
    let auth = StaticPasswordAuthenticatorProvider::new(user, password);

    let cluster_config = NodeTcpConfigBuilder::new()
        .with_contact_point("127.0.0.1:9042".into())
        .with_authenticator_provider(Arc::new(auth))
        .build()
        .await
        .context("failed to build Cassandra cluster config")?;

    let session: CurrentSession = TcpSessionBuilder::new(
        RoundRobinLoadBalancingStrategy::new(),
        cluster_config,
    )
    .build()
    .await
    .context("failed to establish Cassandra session")?;

    session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS test_keyspace WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};",
        )
        .await
        .context("failed to create keyspace")?;

    session
        .query(
            "CREATE TABLE IF NOT EXISTS test_keyspace.users (id UUID PRIMARY KEY, name TEXT, age INT);",
        )
        .await
        .context("failed to create table")?;

    let row = RowStruct {
        key: Uuid::new_v4(),
        name: "Alice".to_string(),
        age: 30,
    };

    session
        .query_with_values(
            "INSERT INTO test_keyspace.users (id, name, age) VALUES (?, ?, ?);",
            row.into_query_values(),
        )
        .await
        .context("failed to insert row")?;

    println!("Inserted user: Alice (ID: {})", row.key);

    let rows = session
        .query_with_values(
            "SELECT id, name, age FROM test_keyspace.users WHERE name = ?;",
            ("Alice",),
        )
        .await?
        .response_body()?
        .into_rows()
        .ok_or_else(|| anyhow::anyhow!("No rows in the result set"))?;

    for row in rows {
        let (id, name, age): (Uuid, String, i32) = row.try_into()?;
        println!("Found user: {name} (ID: {id}, Age: {age})");
    }

    Ok(())
}

// [finish; see also https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/crud_operations.rs](https://github.com/john-cd/rust_howto/issues/1017)
// <https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/multiple_thread.rs>

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn require_external_svc() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
