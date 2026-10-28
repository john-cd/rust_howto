// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Build a platform TLS connector with `native-tls`.
//!
//! This example builds a `TlsConnector` and can make an opt-in verified TLS
//! handshake with a server on port 443.

use std::env;
use std::env::VarError;
use std::net::TcpStream;

use anyhow::Context as _;
use native_tls::TlsConnector;
use native_tls::TlsStream;

/// Builds a connector using the platform's default TLS configuration.
fn build_connector() -> anyhow::Result<TlsConnector> {
    Ok(TlsConnector::builder().build()?)
}

/// Opens a TCP connection and completes a certificate-validated TLS handshake.
///
/// # Errors
/// Returns an error if the connection or TLS handshake fails.
fn connect_tls(
    connector: &TlsConnector,
    host: &str,
) -> anyhow::Result<TlsStream<TcpStream>> {
    let tcp_stream = TcpStream::connect((host, 443))
        .with_context(|| format!("could not connect to {host}:443"))?;
    Ok(connector.connect(host, tcp_stream)?)
}

/// Builds a connector and optionally connects to `NATIVE_TLS_HOST`.
///
/// Without the environment variable this example does not access the network.
///
/// # Errors
/// Returns an error if the connector cannot be built, the environment variable
/// is invalid, or an opted-in connection fails.
fn main() -> anyhow::Result<()> {
    let connector = build_connector()?;
    match env::var("NATIVE_TLS_HOST") {
        Ok(host) => {
            let _tls_stream = connect_tls(&connector, &host)?;
            println!(
                "Completed a verified native TLS handshake with {host}:443."
            );
        }
        Err(VarError::NotPresent) => println!(
            "Built a native-tls connector. Set NATIVE_TLS_HOST to opt in to a TLS handshake."
        ),
        Err(error) => return Err(error.into()),
    }
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connector_builds_without_network_access() -> anyhow::Result<()> {
        build_connector()?;
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
