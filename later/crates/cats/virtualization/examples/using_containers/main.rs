//! Lists and runs Docker containers, then exercises an isolated Redis service.

mod docker_client;
mod redis_test;

/// Executes the Docker recipes against the configured daemon.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    docker_client::run().await?;
    redis_test::run().await
}
