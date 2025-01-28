// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use actix_web::App;
// use actix_web::HttpResponse;
// use actix_web::HttpServer;
// use actix_web::Responder;
// use actix_web::web;

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
// fn test() -> std::io::Result<()> {
//     main()?;
//     Ok(())
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/863)
