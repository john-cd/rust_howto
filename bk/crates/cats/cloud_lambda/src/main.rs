//! A small Lambda handler that can also be tested without AWS.

use lambda_runtime::Error;
use lambda_runtime::LambdaEvent;
use lambda_runtime::service_fn;
use serde::Deserialize;
use serde::Serialize;

// ANCHOR: example
#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Debug, PartialEq, Serialize)]
struct Response {
    message: String,
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
