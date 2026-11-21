// TODO Reviewed ??-??-2026.
//! Two authenticated Iroh endpoints communicating only over loopback.

use std::time::Duration;

use anyhow::Context;
use anyhow::Result;
use iroh::Endpoint;
use iroh::EndpointAddr;
use iroh::endpoint::RelayMode;
use iroh::endpoint::presets;

/// Protocol identifier negotiated by both endpoints during the QUIC handshake.
const ALPN: &[u8] = b"rust-howto/loopback/1";

/// Binds an endpoint to loopback with relays and address discovery disabled.
///
/// Registers the example protocol only when accepting incoming connections.
/// Propagates socket address and endpoint initialization errors.
async fn local_endpoint(accept: bool) -> Result<Endpoint> {
    let mut builder = Endpoint::builder(presets::N0)
        .relay_mode(RelayMode::Disabled)
        .clear_address_lookup()
        .bind_addr("127.0.0.1:0")?;
    if accept {
        builder = builder.alpns(vec![ALPN.to_vec()]);
    }
    Ok(builder.bind().await?)
}

// ANCHOR: example
/// Exchanges ping/pong between authenticated peers using an explicit loopback address.
///
/// First verifies that peer identity alone cannot connect without discovery.
/// Server and client futures then run together so accepting and opening the
/// bidirectional stream can make progress. Reads are capped at 32 bytes and the
/// whole exchange has a ten-second deadline; endpoints close after the exchange.
///
/// # Errors
///
/// Propagates endpoint setup, connection, stream, and timeout failures.
///
/// # Panics
///
/// Panics if peer identities, messages, or the missing-address failure differ
/// from the expected local demonstration.
pub async fn round_trip() -> Result<()> {
    let server = local_endpoint(true).await?;
    let client = local_endpoint(false).await?;
    let socket = server
        .bound_sockets()
        .into_iter()
        .find(std::net::SocketAddr::is_ipv4)
        .context("server has no IPv4 loopback socket")?;
    let server_address = EndpointAddr::new(server.id()).with_ip_addr(socket);

    // With discovery and relays disabled, an identity alone is not an address.
    let missing_address = EndpointAddr::new(server.id());
    let failed = tokio::time::timeout(
        Duration::from_secs(2),
        client.connect(missing_address, ALPN),
    )
    .await?;
    assert!(failed.is_err());

    let exchange = async {
        let serve = async {
            let incoming = server.accept().await.context("server closed")?;
            let connection = incoming.await?;
            assert_eq!(connection.remote_id(), client.id());
            let (mut send, mut receive) = connection.accept_bi().await?;
            assert_eq!(receive.read_to_end(32).await?, b"ping");
            send.write_all(b"pong").await?;
            send.finish()?;
            connection.closed().await;
            Ok::<_, anyhow::Error>(())
        };
        let call = async {
            let connection = client.connect(server_address, ALPN).await?;
            assert_eq!(connection.remote_id(), server.id());
            let (mut send, mut receive) = connection.open_bi().await?;
            send.write_all(b"ping").await?;
            send.finish()?;
            assert_eq!(receive.read_to_end(32).await?, b"pong");
            connection.close(0u8.into(), b"done");
            Ok::<_, anyhow::Error>(())
        };
        let (served, called) = tokio::join!(serve, call);
        served?;
        called?;
        Ok::<_, anyhow::Error>(())
    };
    let result = tokio::time::timeout(Duration::from_secs(10), exchange).await?;
    client.close().await;
    server.close().await;
    result
}
// ANCHOR_END: example

/// Verifies explicit loopback connectivity and failure without a peer address.
#[cfg(test)]
mod tests {
    /// Checks that loopback and missing address.
    #[tokio::test]
    async fn loopback_and_missing_address() -> anyhow::Result<()> {
        super::round_trip().await
    }
}
