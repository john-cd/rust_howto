// #![allow(dead_code)]
// #![allow(unused_variables)]
// // // ANCHOR: example
// // // COMING SOON
// // // ANCHOR_END: example
// //! This module demonstrates basic usage of axum.
// //!
// //! `axum` is a web application framework designed to work with `tokio`
// //! and `hyper`. It doesn't have its own middleware system but instead uses
// //! `tower::Service`. This means `axum` gets timeouts, tracing, compression,
// //! authorization, and more, for free.
// /// See also:
// /// - <https://docs.rs/axum/latest/axum/index.html>
// /// - <https://docs.rs/axum/0.8.1/axum/extract/struct.State.html>
// /// - <https://docs.rs/axum/0.8.1/axum/extract/index.html>
// use std::collections::HashMap;
// use std::sync::Arc;

// use axum::Router;
// use axum::extract::Json;
// use axum::extract::Path;
// use axum::extract::Query;
// use axum::extract::State;
// use axum::http::StatusCode;
// use axum::http::Uri;
// use axum::routing::get;

// // Application state.
// #[derive(Clone)]
// struct AppState {
//     request_count: u64,
// }

// // An Axum "handler" is an async function that accepts zero or more
// // "extractors" as arguments and returns something that can be converted into
// // a response.
// async fn root() -> &'static str {
//     "Hello, world!"
//     // Anything that implements `IntoResponse` can be returned from handlers.
//     // `&'static str` becomes a `200 OK` with `content-type: text/plain;
//     // charset=utf-8`
// }

// // The `Path` extractor gives you the path parameters (here `id` in
// // `/users/{id}`) and deserializes them.
// async fn get_user(Path(id): Path<u64>) {
//     // FIXME
// }

// async fn post_user(Path(id): Path<u64>) {
//     // FIXME
// }

// // The wildcard e.g. `/assets/{*path}` is passed as a `String`.
// async fn serve_asset(Path(path): Path<String>) -> String {
//     path
// }

// // `Query` gives you the query parameters and deserializes them.
// async fn query(Query(_params): Query<HashMap<String, String>>) {
//     // FIXME
// }

// // Buffer the request body and deserialize it as JSON into a
// // `serde_json::Value`. `Json` supports any type that implements
// // `serde::Deserialize`.
// async fn json(Json(_payload): Json<serde_json::Value>) {
//     // FIXME
// }

// // Access the application state via the `State` extractor.
// async fn state(State(state): State<Arc<AppState>>) {
//     // FIXME state.request_count
// }

// // Fallbacks apply to routes that aren't matched by anything in the router.
// async fn fallback(uri: Uri) -> (StatusCode, String) {
//     (StatusCode::NOT_FOUND, format!("No route for {uri}"))
// }

// // `tokio` macros and rt-multi-thread features should be enabled.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //     // FIXME
    //     //  tracing_subscriber::registry()
    //     //     .with(
    //     //         tracing_subscriber::EnvFilter::try_from_default_env()
    //     //             .unwrap_or_else(|_| format!("{}=debug",
    //     // env!("CARGO_CRATE_NAME")).into()),     )
    //     //     .with(tracing_subscriber::fmt::layer())
    //     //     .init();

    //     let shared_state = Arc::new(AppState { request_count: 0 });

    //     // Build our application that routes requests to handlers
    //     // The `path` is a string of path segments separated by /.
    //     // Each segment can be either static, a capture, or a wildcard,
    //     // e.g. /users/123 or /users/{id} or /assets/{*path}.
    //     let router = Router::new()
    //         .route("/", get(root)) // Could be `get`, `post`, or `delete`...
    //         .route("/users/{id}", get(get_user).post(post_user))
    //         // FIXME .route("/assets/{*path}", get(serve_asset)) // Note:
    // doesn't // match empty segments i.e. /assets or /assets/.
    // .fallback(fallback)
    //         .with_state(shared_state);

    //     // Run it
    //     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    //     // tracing::debug!("Listening on {}", listener.local_addr()?);
    //     axum::serve(listener, router).await?;
    Ok(())
}

// #[test]
// fn require_network() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [review time limit](https://github.com/john-cd/rust_howto/issues/865)
// // <https://docs.rs/axum/latest/axum/index.html>
// // <https://docs.rs/axum/0.8.1/axum/extract/struct.State.html>
// // <https://docs.rs/axum/0.8.1/axum/extract/index.html>
