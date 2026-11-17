//! A small Lambda handler that can also be tested without AWS.

use lambda_runtime::Error;
use lambda_runtime::LambdaEvent;
use lambda_runtime::service_fn;
use serde::Deserialize;
use serde::Serialize;

// ANCHOR: example
#[derive(Deserialize)]
struct Request {
    /// Name stored for this value.
    name: String,
}

/// Data for response operations, including message.
#[derive(Debug, PartialEq, Serialize)]
struct Response {
    /// Message stored for this value.
    message: String,
}

/// Handles the handle behavior for this component.
async fn handle(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let (request, _) = event.into_parts();
    Ok(Response {
        message: format!("Hello, {}!", request.name),
    })
}
// ANCHOR_END: example

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
