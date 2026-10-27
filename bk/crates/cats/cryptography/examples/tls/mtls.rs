#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Send an HTTPS request with a client certificate and private key.

use std::time::Duration;

use anyhow::Context as _;
use anyhow::bail;
use reqwest::Certificate;
use reqwest::Client;
use reqwest::Identity;
use rustls::crypto::CryptoProvider;
use rustls::crypto::ring;

/// Builds a client that trusts the supplied server CA and presents a client
/// identity.
fn build_client(
    ca_certificate_pem: &[u8],
    client_identity_pem: &[u8],
) -> anyhow::Result<Client> {
    install_crypto_provider()?;
    let ca_certificate = Certificate::from_pem(ca_certificate_pem)?;
    let identity = Identity::from_pem(client_identity_pem)?;

    Ok(Client::builder()
        .tls_certs_only([ca_certificate])
        .identity(identity)
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(10))
        .build()?)
}

/// Installs the crate's configured Rustls provider when the application has not
/// selected one.
fn install_crypto_provider() -> anyhow::Result<()> {
    if CryptoProvider::get_default().is_some() {
        return Ok(());
    }

    match ring::default_provider().install_default() {
        Ok(()) => Ok(()),
        Err(_) if CryptoProvider::get_default().is_some() => Ok(()),
        Err(_) => bail!("could not install a Rustls crypto provider"),
    }
}

/// Sends a GET request using the configured CA certificate and client identity
/// files.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (Ok(ca_path), Ok(identity_path), Ok(url)) = (
        std::env::var("MTLS_CA_CERT_PEM"),
        std::env::var("MTLS_CLIENT_IDENTITY_PEM"),
        std::env::var("MTLS_URL"),
    ) else {
        println!(
            "Set MTLS_CA_CERT_PEM, MTLS_CLIENT_IDENTITY_PEM, and MTLS_URL to make a verified mTLS request."
        );
        return Ok(());
    };

    let ca_certificate_pem = std::fs::read(&ca_path)
        .with_context(|| format!("reading CA certificate from {ca_path}"))?;
    let client_identity_pem =
        std::fs::read(&identity_path).with_context(|| {
            format!("reading client identity from {identity_path}")
        })?;
    let client = build_client(&ca_certificate_pem, &client_identity_pem)?;
    let response = client.get(&url).send().await?.error_for_status()?;

    println!("mTLS request succeeded: {}", response.status());
    Ok(())
}
// ANCHOR_END: example

