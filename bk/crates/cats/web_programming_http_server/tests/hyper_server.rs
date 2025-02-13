// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use std::convert::Infallible;
// use std::net::SocketAddr;

// use hyper::Request;
// use hyper::Response;
// use hyper::body;
// use hyper::server;
// use hyper::service::service_fn;

// // Add the following to your `Cargo.toml`:
// // [dependencies]
// // hyper = { version = "1", features = ["full"] } # or latest version
// // tokio = { version = "1", features = ["full"] }
// // http-body-util = "0.1"
// // hyper-util = { version = "0.1", features = ["full"] }

// For a server type that can handle both HTTP/1 and HTTP/2 at the same time,
// use the server::conn::auto::Builder from hyper-util.

// async fn hello_world(_: Request<Body>) -> Result<Response<Body>, Infallible>
// {     Ok(Response::new(Body::from("Hello, world!")))
// }

// // A Service lets us define how our server will respond to incoming requests.
// It // represents an async function that takes a Request and returns a Future.
// When // the processing of this future is complete, it will resolve to a
// Response or // an error.

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

//     // We create a TcpListener and bind it to 127.0.0.1:3000
//     let listener = TcpListener::bind(addr).await?;

//     // We start a loop to continuously accept incoming connections
//     loop {
//         let (stream, _) = listener.accept().await?;

//         // Use an adapter to access something implementing `tokio::io` traits
// as         // if they implement `hyper::rt` IO traits.
//         let io = TokioIo::new(stream);

//         // Spawn a tokio task to serve multiple connections concurrently
//         tokio::task::spawn(async move {
//             // Finally, we bind the incoming connection to our `hello`
// service             if let Err(err) = http1::Builder::new()
//                 // `service_fn` converts our function in a `Service`
//                 .serve_connection(io, service_fn(hello))
//                 .await
//             {
//                 eprintln!("Error serving connection: {:?}", err);
//             }
//         });
//     }

//     Ok(())
// }

// #[test]
// fn require_network() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [P0](https://github.com/john-cd/rust_howto/issues/866)
// // https://hyper.rs/
// // https://hyper.rs/guides/1/server/hello-world/
// // https://github.com/hyperium/hyper/tree/master/examples
// // https://github.com/hyperium/hyper/blob/master/examples/README.md
