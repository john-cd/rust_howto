// TODO Reviewed ??-??-2026.

// ANCHOR: example
use std::env;
use std::net::TcpStream;

use anyhow::Context as _;
use openssl::ssl::SslConnector;
use openssl::ssl::SslMethod;
use openssl::ssl::SslStream;
use openssl::ssl::SslVerifyMode;
use openssl::x509::X509;

/// Builds a client connector that validates the certificate chain and host name.
fn build_connector(test_ca: Option<X509>) -> anyhow::Result<SslConnector> {
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_verify(SslVerifyMode::PEER);
    if let Some(certificate) = test_ca {
        builder.cert_store_mut().add_cert(certificate)?;
    }
    Ok(builder.build())
}

/// Opens a TCP connection and completes a verified TLS handshake.
fn connect_tls(
    connector: &SslConnector,
    host: &str,
    port: u16,
) -> anyhow::Result<SslStream<TcpStream>> {
    let tcp = TcpStream::connect((host, port))
        .with_context(|| format!("could not connect to {host}:{port}"))?;
    connector
        .connect(host, tcp)
        .map_err(|error| anyhow::anyhow!("TLS handshake failed: {error}"))
}

fn main() -> anyhow::Result<()> {
    let host = match env::var("TLS_HOST") {
        Ok(host) => host,
        Err(env::VarError::NotPresent) => {
            println!("Set TLS_HOST to opt in to a verified OpenSSL TLS handshake.");
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let port = env::var("TLS_PORT")
        .unwrap_or_else(|_| "443".to_owned())
        .parse::<u16>()
        .context("TLS_PORT must be a valid port number")?;

    let connector = build_connector(None)?;
    let _stream = connect_tls(&connector, &host, port)?;
    println!("Completed a verified OpenSSL TLS handshake with {host}:{port}.");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use std::io::Read as _;
    use std::io::Write as _;
    use std::net::TcpListener;
    use std::thread;

    use openssl::pkey::PKey;
    use openssl::ssl::SslAcceptor;
    use openssl::ssl::SslMethod;
    use openssl::x509::X509;

    use super::*;

    #[test]
    fn client_verifies_a_local_certificate_and_exchanges_data() -> anyhow::Result<()> {
        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(vec!["localhost".to_owned()])?;
        let certificate = X509::from_der(cert.der().as_ref())?;
        let key = PKey::private_key_from_der(&signing_key.serialize_der())?;

        let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls_server())?;
        acceptor.set_certificate(&certificate)?;
        acceptor.set_private_key(&key)?;
        acceptor.check_private_key()?;
        let acceptor = acceptor.build();

        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();
        let server = thread::spawn(move || -> anyhow::Result<()> {
            let (socket, _) = listener.accept()?;
            let mut stream = acceptor
                .accept(socket)
                .map_err(|error| anyhow::anyhow!("TLS accept failed: {error}"))?;
            let mut request = [0; 4];
            stream.read_exact(&mut request)?;
            stream.write_all(&request)?;
            Ok(())
        });

        let connector = build_connector(Some(certificate))?;
        let mut stream = connect_tls(&connector, "localhost", port)?;
        stream.write_all(b"ping")?;
        let mut response = [0; 4];
        stream.read_exact(&mut response)?;
        assert_eq!(&response, b"ping");
        server
            .join()
            .map_err(|_| anyhow::anyhow!("TLS server thread panicked"))??;
        Ok(())
    }
}
