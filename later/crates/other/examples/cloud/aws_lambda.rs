// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This is a basic example of an AWS Lambda function written in Rust.
//! It demonstrates how to use the `lambda_runtime` crate to create a simple
//! function that takes a name as input and returns a greeting message.
//!
//! Install Cargo Lambda to build, run locally, and deploy this function.
//! Configure AWS credentials and a deployment role before deploying.
//!
//! ```sh
//! cargo lambda build --release
//! cargo lambda watch
//! cargo lambda deploy
//! ```

use lambda_runtime::Error;
use lambda_runtime::LambdaEvent;
use lambda_runtime::service_fn;
use serde::Deserialize;
use serde::Serialize;

/// The request that the function receives.
#[derive(Deserialize)]
struct Request {
    /// The name of the person to greet.
    name: String,
}

/// The response that the function returns.
#[derive(PartialEq, Serialize)]
struct Response {
    /// The greeting.
    message: String,
}

/// Handles a request by returning a greeting.
async fn function_handler(
    event: LambdaEvent<Request>,
) -> Result<Response, Error> {
    let (payload, _context) = event.into_parts();

    Ok(Response {
        message: format!("Hello, {}!", payload.name),
    })
}

/// Starts the Lambda runtime with the function handler.
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    let func = service_fn(function_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
// ANCHOR_END: example

/// Starts the Lambda runtime for this handler.
pub fn run() -> anyhow::Result<()> {
    main().map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the handler response without starting the AWS Lambda runtime.
    #[tokio::test]
    async fn handler_returns_a_greeting_for_the_request() -> Result<(), Error> {
        let request = Request {
            name: "Ada".to_owned(),
        };
        let event =
            LambdaEvent::new(request, lambda_runtime::Context::default());

        let response = function_handler(event).await?;

        assert_eq!(response.message, "Hello, Ada!");
        Ok(())
    }
}
