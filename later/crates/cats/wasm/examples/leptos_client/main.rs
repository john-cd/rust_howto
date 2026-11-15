//! Client-side Leptos building blocks: routing, HTTP calls, local storage and
//! a gRPC-Web client.
//!
//! TODO Reviewed ??-??-2026.

/// Call an HTTP API from WebAssembly with [`gloo_net`].
mod gloo_fetch;

/// Persist typed settings in the browser's local storage with [`gloo_storage`].
mod gloo_storage;

/// A gRPC client that uses gRPC-Web in the browser and HTTP/2 elsewhere.
mod grpc_web_client;

/// Use [`icondata`] icon data as inline SVG in a Leptos view.
mod icons;

/// Nested routes, a route parameter and a not-found route with [`leptos_router`].
mod leptos_router;

/// Runs the example.
fn main() {
    println!("Run `cargo test --example leptos_client`.");
}
