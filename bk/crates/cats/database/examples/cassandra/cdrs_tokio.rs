// TODO Reviewed ??-??-2026.

#![allow(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]

// ANCHOR: example
//! Connecting to Cassandra with the `cdrs-tokio` async client.

// This example demonstrates asynchronous Cassandra access using `cdrs-tokio`.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use cdrs_tokio::IntoCdrsValue;
use cdrs_tokio::authenticators::NoneAuthenticatorProvider;
use cdrs_tokio::cluster::NodeTcpConfigBuilder;
use cdrs_tokio::cluster::TcpConnectionManager;
use cdrs_tokio::cluster::session::Session;
use cdrs_tokio::cluster::session::SessionBuilder as _;
use cdrs_tokio::cluster::session::TcpSessionBuilder;
use cdrs_tokio::frame::TryFromRow as TryFromRowTrait;
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use cdrs_tokio::query::*;
use cdrs_tokio::query_values;
use cdrs_tokio::transport::TransportTcp;
use uuid::Uuid;

// `cdrs_tokio` is a native async Cassandra DB client written in Rust.

/// Contact point used by this local Cassandra example.
const CONTACT_POINT: &str = "127.0.0.1:9042";
/// Maximum time to wait for the initial Cassandra connection.
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

/// A session that uses TCP transport and round-robin load balancing.
type CurrentSession = Session<
    TransportTcp,
    TcpConnectionManager,
    RoundRobinLoadBalancingStrategy<TransportTcp, TcpConnectionManager>,
>;

/// A row of the example table.
#[derive(Clone, Debug, IntoCdrsValue, cdrs_tokio::TryFromRow, PartialEq)]
struct RowStruct {
    /// Unique identifier of the row.
    key: Uuid,
    /// Name of the person.
    name: String,
    /// Age of the person, in years.
    age: i32,
}

impl RowStruct {
    /// Converts the row into the values bound to an insert query.
    fn into_query_values(self) -> QueryValues {
        // Transforms arguments to values consumed by queries
        query_values!("id" => self.key, "name" => self.name, "age" => self.age)
    }
}

/// Reads a user row by its primary key.
///
/// # Errors
///
/// Returns an error if the query fails or a result row cannot be decoded.
async fn select_user(
    session: &CurrentSession,
    id: Uuid,
) -> anyhow::Result<Option<RowStruct>> {
    let response = session
        .query_with_values(
            "SELECT id, name, age FROM test_keyspace.users WHERE id = ?",
            query_values!(id),
        )
        .await?;
    let rows = response.response_body()?.into_rows().ok_or_else(|| {
        anyhow::anyhow!("the SELECT response did not contain rows")
    })?;

    rows.into_iter()
        .next()
        .map(<RowStruct as TryFromRowTrait>::try_from_row)
        .transpose()
        .map_err(Into::into)
}

/// Creates the table, then inserts, reads, updates, and deletes a user row.
///
/// # Errors
///
/// Returns an error if Cassandra cannot be reached or any schema or data query
/// fails.
async fn example() -> anyhow::Result<()> {
    let cluster_config = NodeTcpConfigBuilder::new()
        .with_contact_point(CONTACT_POINT.into())
        .with_authenticator_provider(Arc::new(NoneAuthenticatorProvider))
        .build()
        .await
        .context("configure Cassandra contact point 127.0.0.1:9042")?;

    let session: CurrentSession = tokio::time::timeout(
        CONNECTION_TIMEOUT,
        TcpSessionBuilder::new(
            RoundRobinLoadBalancingStrategy::new(),
            cluster_config,
        )
        .build(),
    )
    .await
    .context("time out connecting to Cassandra at 127.0.0.1:9042")?
    .context("connect to Cassandra at 127.0.0.1:9042")?;

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

    let id = Uuid::new_v4();
    let new_user = RowStruct {
        key: id,
        name: "Alice".to_owned(),
        age: 30,
    };
    session
        .query_with_values(
            "INSERT INTO test_keyspace.users (id, name, age) VALUES (?, ?, ?)",
            new_user.into_query_values(),
        )
        .await?;

    let inserted = select_user(&session, id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("inserted row was not found"))?;
    anyhow::ensure!(
        inserted.name == "Alice" && inserted.age == 30,
        "inserted row did not match the expected values"
    );
    println!("Inserted user: {} (ID: {})", inserted.name, inserted.key);

    let rows = session
        .query_with_values(
            "UPDATE test_keyspace.users SET name = ?, age = ? WHERE id = ?",
            query_values!("Alice Updated", 31_i32, id),
        )
        .await?
        .response_body()?;
    println!("Updated user {id}: {rows:?}");

    let updated = select_user(&session, id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("updated row was not found"))?;
    anyhow::ensure!(
        updated.name == "Alice Updated" && updated.age == 31,
        "updated row did not match the expected values"
    );
    println!(
        "Selected updated user: {} (age {})",
        updated.name, updated.age
    );

    session
        .query_with_values(
            "DELETE FROM test_keyspace.users WHERE id = ?",
            query_values!(id),
        )
        .await?
        .response_body()?;
    anyhow::ensure!(
        select_user(&session, id).await?.is_none(),
        "deleted row is still present"
    );
    println!("Deleted user {id}");

    Ok(())
}

/// Runs the cdrs-tokio example.
///
/// # Errors
///
/// Returns an error if connecting to Cassandra or any CRUD operation fails.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    example().await
}
// ANCHOR_END: example

/// Runs the cdrs-tokio CRUD example.
///
/// # Errors
///
/// Returns an error if connecting to Cassandra or any CRUD operation fails.
pub async fn run() -> anyhow::Result<()> {
    example().await
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies create, insert, select, update, and delete against Cassandra.
    ///
    /// Start Cassandra with
    /// `docker run --rm -d --name rust-howto-cassandra -p 9042:9042
    /// cassandra:5.0` and wait for it to become ready before running this
    /// ignored integration test.
    #[tokio::test]
    #[ignore = "needs Cassandra at 127.0.0.1:9042"]
    async fn require_external_svc() -> anyhow::Result<()> {
        run().await?;
        Ok(())
    }

    #[test]
    #[ignore = "needs a running Cassandra server"]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
