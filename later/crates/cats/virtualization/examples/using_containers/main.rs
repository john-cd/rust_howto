//! Lists and runs Docker containers, then exercises an isolated Redis service.

mod docker_client;
mod redis_test;

/// Executes the Docker recipes against the configured daemon.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    docker_client::run().await?;
    redis_test::run().await
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs both container examples through the entry point.
    #[test]
    #[ignore = "needs a Docker daemon"]
    fn test_main_entry_point() {
        let result = main();
        assert!(result.is_ok(), "container examples failed: {result:?}");
    }
}
