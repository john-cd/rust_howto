#![allow(dead_code)]

use std::iter::once;
use std::sync::Arc;

use bytes::Bytes;
use http::header::HeaderName;
use http::header::AUTHORIZATION;
use http::header::CONTENT_TYPE;
use http::Request;
use http::Response;
use http_body_util::Full;
use tower::BoxError;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
// use tower_http::validate_request::ValidateRequestHeaderLayer;

// Our request handler. This is where we would implement the
// application logic for responding to HTTP requests...
async fn handler(
    _request: Request<Full<Bytes>>,
) -> Result<Response<Full<Bytes>>, BoxError> {
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

// Shared state across all request handlers -
// in this case, a pool of database connections.
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

    // Use tower's `ServiceBuilder` API to build a stack of tower
    // middleware wrapping our request handler.
    let _service = ServiceBuilder::new()
        // Mark the `Authorization` request header as sensitive
        // so it doesn't show in logs
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
        //// Authorize requests using a token
        //.layer(ValidateRequestHeaderLayer::bearer("passwordlol"))
        //// Accept only application/json, application/* and */*
        //// in a request's ACCEPT header
        //.layer(ValidateRequestHeaderLayer::accept("application/json"))
        // Wrap the `Service` in our middleware stack
        .service_fn(handler);
}
