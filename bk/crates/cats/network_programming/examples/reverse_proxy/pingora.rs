#![allow(dead_code)]
// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
//! Pingora example.
//!
//! Pingora is a high-performance and low-level async multithreaded library that
//! can be used build services on top of HTTP/1 and HTTP/2, TLS, or just
//! TCP/UDS. As a proxy, it supports HTTP/1 and HTTP/2 end-to-end, gRPC, and
//! websocket proxying.
//!
//! Features:
//! - Modern TLS with OpenSSL or BoringSSL (FIPS compatible).
//! - Zero-downtime upgrade.
//! - Integration with Syslog, Prometheus, Sentry, OpenTelemetry and other
//!   observability tools.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! async-trait="0.1"
//! pingora = { version = "0.5", features = [ "lb" ] }
//! ```
//!
//! Make sure that the following is installed on your system:
//! - Clang (for the boringssl optional feature),
//! - Perl 5 (for openssl).

// use std::net::SocketAddr;
// use std::sync::Arc;

// use async_trait::async_trait;
// use pingora::prelude::*;
use pingora::server::Server;

// FIXME decide what to implement
fn main() {
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();
    my_server.run_forever();
}

#[ignore = "Needs review"]
#[test]
fn require_network() {
    main();
}
// [finish NOW](https://github.com/john-cd/rust_howto/issues/812)
// <https://github.com/cloudflare/pingora/blob/main/pingora/examples/server.rs>
