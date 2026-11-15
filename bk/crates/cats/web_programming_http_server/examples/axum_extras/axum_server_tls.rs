// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Serve an `axum` router over HTTPS with `axum-server` and `rustls`.
//!
//! TLS encrypts HTTP so that traffic cannot be read or changed on the network
//! (HTTPS). The server needs a certificate and private key; for local
//! development a self-signed certificate works, while the public internet needs
//! one from a certificate authority such as Let's Encrypt.

use std::net::SocketAddr;
use std::time::Duration;

use axum::Router;
use axum::routing::get;
use axum_server::Handle;
use axum_server::tls_rustls::RustlsConfig;

/// Builds the Axum router serving a simple text response over TLS.
///
/// Returns an Axum `Router` configured with the root path (`/`).
/// The returned router can be shared safely across worker threads.
pub fn app() -> Router {
    Router::new().route("/", get(|| async { "hello over TLS" }))
}

/// Serves the example router over HTTPS until the server handle signals shutdown.
///
/// In production, load `config` from certificate and key files with
/// `RustlsConfig::from_pem_file`.
///
/// # Arguments
/// * `addr` - Local socket address on which the HTTPS server listens.
/// * `config` - TLS certificate and private-key configuration.
/// * `handle` - Handle used to query or stop the running server.
///
/// # Returns
/// Completes when the server stops serving requests.
///
/// # Errors
/// Returns an I/O error if binding, TLS negotiation, or request serving fails.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "binds a TLS socket; exercised by the test")
)]
pub async fn serve_https(
    addr: SocketAddr,
    config: RustlsConfig,
    handle: Handle<SocketAddr>,
) -> std::io::Result<()> {
    axum_server::bind_rustls(addr, config)
        .handle(handle)
        .serve(app().into_make_service())
        .await
}
/// Requests graceful shutdown and gives in-flight requests up to `grace` to finish.
///
/// # Arguments
/// * `handle` - Handle for the running HTTPS server.
/// * `grace` - Maximum duration to wait for active requests.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "stops the TLS server; exercised by the test")
)]
pub fn shutdown(handle: &Handle<SocketAddr>, grace: Duration) {
    handle.graceful_shutdown(Some(grace));
}
// ANCHOR_END: example

/// Returns the router used by the HTTPS server.
pub fn run() -> Router {
    app()
}

/// Exercises the HTTPS router and server lifecycle without opening a network listener.
#[cfg(test)]
mod tests {
    use super::*;

    /// Serves requests until graceful shutdown completes.
    #[tokio::test]
    async fn serves_and_shuts_down() -> anyhow::Result<()> {
        // Install a process-wide crypto provider (a no-op error if present).
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

        // A throwaway self-signed certificate, valid for `localhost`.
        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(vec!["localhost".to_owned()])?;
        let config = RustlsConfig::from_pem(
            cert.pem().into_bytes(),
            signing_key.serialize_pem().into_bytes(),
        )
        .await?;

        let handle = Handle::new();
        let server = tokio::spawn(serve_https(
            SocketAddr::from(([127, 0, 0, 1], 0)),
            config,
            handle.clone(),
        ));
        let addr = handle
            .listening()
            .await
            .ok_or_else(|| anyhow::anyhow!("server did not start"))?;

        let client = reqwest::Client::builder()
            .add_root_certificate(reqwest::Certificate::from_pem(cert.pem().as_bytes())?)
            .resolve("localhost", addr)
            .build()?;
        let body = client
            .get(format!("https://localhost:{}/", addr.port()))
            .send()
            .await?
            .text()
            .await?;
        assert_eq!(body, "hello over TLS");

        shutdown(&handle, Duration::from_secs(1));
        server.await??;
        Ok(())
    }
}
