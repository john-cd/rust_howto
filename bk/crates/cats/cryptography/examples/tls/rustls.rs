#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Connect to a TLS server with `rustls` and verify its certificate.

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context as _;
use rustls::ClientConfig;
use rustls::RootCertStore;
use rustls::crypto::ring;
use rustls::pki_types::ServerName;
use tokio::io::AsyncWriteExt as _;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;

/// Builds a Rustls connector that trusts the supplied certificate roots.
fn connector_with_roots(
    root_store: RootCertStore,
) -> anyhow::Result<TlsConnector> {
    let config =
        ClientConfig::builder_with_provider(Arc::new(ring::default_provider()))
            .with_safe_default_protocol_versions()?
            .with_root_certificates(root_store)
            .with_no_client_auth();

    Ok(TlsConnector::from(Arc::new(config)))
}

/// Builds a root store from Mozilla's WebPKI certificate list.
fn webpki_root_store() -> RootCertStore {
    RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned())
}

/// Opens a verified TLS connection to the resolved server address.
async fn connect_to_server(
    server_name: &str,
    address: SocketAddr,
    root_store: RootCertStore,
) -> anyhow::Result<TlsStream<TcpStream>> {
    let connector = connector_with_roots(root_store)?;
    let server_name = ServerName::try_from(server_name.to_owned())?;
    let tcp_stream = TcpStream::connect(address).await?;

    Ok(connector.connect(server_name, tcp_stream).await?)
}

/// Connects to the host named by `TLS_SERVER` when one is configured.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Ok(host) = std::env::var("TLS_SERVER") else {
        println!("Set TLS_SERVER to connect to a TLS server on port 443.");
        return Ok(());
    };
    let port = std::env::var("TLS_PORT")
        .unwrap_or_else(|_| "443".to_owned())
        .parse::<u16>()
        .context("TLS_PORT must be a valid TCP port")?;
    let address = tokio::net::lookup_host((host.as_str(), port))
        .await?
        .next()
        .with_context(|| format!("{host} resolved to no addresses"))?;

    let mut stream =
        connect_to_server(&host, address, webpki_root_store()).await?;
    stream.shutdown().await?;
    println!("Established a verified TLS connection to {host}:{port}.");
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rustls::ServerConfig;
    use rustls::pki_types::PrivateKeyDer;
    use rustls::pki_types::PrivatePkcs8KeyDer;
    use tokio::io::AsyncReadExt as _;
    use tokio::net::TcpListener;
    use tokio_rustls::TlsAcceptor;

    use super::*;

    /// Verifies that the client validates a local certificate and exchanges
    /// data.
    #[tokio::test]
    async fn client_verifies_certificate_and_round_trips_over_loopback()
    -> anyhow::Result<()> {
        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(vec!["localhost".to_owned()])?;
        let private_key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(
            signing_key.serialize_der(),
        ));
        let provider = Arc::new(ring::default_provider());
        let server_config = ServerConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()?
            .with_no_client_auth()
            .with_single_cert(vec![cert.der().clone()], private_key)?;
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let acceptor = TlsAcceptor::from(Arc::new(server_config));
        let server_task = tokio::spawn(async move {
            let (tcp_stream, _) = listener.accept().await?;
            let mut tls_stream = acceptor.accept(tcp_stream).await?;
            let mut request = [0; 4];
            tls_stream.read_exact(&mut request).await?;
            tls_stream.write_all(&request).await?;
            anyhow::Ok(())
        });

        let mut root_store = RootCertStore::empty();
        root_store.add(cert.der().clone())?;
        let mut stream =
            connect_to_server("localhost", address, root_store).await?;
        stream.write_all(b"ping").await?;
        let mut response = [0; 4];
        stream.read_exact(&mut response).await?;
        assert_eq!(&response, b"ping");
        drop(stream);
        server_task.await??;

        Ok(())
    }
}
