// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use lambda_runtime::Context;
// use lambda_runtime::Error;
// // use lambda_runtime::LambdaEvent;
// use lambda_runtime::service_fn;
// use serde::Deserialize;
// use serde::Serialize;
// // use serde_json::json;
// // use tracing::{error, info};
// use tracing_subscriber;

// // Install AWS CLI
// // Install AWS SAM CLI
// // cargo build --release --target x86_64-unknown-linux-musl
// // sam build
// // sam deploy --guided

// #[derive(Deserialize)]
// struct Request {
//     name: String,
// }

// #[derive(Serialize)]
// struct Response {
//     message: String,
// }

// async fn function_handler(
//     event: Request,
//     _: Context,
// ) -> Result<Response, Error> {
//     Ok(Response {
//         message: format!("Hello, {}!", event.name),
//     })
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     // Initialize tracing subscriber
//     tracing_subscriber::fmt::init();

//     let func = service_fn(function_handler);
//     lambda_runtime::run(func).await?;
//     Ok(())
// }

// #[test]
// fn require_network() {
//     main();
// }
// // [WIP finish](https://github.com/john-cd/rust_howto/issues/878)
