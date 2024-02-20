use std::convert::Infallible;
use std::error::Error;

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

#[tokio::test]
async fn test() -> Result<(), Box<dyn Error>> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let mut service = ServiceBuilder::new().layer(cors).service_fn(handle);

    let request = Request::builder()
        .header(header::ORIGIN, "https://example.com")
        .body(Full::default())
        .unwrap();

    let response = service.ready().await?.call(request).await?;

    assert_eq!(
        response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .unwrap(),
        "*",
    );
    Ok(())
}
