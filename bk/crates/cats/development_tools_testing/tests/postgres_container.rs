// TODO Reviewed ??-??-2026.
//! Checks PostgreSQL queries and type mappings against disposable Docker containers.

// ANCHOR: example
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::ContainerAsync;
use testcontainers_modules::testcontainers::ImageExt;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use tokio_postgres::NoTls;

/// Starts a ready PostgreSQL instance and returns its handle and connection settings.
///
/// # Errors
///
/// Returns an error if Docker cannot start the container or resolve its host and port.
async fn start_postgres() -> anyhow::Result<(ContainerAsync<Postgres>, tokio_postgres::Config)> {
    let container = Postgres::default().with_tag("17.6-alpine").start().await?;
    let mut config = tokio_postgres::Config::new();
    config
        .host(container.get_host().await?.to_string())
        .port(container.get_host_port_ipv4(5432).await?)
        .user("postgres")
        .password("postgres")
        .dbname("postgres");
    Ok((container, config))
}

/// Runs a query after the PostgreSQL container is ready.
///
/// # Errors
///
/// Returns an error if Docker, the database connection, or the query fails.
#[tokio::test]
#[ignore = "requires a running Docker daemon and a PostgreSQL image"]
async fn queries_postgres_in_docker() -> anyhow::Result<()> {
    let (_container, config) = start_postgres().await?;
    let (client, connection) = config.connect(NoTls).await?;
    let query = async move {
        let row = client.query_one("SELECT 6 * 7", &[]).await?;
        row.try_get::<_, i32>(0)
    };
    let (answer, ()) = tokio::try_join!(query, connection)?;
    assert_eq!(answer, 42);
    Ok(())
}
// ANCHOR_END: example

// ANCHOR: postgres_enum
use postgres_types::FromSql;
use postgres_types::ToSql;

/// A Rust enum matching the PostgreSQL type and its SQL labels.
#[derive(Debug, PartialEq, Eq, FromSql, ToSql)]
#[postgres(name = "order_status", rename_all = "snake_case")]
enum OrderStatus {
    Pending,
    Shipped,
}

/// Inserts and reads back an enum value using the shared PostgreSQL fixture.
///
/// # Errors
///
/// Returns an error if Docker, the database connection, or a SQL statement fails.
#[tokio::test]
#[ignore = "requires a running Docker daemon and a PostgreSQL image"]
async fn round_trips_postgres_enum() -> anyhow::Result<()> {
    let (_container, config) = start_postgres().await?;
    let (client, connection) = config.connect(NoTls).await?;
    let query = async move {
        client
            .batch_execute(
                "CREATE TYPE order_status AS ENUM ('pending', 'shipped');
                 CREATE TEMP TABLE orders (status order_status NOT NULL);",
            )
            .await?;
        client
            .execute(
                "INSERT INTO orders (status) VALUES ($1)",
                &[&OrderStatus::Shipped],
            )
            .await?;
        let row = client.query_one("SELECT status FROM orders", &[]).await?;
        row.try_get::<_, OrderStatus>(0)
    };

    let (status, ()) = tokio::try_join!(query, connection)?;
    assert_eq!(status, OrderStatus::Shipped);
    Ok(())
}
// ANCHOR_END: postgres_enum
