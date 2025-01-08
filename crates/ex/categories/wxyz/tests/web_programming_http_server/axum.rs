// // ANCHOR: example
// use std::net::SocketAddr;

// use axum::Router;
// use axum::handler::get;

// async fn hello() -> &'static str {
//     "Hello, world!"
// }

// #[tokio::main]
// async fn main() {
//     // build our application with a route
//     let app = Router::new().route("/", get(hello));

//     // run it
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("Listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
// // ANCHOR_END: example

// #[test]
// #[ignore = "not yet implemented"]
// fn test() {
//     main();
// }
// // [P0](https://github.com/john-cd/rust_howto/issues/865)
