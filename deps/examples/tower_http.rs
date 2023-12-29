// use tower_http::{
//     add_extension::AddExtensionLayer,
//     compression::CompressionLayer,
//     propagate_header::PropagateHeaderLayer,
//     sensitive_headers::SetSensitiveRequestHeadersLayer,
//     set_header::SetResponseHeaderLayer,
//     trace::TraceLayer,
//     validate_request::ValidateRequestHeaderLayer,
// };
// use tower::{ServiceBuilder, service_fn, BoxError};
// use http::{Request, Response, header::{HeaderName, CONTENT_TYPE, AUTHORIZATION}};
// use std::{sync::Arc, net::SocketAddr, convert::Infallible, iter::once};
// use bytes::Bytes;
// use http_body_util::Full;

// // Our request handler. This is where we would implement the application logic
// // for responding to HTTP requests...
// async fn handler(request: Request<Full<Bytes>>) -> Result<Response<Full<Bytes>>, BoxError> {
//     // ...
//     OK(())
// }

// struct DatabaseConnectionPool;

// impl DatabaseConnectionPool {
//     fn new() -> Self { DatabaseConnectionPool }
// }

// // Shared state across all request handlers --- in this case, a pool of database connections.
// struct State {
//     pool: DatabaseConnectionPool,
// }

// #[tokio::main]
// async fn main() {
//     // Construct the shared state.
//     let state = State {
//         pool: DatabaseConnectionPool::new(),
//     };

//     // Use tower's `ServiceBuilder` API to build a stack of tower middleware
//     // wrapping our request handler.
//     let service = ServiceBuilder::new()
//         // Mark the `Authorization` request header as sensitive so it doesn't show in logs
//         .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
//         // High level logging of requests and responses
//         .layer(TraceLayer::new_for_http())
//         // Share an `Arc<State>` with all requests
//         .layer(AddExtensionLayer::new(Arc::new(state)))
//         // Compress responses
//         .layer(CompressionLayer::new())
//         // Propagate `X-Request-Id`s from requests to responses
//         .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
//         // If the response has a known size set the `Content-Length` header
//         .layer(SetResponseHeaderLayer::overriding(CONTENT_TYPE, content_length_from_response))
//         // Authorize requests using a token
//         .layer(ValidateRequestHeaderLayer::bearer("passwordlol"))
//         // Accept only application/json, application/* and */* in a request's ACCEPT header
//         .layer(ValidateRequestHeaderLayer::accept("application/json"))
//         // Wrap a `Service` in our middleware stack
//         .service_fn(handler);
// }

fn main() {}
