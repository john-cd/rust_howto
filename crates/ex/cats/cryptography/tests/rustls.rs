// // ANCHOR: example
// use std::io;
// use std::sync::Arc;

// // Common configuration for (typically) all connections made by a program:
// use rustls::ClientConfig;
// use rustls::pki_types::ServerName;
// // Extension trait, adding utility methods to all AsyncRead types
// use tokio::io::AsyncReadExt;
// // Extension trait, adding utility methods to all AsyncWrite types
// use tokio::io::AsyncWriteExt;

// // Rustls is a TLS library that aims to provide a good level of cryptographic
// // security, requires no configuration to achieve that security,
// // and provides no unsafe features or obsolete cryptography by default.
// // Rustls implements TLS1.2 and TLS1.3 for both clients and servers

// // If you’re already using Tokio for an async runtime you may prefer to use
// // `tokio-rustls` instead of interacting with `rustls` directly.

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // Load the root certificate store,
//     // a container for root certificates able to provide
//     // a root-of-trust for connection authentication:
//     let root_store = rustls::RootCertStore::from_iter(
//         // Mozilla's root certificates for use with the webpki or rustls
//         // crates: https://github.com/rustls/webpki-roots
//         webpki_roots::TLS_SERVER_ROOTS.iter().cloned(),
//     );

//     // Create a `ClientConfig` with the root certificate store
//     // - Create a builder for a client configuration with the process-default
//     //   CryptoProvider and safe protocol version defaults.
//     let config = ClientConfig::builder()
//         .with_root_certificates(root_store)
//         // Disable client authentication (most common)
//         .with_no_client_auth();
//         // Create a wrapper around a `rustls::ClientConfig`,
//         // providing an async `connect` method
//     let connector = tokio_rustls::TlsConnector::from(Arc::new(config));

//     // Define the domain and server address
//     let domain = "example.com";
//     let addr = "23.215.0.136:443";

//     // Connect to the server
//     let stream = tokio::net::TcpStream::connect(addr).await?;
//     //
//     let domain = ServerName::try_from(domain).map_err(|_| {
//         io::Error::new(io::ErrorKind::InvalidInput, "Invalid domain")
//     })?;
//     //
//     let mut stream = connector.connect(domain, stream).await?;

//     // Send a HTTP GET request
//     stream
//         .write_all(
// b"GET / HTTP/1.1\r\nHost:example.com\r\nConnection:close\r\n\r\n")
//         .await?;

//     // Read the response
//     let mut response = Vec::new();
//     stream.read_to_end(&mut response).await?;

//     // Print the response
//     println!("{}", String::from_utf8_lossy(&response));

//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/700) need full integration test; document further; review https://github.com/rustls/rustls/tree/main/examples
// // review tokio-rustls
