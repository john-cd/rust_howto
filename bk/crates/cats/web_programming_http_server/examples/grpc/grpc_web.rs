// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Serve a tonic service to browsers with gRPC-Web.
//!
//! Browsers cannot speak gRPC over HTTP/2 directly. `GrpcWebLayer` translates
//! gRPC-Web requests (sent over HTTP/1.1 or HTTP/2) into gRPC, and a CORS layer
//! lets pages from other origins call the service. gRPC-Web does not support
//! client streaming; put Envoy in front instead if you need it.

use std::future::Future;

use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
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

/// Serves the `Greeter` service over gRPC and gRPC-Web on a bound listener.
///
/// # Errors
/// Returns an error if the server cannot serve the listener.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "serves until its shutdown signal is received")
)]
pub async fn serve<F>(listener: TcpListener, shutdown: F) -> anyhow::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    Server::builder()
        // Required: gRPC-Web may arrive over HTTP/1.1.
        .accept_http1(true)
        .layer(cors())
        .layer(GrpcWebLayer::new())
        .add_service(GreeterServer::new(MyGreeter::default()))
        .serve_with_incoming_shutdown(
            TcpListenerStream::new(listener),
            shutdown,
        )
        .await?;
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use bytes::BytesMut;

    use super::*;

    /// Encodes `HelloRequest { name }` in a gRPC length-prefixed frame.
    fn frame(name: &str) -> Bytes {
        // Protobuf: field 1, wire type 2 (length-delimited).
        let name_length = u8::try_from(name.len())
            .expect("test greeting names fit in a one-byte protobuf length");
        let mut message = vec![0x0a, name_length];
        message.extend_from_slice(name.as_bytes());
        let mut out = BytesMut::new();
        out.extend_from_slice(&[0]); // Not compressed.
        let message_length = u32::try_from(message.len())
            .expect("test protobuf messages fit in a four-byte frame length");
        out.extend_from_slice(&message_length.to_be_bytes());
        out.extend_from_slice(&message);
        out.freeze()
    }

    /// Verifies the service handles a gRPC-Web request over a loopback socket.
    #[tokio::test]
    async fn grpc_web_request_round_trip_over_loopback() -> anyhow::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let (shutdown_sender, shutdown_receiver) =
            tokio::sync::oneshot::channel();
        let server = tokio::spawn(serve(listener, async move {
            let _ = shutdown_receiver.await;
        }));
        tokio::task::yield_now().await;

        let request_result = async {
            let response = reqwest::Client::builder()
                .http1_only()
                .build()?
                .post(format!("http://{address}/helloworld.Greeter/SayHello"))
                .header("content-type", "application/grpc-web")
                .header("x-grpc-web", "1")
                .body(frame("Rust"))
                .send()
                .await?;
            assert_eq!(response.status().as_u16(), 200);
            assert_eq!(
                response.headers()["content-type"],
                "application/grpc-web+proto"
            );
            let body = response.bytes().await?;
            assert!(
                body.windows(b"Hello Rust!".len())
                    .any(|window| window == b"Hello Rust!"),
                "unexpected gRPC-Web body: {body:?}"
            );
            anyhow::Result::<()>::Ok(())
        }
        .await;

        let shutdown_result = shutdown_sender.send(());
        let server_result = server.await;
        request_result?;
        shutdown_result.map_err(|()| {
            anyhow::anyhow!("gRPC-Web server exited before shutdown")
        })?;
        server_result??;
        Ok(())
    }
}