/// Dispatches this example from the TLS command-line group.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::ensure;
    use rcgen::BasicConstraints;
    use rcgen::CertificateParams;
    use rcgen::ExtendedKeyUsagePurpose;
    use rcgen::IsCa;
    use rcgen::Issuer;
    use rcgen::KeyPair;
    use rcgen::KeyUsagePurpose;
    use rcgen::PKCS_ED25519;
    use rcgen::SerialNumber;
    use rcgen::date_time_ymd;
    use rustls::RootCertStore;
    use rustls::ServerConfig;
    use rustls::crypto::ring;
    use rustls::pki_types::CertificateDer;
    use rustls::pki_types::PrivateKeyDer;
    use rustls::pki_types::PrivatePkcs8KeyDer;
    use rustls::server::WebPkiClientVerifier;
    use tokio::io::AsyncReadExt as _;
    use tokio::io::AsyncWriteExt as _;
    use tokio::net::TcpListener;
    use tokio_rustls::TlsAcceptor;

    use super::build_client;

    /// Holds in-memory certificates and keys for a local mutual TLS exchange.
    struct LocalCertificateFixtures {
        ca_certificate_der: CertificateDer<'static>,
        ca_certificate_pem: String,
        server_certificate_der: CertificateDer<'static>,
        server_private_key: PrivateKeyDer<'static>,
        client_identity_pem: String,
    }

    /// Builds an Ed25519 PKCS#8 fixture key from a fixed test-only seed.
    fn fixture_key(seed: u8) -> anyhow::Result<KeyPair> {
        let mut der = vec![
            0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65,
            0x70, 0x04, 0x22, 0x04, 0x20,
        ];
        der.extend([seed; 32]);
        let private_key = PrivatePkcs8KeyDer::from(der);

        Ok(KeyPair::from_pkcs8_der_and_sign_algo(
            &private_key,
            &PKCS_ED25519,
        )?)
    }

    /// Assigns fixed validity dates so local certificate fixtures are
    /// reproducible.
    fn set_fixture_validity(
        params: &mut CertificateParams,
        serial_number: u64,
    ) {
        params.not_before = date_time_ymd(2020, 1, 1);
        params.not_after = date_time_ymd(2100, 1, 1);
        params.serial_number = Some(SerialNumber::from(serial_number));
    }

    /// Creates a local CA, a localhost server certificate, and a client
    /// identity.
    fn local_certificate_fixtures() -> anyhow::Result<LocalCertificateFixtures>
    {
        let mut ca_params = CertificateParams::default();
        set_fixture_validity(&mut ca_params, 1);
        ca_params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
        ca_params.key_usages =
            vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];
        let ca_key = fixture_key(1)?;
        let ca_certificate = ca_params.self_signed(&ca_key)?;
        let issuer = Issuer::from_params(&ca_params, &ca_key);

        let server_key = fixture_key(2)?;
        let mut server_params =
            CertificateParams::new(vec!["localhost".to_owned()])?;
        set_fixture_validity(&mut server_params, 2);
        server_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
        server_params.extended_key_usages =
            vec![ExtendedKeyUsagePurpose::ServerAuth];
        let server_certificate =
            server_params.signed_by(&server_key, &issuer)?;

        let client_key = fixture_key(3)?;
        let mut client_params = CertificateParams::default();
        set_fixture_validity(&mut client_params, 3);
        client_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
        client_params.extended_key_usages =
            vec![ExtendedKeyUsagePurpose::ClientAuth];
        let client_certificate =
            client_params.signed_by(&client_key, &issuer)?;
        let client_identity_pem = format!(
            "{}{}",
            client_certificate.pem(),
            client_key.serialize_pem()
        );

        Ok(LocalCertificateFixtures {
            ca_certificate_der: ca_certificate.der().clone(),
            ca_certificate_pem: ca_certificate.pem(),
            server_certificate_der: server_certificate.der().clone(),
            server_private_key: PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(
                server_key.serialize_der(),
            )),
            client_identity_pem,
        })
    }

    /// Reads one bounded HTTP request and returns a fixed response over TLS.
    async fn serve_one_request(
        listener: TcpListener,
        acceptor: TlsAcceptor,
    ) -> anyhow::Result<()> {
        let (tcp_stream, _) = listener.accept().await?;
        let mut tls_stream = acceptor.accept(tcp_stream).await?;
        let mut request = Vec::with_capacity(1024);

        while !request.windows(4).any(|window| window == b"\r\n\r\n") {
            let mut chunk = [0; 1024];
            let bytes_read = tls_stream.read(&mut chunk).await?;
            ensure!(
                bytes_read > 0,
                "client closed before sending HTTP headers"
            );
            request.extend_from_slice(&chunk[..bytes_read]);
            ensure!(
                request.len() <= 16 * 1024,
                "HTTP request headers exceeded 16 KiB"
            );
        }

        ensure!(
            request.starts_with(b"GET /records HTTP/1.1\r\n"),
            "expected a GET request for /records"
        );
        tls_stream
            .write_all(
                b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nok",
            )
            .await?;
        Ok(())
    }

    /// Verifies a reqwest client presents its identity to a trusted loopback
    /// mTLS server.
    #[tokio::test]
    async fn client_certificate_authenticates_to_loopback_server()
    -> anyhow::Result<()> {
        let fixtures = local_certificate_fixtures()?;
        let mut client_roots = RootCertStore::empty();
        client_roots.add(fixtures.ca_certificate_der.clone())?;
        let verifier = WebPkiClientVerifier::builder_with_provider(
            Arc::new(client_roots),
            Arc::new(ring::default_provider()),
        )
        .build()?;
        let server_config = ServerConfig::builder_with_provider(Arc::new(
            ring::default_provider(),
        ))
        .with_safe_default_protocol_versions()?
        .with_client_cert_verifier(verifier)
        .with_single_cert(
            vec![fixtures.server_certificate_der],
            fixtures.server_private_key,
        )?;
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let acceptor = TlsAcceptor::from(Arc::new(server_config));
        let server_task = tokio::spawn(serve_one_request(listener, acceptor));

        let client = build_client(
            fixtures.ca_certificate_pem.as_bytes(),
            fixtures.client_identity_pem.as_bytes(),
        )?;
        let response = client
            .get(format!("https://localhost:{}/records", address.port()))
            .send()
            .await?
            .error_for_status()?;

        assert_eq!(response.text().await?, "ok");
        server_task.await??;
        Ok(())
    }
}
