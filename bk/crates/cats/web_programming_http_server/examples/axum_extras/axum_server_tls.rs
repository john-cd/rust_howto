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

/// Builds the router.
pub fn app() -> Router {
    Router::new().route("/", get(|| async { "hello over TLS" }))
}

/// Serves `app` over HTTPS until `handle` asks the server to shut down.
///
/// In production, load the certificate and key with
/// `RustlsConfig::from_pem_file("cert.pem", "key.pem")` instead.
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

/// Stops accepting connections and waits up to `grace` for in-flight
/// requests to finish.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "stops the TLS server; exercised by the test")
)]
pub fn shutdown(handle: &Handle<SocketAddr>, grace: Duration) {
    handle.graceful_shutdown(Some(grace));
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

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
            .add_root_certificate(reqwest::Certificate::from_pem(
                cert.pem().as_bytes(),
            )?)
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
