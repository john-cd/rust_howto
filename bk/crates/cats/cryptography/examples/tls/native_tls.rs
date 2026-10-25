#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Build a platform TLS connector with `native-tls` and use it.
//!
//! `native-tls` delegates to the operating system's TLS stack (`SChannel` on
//! Windows, Secure Transport on macOS, `OpenSSL` elsewhere), so it trusts the
//! system certificate store.

use std::io::Read as _;
use std::io::Write as _;
use std::net::TcpStream;

use native_tls::Protocol;
use native_tls::TlsConnector;

/// Builds a connector that refuses anything older than TLS 1.2.
fn connector() -> native_tls::Result<TlsConnector> {
    TlsConnector::builder()
        .min_protocol_version(Some(Protocol::Tlsv12))
        .build()
}

/// Opens a TLS connection to `host:443`, sends a `HEAD` request and returns
/// the HTTP status line.
fn status_line(host: &str) -> anyhow::Result<String> {
    let tcp = TcpStream::connect((host, 443))?;
    // The handshake verifies the server certificate against `host`.
    let mut tls = connector()?.connect(host, tcp)?;
    write!(
        tls,
        "HEAD / HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    )?;
    let mut response = String::new();
    tls.read_to_string(&mut response)?;
    Ok(response.lines().next().unwrap_or_default().to_owned())
}

/// Builds the connector; call [`status_line`] to also use the network.
fn main() -> anyhow::Result<()> {
    let _connector = connector()?;
    println!("Built a native-tls connector with a TLS 1.2 minimum.");
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    use super::*;

    /// The example runs end to end.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }

    /// A peer that does not speak TLS makes the handshake fail instead of
    /// hanging or succeeding.
    #[test]
    fn handshake_with_non_tls_peer_fails() -> anyhow::Result<()> {
        let listener = TcpListener::bind(("127.0.0.1", 0))?;
        let port = listener.local_addr()?.port();
        let server = std::thread::spawn(move || -> std::io::Result<()> {
            let (mut stream, _) = listener.accept()?;
            // Not a TLS record; the stream closes when dropped.
            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")
        });
        let tcp = TcpStream::connect(("127.0.0.1", port))?;
        let result = connector()?.connect("localhost", tcp);
        assert!(result.is_err());
        // The server thread may fail if the client closes first; that is fine.
        let _ = server.join();
        Ok(())
    }

    /// Needs network access and a trusted system certificate store.
    #[test]
    #[ignore = "requires network access"]
    fn fetches_a_status_line_over_tls() -> anyhow::Result<()> {
        let line = status_line("example.com")?;
        assert!(line.starts_with("HTTP/1.1 "), "unexpected: {line}");
        Ok(())
    }
}
