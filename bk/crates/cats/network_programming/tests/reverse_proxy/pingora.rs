// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This is a simple example of a Pingora server that listens on port 8080
// //! and responds with "Hello, world!".
// //!
// //! Pingora is a high-performance and low-level HTTP library in Rust.
// //! - HTTP 1.x and HTTP 2
// //! - Modern TLS with OpenSSL or BoringSSL (FIPS compatible)
// //! - Zero downtime upgrade
// use pingora::prelude::*;
// use std::net::SocketAddr;
// use pingora::server::Server;

// async fn handle_request(_: Request) -> Response {
//     Response::new(200, "Hello, world!".into())
// }

// /// Create a basic HTTP server that responds with "Hello, world!".
// #[tokio::main]
// async fn main() {
// let addr: SocketAddr = "127.0.0.1:8080".parse().expect("Unable to parse
// socket address");
// // The server object represents an entire pingora
// // server process, which may have multiple independent services.
// let server = Server::bind(&addr).serve(handle_request);
// println!("Listening on http://{}", addr);
//     server.await.expect("Server failed");
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish; require network NOW](https://github.com/john-cd/rust_howto/issues/812)
// // https://github.com/cloudflare/pingora/blob/main/pingora/examples/server.rs
