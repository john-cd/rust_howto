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
//!
//! TODO Reviewed ??-??-2026

use axum::Router;
use axum::routing::get;

/// Creates the router of the web application.
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}

// A function that returns a static string.
// All functions used as endpoints must return a HTTP-compatible response
/// Returns a greeting.
async fn hello_world() -> &'static str {
    "Hello, Shuttle!"
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

// Local tests are omitted since shuttle depends on a specific runtime.
// The code itself is type-checked during compilation.

// Workaround for unused code warnings when not compiled by shuttle:
/// Calls the handler, so that it is not reported as unused.
#[allow(dead_code)]
fn dummy() {
    std::mem::drop(hello_world());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires Shuttle runtime"]
    fn test_main() {
        main();
    }
}
