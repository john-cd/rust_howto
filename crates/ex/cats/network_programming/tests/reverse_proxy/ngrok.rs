// // ANCHOR: example
// // ANCHOR_END: example

// use std::convert::Infallible;

// use hyper::Request;
// use hyper::Response;
// use hyper::Server;
// use hyper::body::Body;
// use hyper::service::service_fn;
// use hyper::service_fn::make_service_fn;
// use tokio::runtime;

// async fn handle_request(
//     _req: Request<Body>,
// ) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new(Body::from("Hello, ngrok!")))
// }

// #[tokio::main]
// async fn main() {
//     let make_svc = make_service_fn(|_conn| async {
//         Ok::<_, Infallible>(service_fn(handle_request))
//     });

//     let addr = ([127, 0, 0, 1], 3000).into();
//     let server = Server::bind(&addr).serve(make_svc);

//     println!("Server running on http://{}", addr);
//     if let Err(e) = server.await {
//         eprintln!("server error: {}", e);
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/811)
