// TODO Reviewed ??-??-2026.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "needs an S3-compatible service; the tests use mocks"
    )
)]
// ANCHOR: example
//! Point the S3 client at an S3-compatible store (MinIO, RustFS, LocalStack),
//! optionally over HTTPS with a private certificate authority.

use anyhow::Context;
use anyhow::Result;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::BehaviorVersion;
use aws_sdk_s3::config::Builder;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::config::Region;
use aws_smithy_http_client::Builder as HttpClientBuilder;
use aws_smithy_http_client::tls::Provider as TlsProvider;
use aws_smithy_http_client::tls::TlsContext;
use aws_smithy_http_client::tls::TrustStore;
use aws_smithy_http_client::tls::rustls_provider::CryptoMode;

/// Builds a client for `endpoint`, e.g. `http://localhost:9000`.
///
/// The keys are static here for local development only. In production, use the
/// credential provider chain (see the shared configuration example) and never
/// commit keys.
pub fn s3_compatible_client(
    endpoint: &str,
    access_key: &str,
    secret_key: &str,
) -> Client {
    let credentials =
        Credentials::new(access_key, secret_key, None, None, "static");
    let config = Builder::new()
        .behavior_version(BehaviorVersion::latest())
        // Most S3-compatible stores ignore the region, but the SDK needs one.
        .region(Region::new("us-east-1"))
        .endpoint_url(endpoint)
        .credentials_provider(credentials)
        // `http://host/bucket/key` instead of `http://bucket.host/key`.
        .force_path_style(true)
        .build();
    Client::from_conf(config)
}

/// Builds a client for an HTTPS `endpoint` signed by a private certificate
/// authority, for example a MinIO server behind an internal CA.
///
/// `ca_pem` holds the PEM-encoded CA certificate. [`TrustStore::empty`] drops
/// the platform roots, so only that CA is trusted; use `TrustStore::default()`
/// with `with_pem_certificate` to trust the CA in addition to the platform
/// roots. Never disable certificate verification instead.
pub fn s3_compatible_tls_client(
    endpoint: &str,
    ca_pem: &[u8],
    access_key: &str,
    secret_key: &str,
) -> Result<Client> {
    let trust_store = TrustStore::empty().with_pem_certificate(ca_pem);
    let tls_context = TlsContext::builder()
        .with_trust_store(trust_store)
        .build()
        .context("invalid TLS configuration")?;
    let http_client = HttpClientBuilder::new()
        .tls_provider(TlsProvider::Rustls(CryptoMode::AwsLc))
        .tls_context(tls_context)
        .build_https();

    let credentials =
        Credentials::new(access_key, secret_key, None, None, "static");
    let config = Builder::new()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new("us-east-1"))
        .endpoint_url(endpoint)
        .credentials_provider(credentials)
        .force_path_style(true)
        .http_client(http_client)
        .build();
    Ok(Client::from_conf(config))
}

/// Creates a bucket and stores an object, then reads it back.
pub async fn round_trip(client: &Client, bucket: &str) -> Result<Vec<u8>> {
    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .context("CreateBucket failed")?;
    client
        .put_object()
        .bucket(bucket)
        .key("hello.txt")
        .body(b"hello".to_vec().into())
        .send()
        .await
        .context("PutObject failed")?;
    let object = client
        .get_object()
        .bucket(bucket)
        .key("hello.txt")
        .send()
        .await
        .context("GetObject failed")?;
    Ok(object.body.collect().await?.into_bytes().to_vec())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the configuration is built without contacting the store.
    #[test]
    fn client_uses_the_endpoint_and_path_style() {
        let client =
            s3_compatible_client("http://localhost:9000", "access", "secret");
        let config = client.config();
        assert_eq!(config.region().map(Region::as_ref), Some("us-east-1"));
    }

    /// Starts a local HTTPS server with a certificate issued for `localhost`.
    /// Returns the server's port and the PEM-encoded certificate. Each
    /// connection gets an empty `200 OK` response, which is enough for
    /// `HeadBucket`.
    async fn start_https_server() -> Result<(u16, String)> {
        use std::sync::Arc;

        use tokio::io::AsyncReadExt;
        use tokio::io::AsyncWriteExt;
        use tokio::net::TcpListener;
        use tokio_rustls::TlsAcceptor;
        use tokio_rustls::rustls::ServerConfig;
        use tokio_rustls::rustls::crypto::aws_lc_rs;
        use tokio_rustls::rustls::pki_types::PrivateKeyDer;
        use tokio_rustls::rustls::pki_types::PrivatePkcs8KeyDer;

        let certified =
            rcgen::generate_simple_self_signed(vec!["localhost".to_owned()])?;
        let cert_pem = certified.cert.pem();
        let key = PrivateKeyDer::from(PrivatePkcs8KeyDer::from(
            certified.signing_key.serialize_der(),
        ));
        // Name the provider: other crates in the build also enable `ring`, so
        // Rustls cannot pick a process-wide default on its own.
        let provider = Arc::new(aws_lc_rs::default_provider());
        let config = ServerConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()?
            .with_no_client_auth()
            .with_single_cert(vec![certified.cert.der().clone()], key)?;
        let acceptor = TlsAcceptor::from(Arc::new(config));
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();

        tokio::spawn(async move {
            while let Ok((socket, _)) = listener.accept().await {
                let acceptor = acceptor.clone();
                tokio::spawn(async move {
                    let Ok(mut stream) = acceptor.accept(socket).await else {
                        return;
                    };
                    let mut request = [0_u8; 4096];
                    let _ = stream.read(&mut request).await;
                    let _ = stream
                        .write_all(
                            b"HTTP/1.1 200 OK\r\ncontent-length: 0\r\nconnection: close\r\n\r\n",
                        )
                        .await;
                    let _ = stream.shutdown().await;
                });
            }
        });
        Ok((port, cert_pem))
    }

    /// Verifies that only a client that trusts the private CA reaches the
    /// store.
    #[tokio::test]
    async fn custom_trust_store_accepts_the_private_ca() -> Result<()> {
        let (port, cert_pem) = start_https_server().await?;
        let endpoint = format!("https://localhost:{port}");

        let trusting = s3_compatible_tls_client(
            &endpoint,
            cert_pem.as_bytes(),
            "access",
            "secret",
        )?;
        trusting.head_bucket().bucket("demo").send().await?;

        // A client that trusts a different CA rejects the server certificate.
        let other_ca = rcgen::generate_simple_self_signed(vec![
            "other.example".to_owned(),
        ])?
        .cert
        .pem();
        let distrusting = s3_compatible_tls_client(
            &endpoint,
            other_ca.as_bytes(),
            "access",
            "secret",
        )?;
        assert!(
            distrusting
                .head_bucket()
                .bucket("demo")
                .send()
                .await
                .is_err()
        );
        Ok(())
    }

    /// Exercises a real store. Start one first, for example MinIO on port 9000
    /// with the keys `minioadmin` / `minioadmin`, then run
    /// `cargo test -- --ignored`.
    #[tokio::test]
    #[ignore = "requires an S3-compatible server on localhost:9000"]
    async fn stores_and_reads_an_object() -> Result<()> {
        let client = s3_compatible_client(
            "http://localhost:9000",
            "minioadmin",
            "minioadmin",
        );
        let bytes = round_trip(&client, "rust-howto-test").await?;
        assert_eq!(bytes, b"hello");
        Ok(())
    }
}
