// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use shuttle_microservice::prelude::*;

// We use the shuttle_microservice crate to create a simple web application.
// We define an async init function with the shuttle_service attribute to
// initialize the application. Inside the `init` function, we create an
// instance of `App` and define a single route (/) that returns "Hello,
// Shuttle!" when accessed. We return an instance of `ShuttleService` with the
// initialized app.

// #[shuttle_service]
// async fn init() -> Result<ShuttleService, ShuttleError> {
//     let app = shuttle_microservice::app::App::new()
//         .route("/", get(|| async { "Hello, Shuttle!" }));

//     Ok(ShuttleService::new(app))
// }

// #[test]
// fn test() {
//     main();
// }
// // [add shuttle.rs example P1](https://github.com/john-cd/rust_howto/issues/49)
