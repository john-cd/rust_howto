// // // ANCHOR: example
// // // COMING SOON
// // // ANCHOR_END: example
// //! This example demonstrates a basic Actix Web server.
// //!
// //! It defines a simple HTTP server that listens on `127.0.0.1:8080` and
// //! responds with "Hello, world!" to GET requests on the root path ("/").

// use actix_web::App;
// use actix_web::HttpResponse;
// use actix_web::HttpServer;
// use actix_web::Responder;
// use actix_web::web;

// /// Returns a `HttpResponse` with status code 200 (OK) and the body "Hello,
// /// world!".
// async fn greet() -> impl Responder {
//     HttpResponse::Ok().body("Hello, world!")
// }
// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/", web::get().to(greet)))
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }

// #[test]
// fn require_network() -> std::io::Result<()> {
//     main()?;
//     Ok(())
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/863)
