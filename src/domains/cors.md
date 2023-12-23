# CORS

[https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS]( https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS )

Using the Tower ecosystem:

```rust,editable,ignore
use http::{Request, Response, Method, header};
use http_body_util::Full;
use bytes::Bytes;
use tower::{ServiceBuilder, ServiceExt, Service};
use tower_http::cors::{Any, CorsLayer};
use std::convert::Infallible;
use std::error::Error;

async fn handle(request: Request<Full<Bytes>>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::default()))
}

async fn main() -> Result<(), Box<dyn Error>> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let mut service = ServiceBuilder::new()
        .layer(cors)
        .service_fn(handle);

    let request = Request::builder()
        .header(header::ORIGIN, "https://example.com")
        .body(Full::default())
        .unwrap();

    let response = service
        .ready()
        .await?
        .call(request)
        .await?;

    assert_eq!(
        response.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(),
        "*",
    );
}
```
