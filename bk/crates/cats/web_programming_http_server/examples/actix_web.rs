// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates a basic Actix Web server.
//!
//! It defines a simple HTTP server that listens on `127.0.0.1:8080` and
//! responds with "Hello, world!" to GET requests on the root path ("/").

use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;

/// Returns a `HttpResponse` with status code 200 (OK) and the body "Hello,
/// world!".
///
/// This is a basic request handler that implements the `Responder` trait.
#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

/// Starts the Actix Web server.
///
/// Binds to localhost on port 8080.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use actix_web::test as actix_test;

    use super::*;

    /// Verifies the server's network-facing response.
    #[actix_web::test]
    async fn require_network() -> std::io::Result<()> {
        let app = actix_test::init_service(App::new().service(greet)).await;
        let req = actix_test::TestRequest::get().uri("/").to_request();

        let resp = actix_test::call_service(&app, req).await;
        let body = actix_test::read_body(resp).await;

        assert_eq!(body, "Hello, world!");
        Ok(())
    }
}
