// // ANCHOR: example
// // ANCHOR_END: example

// use std::convert::Infallible;
// use std::net::SocketAddr;

// use hyper::body;
// use hyper::Request;
// use hyper::Response;
// use hyper::server;
// use hyper::service::make_service_fn;
// use hyper::service::service_fn;

// async fn hello_world(_: Request<Body>) -> Result<Response<Body>, Infallible>
// {     Ok(Response::new(Body::from("Hello, world!")))
// }

// #[tokio::main]
// async fn main() {
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

//     let make_svc = make_service_fn(|_conn| async {
//         Ok::<_, Infallible>(service_fn(hello_world))
//     });

//     let server = Server::bind(&addr).serve(make_svc);

//     println!("Listening on http://{}", addr);

//     if let Err(e) = server.await {
//         eprintln!("Server error: {}", e);
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [P0](https://github.com/john-cd/rust_howto/issues/866)
