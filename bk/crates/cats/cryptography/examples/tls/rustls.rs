#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Build a Rustls client configuration and async connector.
//!
//! This example creates a `ClientConfig` with an empty root store and
//! wraps it in a tokio-compatible TLS connector.

use std::sync::Arc;

use rustls::ClientConfig;
use rustls::RootCertStore;
use rustls::crypto::ring;
use tokio_rustls::TlsConnector;

/// Builds a TLS connector that trusts the supplied certificate roots.
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

/// Builds a client connector and explains how to supply trusted root
/// certificates.
fn main() -> anyhow::Result<()> {
    let _connector = connector_with_roots(RootCertStore::empty())?;

    println!(
        "Built a rustls client connector. Add trusted root certificates before connecting to a TLS server."
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use rustls::ServerConfig;
    use rustls::pki_types::PrivateKeyDer;
    use rustls::pki_types::PrivatePkcs8KeyDer;
    use rustls::pki_types::ServerName;
    use tokio::io::AsyncReadExt as _;
    use tokio::io::AsyncWriteExt as _;
    use tokio::io::duplex;
    use tokio_rustls::TlsAcceptor;

    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }

    /// Verifies the example connector completes a certificate-validated TLS
    /// handshake.
    #[tokio::test]
    async fn client_connector_performs_a_verified_tls_round_trip()
    -> anyhow::Result<()> {
        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(vec!["localhost".to_owned()])?;
        let private_key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(
            signing_key.serialize_der(),
        ));
        let provider = Arc::new(ring::default_provider());
        let server_config =
            ServerConfig::builder_with_provider(provider.clone())
                .with_safe_default_protocol_versions()?
                .with_no_client_auth()
                .with_single_cert(vec![cert.der().clone()], private_key)?;

        let mut root_store = RootCertStore::empty();
        root_store.add(cert.der().clone())?;
        let connector = connector_with_roots(root_store)?;
        let acceptor = TlsAcceptor::from(Arc::new(server_config));
        let (client_io, server_io) = duplex(1024);

        let server_task = tokio::spawn(async move {
            let mut stream = acceptor.accept(server_io).await?;
            let mut request = [0; 4];
            stream.read_exact(&mut request).await?;
            stream.write_all(&request).await?;
            anyhow::Ok(())
        });

        let server_name = ServerName::try_from("localhost".to_owned())?;
        let mut stream = connector.connect(server_name, client_io).await?;
        stream.write_all(b"ping").await?;
        let mut response = [0; 4];
        stream.read_exact(&mut response).await?;
        assert_eq!(&response, b"ping");
        drop(stream);
        server_task.await??;

        Ok(())
    }
}
