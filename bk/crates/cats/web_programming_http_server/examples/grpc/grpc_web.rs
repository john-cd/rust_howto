// TODO Reviewed ??-??-2026.

#![allow(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Serve a tonic service to browsers with gRPC-Web.
//!
//! Browsers cannot speak gRPC over HTTP/2 directly. `GrpcWebLayer` translates
//! gRPC-Web requests (sent over HTTP/1.1 or HTTP/2) into gRPC, and a CORS layer
//! lets pages from other origins call the service. gRPC-Web does not support
//! client streaming; put Envoy in front instead if you need it.

use std::net::SocketAddr;

use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::AllowOrigin;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

use super::tonic::MyGreeter;
use super::tonic::helloworld::greeter_server::GreeterServer;

/// CORS settings for browser clients. Restrict `allow_origin` in production.
pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_headers(Any)
        .expose_headers([
            http::HeaderName::from_static("grpc-status"),
            http::HeaderName::from_static("grpc-message"),
        ])
}

/// Serves the `Greeter` service over gRPC and gRPC-Web on the same port.
pub async fn serve(addr: SocketAddr) -> anyhow::Result<()> {
    Server::builder()
        // Required: gRPC-Web may arrive over HTTP/1.1.
        .accept_http1(true)
        .layer(cors())
        .layer(GrpcWebLayer::new())
        .add_service(GreeterServer::new(MyGreeter::default()))
        .serve(addr)
        .await?;
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use bytes::BytesMut;
    use http_body_util::BodyExt;
    use http_body_util::Full;
    use tower::Layer as _;
    use tower::ServiceExt as _;

    use super::*;

    /// Encodes `HelloRequest { name }` in a gRPC length-prefixed frame.
    fn frame(name: &str) -> Bytes {
        // Protobuf: field 1, wire type 2 (length-delimited).
        let mut message = vec![0x0a, u8::try_from(name.len()).unwrap_or(0)];
        message.extend_from_slice(name.as_bytes());
        let mut out = BytesMut::new();
        out.extend_from_slice(&[0]); // Not compressed.
        out.extend_from_slice(&(message.len() as u32).to_be_bytes());
        out.extend_from_slice(&message);
        out.freeze()
    }

    #[tokio::test]
    async fn grpc_web_request_round_trip() -> anyhow::Result<()> {
        let service =
            GrpcWebLayer::new().layer(GreeterServer::new(MyGreeter::default()));
        let request = http::Request::post("/helloworld.Greeter/SayHello")
            .header("content-type", "application/grpc-web")
            .header("x-grpc-web", "1")
            .body(tonic::body::Body::new(Full::new(frame("Rust"))))?;

        let response = service.oneshot(request).await?;
        assert_eq!(response.status(), http::StatusCode::OK);
        assert_eq!(
            response.headers()["content-type"],
            "application/grpc-web+proto"
        );
        let body = response.into_body().collect().await?.to_bytes();
        let text = String::from_utf8_lossy(&body);
        assert!(text.contains("Hello Rust!"), "unexpected body: {text:?}");
        Ok(())
    }
}
