// ANCHOR: example
//! This example demonstrates a simple web application using the
//! `shuttle_runtime` and `shuttle_axum` crates.
//!
//! The application defines a single route (`/`) that returns
//! "Hello, Shuttle!"  when accessed.
//!
//! - We define an async main function with the `shuttle_runtime::main`
//!   attribute to initialize the application.
//! - Inside the `main` function, we create an instance of axum `Router` and
//!   define a single route (/) that returns "Hello, Shuttle!" when accessed.
//! - We return an instance of `ShuttleAxum` with the initialized router.

use axum::Router;
use axum::routing::get;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}

// this is a function that returns a static string
// all functions used as endpoints must return a HTTP-compatible response
async fn hello_world() -> &'static str {
    "Hello, Shuttle!"
}
// ANCHOR_END: example

// Local tests are omitted since shuttle depends on a specific runtime.
// The code itself is type-checked during compilation.

// Workaround for unused code warnings when not compiled by shuttle:
#[allow(dead_code)]
fn dummy() {
    let _ = hello_world();
}
