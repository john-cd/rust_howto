//! A small HTTP/1 server built directly with Hyper and Tokio.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use std::convert::Infallible;

use bytes::Bytes;
use http_body_util::Full;
use hyper::Method;
use hyper::Request;
use hyper::Response;
use hyper::StatusCode;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

/// Runs the HTTP/1 server on loopback port 3000.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 3000)).await?;
    println!("Listening on http://127.0.0.1:3000");

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let io = TokioIo::new(stream);
            let connection = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request));

            if let Err(error) = connection.await {
                eprintln!("Error serving HTTP connection: {error}");
            }
        });
    }
}

/// Converts a Hyper request into a response for the small example API.
async fn handle_request(
    request: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(response_for(request.method(), request.uri().path()))
}

/// Returns a response for a method and path in the example API.
fn response_for(method: &Method, path: &str) -> Response<Full<Bytes>> {
    let (status, body) = if method == Method::GET {
        match path {
            "/" => (StatusCode::OK, "Hello, world!"),
            "/health" => (StatusCode::OK, "OK"),
            _ => (StatusCode::NOT_FOUND, "Not found"),
        }
    } else {
        (StatusCode::METHOD_NOT_ALLOWED, "Method not allowed")
    };

    let mut response =
        Response::new(Full::new(Bytes::from_static(body.as_bytes())));
    *response.status_mut() = status;
    response
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt as _;
    use hyper::Method;
    use hyper::StatusCode;

    use super::response_for;

    /// Returns a greeting for a GET request to the root path.
    #[tokio::test]
    async fn get_root_returns_hello() {
        let response = response_for(&Method::GET, "/");
        assert_eq!(response.status(), StatusCode::OK);

        let body = response
            .into_body()
            .collect()
            .await
            .expect("the full response body is infallible")
            .to_bytes();
        assert_eq!(body.as_ref(), b"Hello, world!");
    }

    /// Returns not found for an unknown GET route.
    #[tokio::test]
    async fn get_unknown_path_returns_not_found() {
        let response = response_for(&Method::GET, "/missing");
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    /// Rejects methods other than GET.
    #[tokio::test]
    async fn post_returns_method_not_allowed() {
        let response = response_for(&Method::POST, "/");
        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}

// Update Hyper server example to 1.0 API and implement request routing
// <https://hyper.rs/>
// <https://hyper.rs/guides/1/server/hello-world/>
// <https://github.com/hyperium/hyper/tree/master/examples>
// <https://github.com/hyperium/hyper/blob/master/examples/README.md>
