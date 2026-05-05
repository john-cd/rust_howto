#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates an async TLS client using `tokio-rustls`.

use std::sync::Arc;

use rustls::ClientConfig;
use rustls::OwnedTrustAnchor;
use rustls::RootCertStore;
use rustls::client::ServerName;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use webpki_roots::TLS_SERVER_ROOTS;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(TLS_SERVER_ROOTS.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect("example.com:443").await?;
    let server_name = ServerName::try_from("example.com")?;
    let mut stream = connector.connect(server_name, stream).await?;

    stream
        .write_all(
            b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n",
        )
        .await?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;

    println!("Response headers:");
    for line in String::from_utf8_lossy(&response).lines().take(6) {
        println!("{line}");
    }

    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires network"]
    async fn test_main() {
        main().await.unwrap();
    }
}
