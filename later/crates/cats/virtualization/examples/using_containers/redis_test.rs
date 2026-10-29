//! Tests a Redis service on a randomly mapped host port.

// ANCHOR: example
use std::time::Duration;

use testcontainers::GenericImage;
use testcontainers::core::IntoContainerPort;
use testcontainers::core::WaitFor;
use testcontainers::runners::AsyncRunner;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

/// Starts Redis, waits for readiness, and checks its RESP PING response.
async fn main() -> anyhow::Result<()> {
    let container = GenericImage::new("redis", "8.2")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout(
            "Ready to accept connections",
        ))
        .start()
        .await?;
    let host = container.get_host().await?;
    let port = container.get_host_port_ipv4(6379.tcp()).await?;
    tokio::time::timeout(Duration::from_secs(5), async {
        let mut stream =
            TcpStream::connect((host.to_string().as_str(), port)).await?;
        stream.write_all(b"*1\r\n$4\r\nPING\r\n").await?;
        let mut response = [0; 7];
        stream.read_exact(&mut response).await?;
        anyhow::ensure!(
            &response == b"+PONG\r\n",
            "unexpected Redis response: {response:?}"
        );
        Ok::<_, anyhow::Error>(())
    })
    .await??;
    // Drop removes this container under Testcontainers' default configuration.
    drop(container);
    Ok(())
}
// ANCHOR_END: example

/// Runs the isolated Redis service recipe.
pub async fn run() -> anyhow::Result<()> {
    main().await
}

#[cfg(test)]
mod tests {
    /// Checks Redis readiness, dynamic port discovery, and a real PING/PONG.
    #[tokio::test]
    #[ignore = "needs a Docker daemon"]
    async fn isolated_redis_answers_ping() -> anyhow::Result<()> {
        super::main().await
    }
}
