//! Checks a PostgreSQL query against a disposable Docker container.

// ANCHOR: example
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::ImageExt;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use tokio_postgres::NoTls;

/// Runs a query after the PostgreSQL container is ready.
///
/// # Errors
///
/// Returns an error if Docker, the database connection, or the query fails.
#[tokio::test]
#[ignore = "requires a running Docker daemon and a PostgreSQL image"]
async fn queries_postgres_in_docker() -> anyhow::Result<()> {
    let container = Postgres::default().with_tag("17.6-alpine").start().await?;
    let mut config = tokio_postgres::Config::new();
    config
        .host(&container.get_host().await?.to_string())
        .port(container.get_host_port_ipv4(5432).await?)
        .user("postgres")
        .password("postgres")
        .dbname("postgres");

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
