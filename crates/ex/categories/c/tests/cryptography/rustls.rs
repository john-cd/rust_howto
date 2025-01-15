// // ANCHOR: example
// // ANCHOR_END: example

// use std::io;
// use std::sync::Arc;

// use rustls::ClientConfig;
// use rustls::RootCertStore;
// use rustls::pki_types::ServerName;
// use tokio::io::AsyncReadExt;
// use tokio::io::AsyncWriteExt;
// use tokio::net::TcpStream;
// use tokio_rustls::TlsConnector;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Load the root certificate store
//     let mut root_cert_store = RootCertStore::empty();
//     root_cert_store.add_server_trust_anchors(
//         webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
//             OwnedTrustAnchor::from_subject_spki_name_constraints(
//                 ta.subject,
//                 ta.spki,
//                 ta.name_constraints,
//             )
//         }),
//     );

//     // Create a `ClientConfig` with the root certificate store
//     let config = ClientConfig::builder()
//         .with_safe_defaults()
//         .with_root_certificates(root_cert_store)
//         .with_no_client_auth();
//     let connector = TlsConnector::from(Arc::new(config));

//     // Define the domain and server address
//     let domain = "example.com";
//     let addr = "93.184.216.34:443";

//     // Connect to the server
//     let stream = TcpStream::connect(addr).await?;
//     let domain = rustls::ServerName::try_from(domain).map_err(|_| {
//         io::Error::new(io::ErrorKind::InvalidInput, "Invalid domain")
//     })?;
//     let mut stream = connector.connect(domain, stream).await?;

//     // Send a HTTP GET request
//     stream
//         .write_all(
//             b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection:
// close\r\n\r\n",         )
//         .await?;

//     // Read the response
//     let mut response = Vec::new();
//     stream.read_to_end(&mut response).await?;

//     // Print the response
//     println!("{}", String::from_utf8_lossy(&response));

//     Ok(())
// }

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/700)
