#![allow(dead_code)]

// ANCHOR: example
//! Build a Rustls client configuration and async connector.
//!
//! This example creates a `ClientConfig` with an empty root store and
//! wraps it in a tokio-compatible TLS connector.
//!
//! TODO Reviewed ??-??-2026

use std::sync::Arc;

use rustls::ClientConfig;
use rustls::RootCertStore;
use rustls::crypto::aws_lc_rs;

fn main() -> anyhow::Result<()> {
    let root_store = RootCertStore::empty();
    let config = ClientConfig::builder_with_provider(Arc::new(
        aws_lc_rs::default_provider(),
    ))
    .with_safe_default_protocol_versions()?
    .with_root_certificates(root_store)
    .with_no_client_auth();
    let _connector = tokio_rustls::TlsConnector::from(Arc::new(config));

    println!(
        "Built a rustls client configuration and async connector. Add root certificates before performing a real TLS handshake."
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
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
    // [review tokio-rustls; need full integration test; document further; review https://github.com/rustls/rustls/tree/main/examples](https://github.com/john-cd/rust_howto/issues/700)
}
