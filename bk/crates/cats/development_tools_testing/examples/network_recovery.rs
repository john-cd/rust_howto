//! Tests TCP recovery across a simulated network partition.

// ANCHOR: example
use std::io::ErrorKind;
use std::time::Duration;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time::timeout;
use turmoil::net::TcpListener;
use turmoil::net::TcpStream;

/// Reads a greeting from a new connection to the simulated server.
///
/// # Errors
///
/// Returns an error if the connection or read fails.
async fn greeting() -> std::io::Result<[u8; 2]> {
    let mut stream = TcpStream::connect(("server", 8080)).await?;
    let mut message = [0; 2];
    stream.read_exact(&mut message).await?;
    Ok(message)
}

/// Verifies that a connection fails during a partition and succeed after repair.
///
/// # Errors
///
/// Returns an error if the simulation, its server, or a healthy request fails.
fn main() -> turmoil::Result {
    let mut sim = turmoil::Builder::new()
        .rng_seed(42)
        .simulation_duration(Duration::from_secs(2))
        .build();

    sim.host("server", || async {
        let listener = TcpListener::bind("0.0.0.0:8080").await?;
        loop {
            let (mut stream, _) = listener.accept().await?;
            stream.write_all(b"ok").await?;
        }
    });

    sim.client("client", async {
        let deadline = Duration::from_millis(100);
        assert_eq!(timeout(deadline, greeting()).await??, *b"ok");

        turmoil::partition("client", "server");
        let interrupted = timeout(deadline, greeting()).await;
        assert!(matches!(
            interrupted,
            Ok(Err(error)) if error.kind() == ErrorKind::ConnectionRefused
        ));

        turmoil::repair("client", "server");
        assert_eq!(timeout(deadline, greeting()).await??, *b"ok");
        Ok(())
    });

    sim.run()
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Checks a simulated TCP client fails to connect during a partition and recovers after repair.
    #[test]
    fn recovers_after_network_repair() -> turmoil::Result {
        super::main()
    }
}
