// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Run a local HTTP reverse proxy with Pingora.
//!
//! Start an HTTP server on `127.0.0.1:8000`, then run
//! `cargo run -p network_programming --example reverse_proxy -- pingora` on a
//! Unix host. The proxy listens on `127.0.0.1:6188`, forwards each request to
//! the upstream, and adds an `X-Rust-Howto` response header. For example, start
//! the upstream with `python3 -m http.server 8000`, then request
//! `http://127.0.0.1:6188/` with `curl -i`.

use async_trait::async_trait;
use pingora::prelude::*;
use pingora::server::Server;

const UPSTREAM_HOST: &str = "127.0.0.1";
const UPSTREAM_PORT: u16 = 8000;
const PROXY_LISTEN_ADDRESS: &str = "127.0.0.1:6188";
const RESPONSE_HEADER_NAME: &str = "X-Rust-Howto";
const RESPONSE_HEADER_VALUE: &str = "Pingora";

/// Forwards HTTP requests to the local example upstream.
struct LocalProxy;

#[async_trait]
impl ProxyHttp for LocalProxy {
    type CTX = ();

    /// Returns the empty request context used by this stateless proxy.
    fn new_ctx(&self) -> Self::CTX {}

    /// Returns the fixed plain-HTTP upstream peer.
    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        Ok(Box::new(HttpPeer::new(
            (UPSTREAM_HOST, UPSTREAM_PORT),
            false,
            String::new(),
        )))
    }

    /// Adds the example response header after receiving an upstream response.
    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        add_example_response_header(upstream_response)
    }
}

/// Adds the identifying header to a Pingora response without starting a server.
fn add_example_response_header(response: &mut ResponseHeader) -> Result<()> {
    response.insert_header(RESPONSE_HEADER_NAME, RESPONSE_HEADER_VALUE)?;
    Ok(())
}

/// Starts the Pingora server and its local reverse-proxy service.
fn main() -> anyhow::Result<()> {
    run_proxy()
}

/// Starts the Pingora server and its local reverse-proxy service.
///
/// # Errors
/// Returns an error if server configuration cannot be initialized.
fn run_proxy() -> anyhow::Result<()> {
    let mut server = Server::new(None)?;
    server.bootstrap();

    let mut proxy = http_proxy_service(&server.configuration, LocalProxy);
    proxy.add_tcp(PROXY_LISTEN_ADDRESS);
    server.add_service(proxy);
    server.run_forever();
}
// ANCHOR_END: example

/// Runs the Pingora reverse proxy and propagates setup errors.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the response hook adds its header without a network request.
    #[test]
    fn response_filter_adds_example_header() -> anyhow::Result<()> {
        let mut response = ResponseHeader::build(200, None)?;
        add_example_response_header(&mut response)?;

        let value =
            response.headers.get(RESPONSE_HEADER_NAME).ok_or_else(|| {
                anyhow::anyhow!("the example response header is missing")
            })?;
        assert_eq!(value.to_str()?, RESPONSE_HEADER_VALUE);

        Ok(())
    }

    /// Starts the proxy for a manual local upstream test.
    #[test]
    #[ignore = "requires a local upstream on 127.0.0.1:8000 and a proxy listener"]
    fn require_network() -> anyhow::Result<()> {
        main()
    }
}
