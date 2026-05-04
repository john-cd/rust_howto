#![allow(dead_code)]
// ANCHOR: example
//! This is a basic example of an AWS Lambda function written in Rust.
//! It demonstrates how to use the `lambda_runtime` crate to create a simple
//! function that takes a name as input and returns a greeting message.
//!
//! Prerequisites:
//!
//! - Install the AWS CLI.
//! - Install the AWS SAM CLI.
//!
//! ```sh
//! cargo build --release --target x86_64-unknown-linux-musl
//! sam build
//! sam deploy --guided
//! ```

use lambda_runtime::Error;
use lambda_runtime::LambdaEvent;
use lambda_runtime::service_fn;
use serde::Deserialize;
use serde::Serialize;
use tracing_subscriber;

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

async fn function_handler(
    event: LambdaEvent<Request>,
) -> Result<Response, Error> {
    let (payload, _context) = event.into_parts();

    Ok(Response {
        message: format!("Hello, {}!", payload.name),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    let func = service_fn(function_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main().map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires Lambda runtime"]
    fn test_main() {
        main().unwrap();
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/878)
