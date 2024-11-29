// ANCHOR: example
use std::convert::Infallible;

use anyhow::Result;
use bytes::Bytes;
use http::header;
use http::Method;
use http::Request;
use http::Response;
use http_body_util::Full;
use tower::Service;
use tower::ServiceBuilder;
use tower::ServiceExt;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

async fn handle(
    _request: Request<Full<Bytes>>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::default()))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cors = CorsLayer::new()
        // Allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // Allow requests from any origin
        .allow_origin(Any);

    let mut service = ServiceBuilder::new().layer(cors).service_fn(handle);

    let request = Request::builder()
        .header(header::ORIGIN, "https://example.com")
        .body(Full::default())
        .unwrap();

    let response = service.ready().await?.call(request).await?;
    println!("{:?}", response);
    assert_eq!(
        response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .unwrap(),
        "*",
    );
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
