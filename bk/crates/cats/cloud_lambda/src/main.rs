//! A small Lambda handler that can also be tested without AWS.

use lambda_runtime::Error;
use lambda_runtime::LambdaEvent;
use lambda_runtime::service_fn;
use serde::Deserialize;
use serde::Serialize;

// ANCHOR: example
/// JSON event payload containing the name to greet.
#[derive(Deserialize)]
struct Request {
    /// Recipient name read from the Lambda event payload.
    name: String,
}

/// JSON response returned to the Lambda caller.
#[derive(Debug, PartialEq, Serialize)]
struct Response {
    /// Greeting constructed from the event's recipient name.
    message: String,
}

/// Converts the event payload into a greeting without using AWS-specific context.
async fn handle(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let (request, _) = event.into_parts();
    Ok(Response {
        message: format!("Hello, {}!", request.name),
    })
}
// ANCHOR_END: example

/// Starts the Lambda polling runtime and delegates each event to the greeting handler.
#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handle)).await
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that event becomes greeting.
    #[tokio::test]
    async fn event_becomes_greeting() -> Result<(), Error> {
        let event = LambdaEvent::new(
            Request {
                name: "Ada".to_owned(),
            },
            lambda_runtime::Context::default(),
        );
        assert_eq!(
            handle(event).await?,
            Response {
                message: "Hello, Ada!".to_owned()
            }
        );
        Ok(())
    }
}
