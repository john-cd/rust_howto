#![allow(unused_imports)]
#![allow(dead_code)]

use bytes::Bytes;
use http::{
    header::{HeaderName, AUTHORIZATION, CONTENT_TYPE},
    Request, Response,
};
use http_body_util::Full;
use std::{convert::Infallible, iter::once, net::SocketAddr, sync::Arc};
use tower::{service_fn, BoxError, ServiceBuilder};
use tower_http::{
    add_extension::AddExtensionLayer, compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer, sensitive_headers::SetSensitiveRequestHeadersLayer,
    set_header::SetResponseHeaderLayer, trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
};

// Our request handler. This is where we would implement the application logic
// for responding to HTTP requests...
async fn handler(_request: Request<Full<Bytes>>) -> Result<Response<Full<Bytes>>, BoxError> {
    let empty_body = Full::new(Bytes::new());
    let builder = Response::builder()
        .header("X-Custom-Foo", "bar")
        .status(http::status::StatusCode::OK);
    Ok(builder.body(empty_body).unwrap())
}

struct DatabaseConnectionPool;

impl DatabaseConnectionPool {
    fn new() -> Self {
        DatabaseConnectionPool
    }
}

// Shared state across all request handlers --- in this case, a pool of database connections.
struct State {
    pool: DatabaseConnectionPool,
}

#[tokio::main]
async fn main() {
    // Construct the shared state.
    let state = State {
        pool: DatabaseConnectionPool::new(),
    };

    let content_length_from_response = 0;

    // Use tower's `ServiceBuilder` API to build a stack of tower middleware
    // wrapping our request handler.
    let _service = ServiceBuilder::new()
        // Mark the `Authorization` request header as sensitive so it doesn't show in logs
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        // High level logging of requests and responses
        .layer(TraceLayer::new_for_http())
        // Share an `Arc<State>` with all requests
        .layer(AddExtensionLayer::new(Arc::new(state)))
        // Compress responses
        .layer(CompressionLayer::new())
        // Propagate `X-Request-Id`s from requests to responses
        .layer(PropagateHeaderLayer::new(HeaderName::from_static(
            "x-request-id",
        )))
        // If the response has a known size set the `Content-Length` header
        .layer(SetResponseHeaderLayer::overriding(
            CONTENT_TYPE,
            content_length_from_response,
        ))
        // Authorize requests using a token
        //.layer(ValidateRequestHeaderLayer::bearer("passwordlol"))
        // Accept only application/json, application/* and */* in a request's ACCEPT header
        //.layer(ValidateRequestHeaderLayer::accept("application/json"))
        // Wrap a `Service` in our middleware stack
        .service_fn(handler);
}
