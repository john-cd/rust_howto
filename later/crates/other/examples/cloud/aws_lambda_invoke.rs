// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "needs AWS credentials (Lambda); the tests use mocks"
)]
// ANCHOR: example
//! Invoke an AWS Lambda function synchronously and asynchronously.
//!
//! AWS Lambda runs your function on demand without servers to manage. A
//! synchronous invocation waits for the result; an asynchronous one queues the
//! event and returns at once, and Lambda retries on failure. Running this needs
//! AWS credentials.

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use aws_sdk_lambda::Client;
use aws_sdk_lambda::primitives::Blob;
use aws_sdk_lambda::types::InvocationType;
use serde::Deserialize;
use serde::Serialize;

/// The request understood by the function (see the Lambda example).
#[derive(Serialize)]
pub struct Request {
    /// The name to greet.
    pub name: String,
}

/// The response returned by the function.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Response {
    /// The greeting.
    pub message: String,
}

/// Calls the function and waits for its JSON response.
pub async fn invoke_sync(
    client: &Client,
    function_name: &str,
    request: &Request,
) -> Result<Response> {
    let output = client
        .invoke()
        .function_name(function_name)
        .invocation_type(InvocationType::RequestResponse)
        .payload(Blob::new(serde_json::to_vec(request)?))
        .send()
        .await
        .context("Lambda Invoke failed")?;
    // A function that fails still returns HTTP 200: check `function_error`.
    if let Some(error) = output.function_error() {
        bail!("function returned an error: {error}");
    }
    let payload = output.payload().context("empty payload")?;
    parse_response(payload.as_ref())
}

/// Queues an invocation and returns immediately (HTTP 202).
pub async fn invoke_async(client: &Client, function_name: &str, request: &Request) -> Result<i32> {
    let output = client
        .invoke()
        .function_name(function_name)
        .invocation_type(InvocationType::Event)
        .payload(Blob::new(serde_json::to_vec(request)?))
        .send()
        .await
        .context("Lambda Invoke failed")?;
    Ok(output.status_code())
}

/// Parses the JSON bytes returned by the function.
pub fn parse_response(bytes: &[u8]) -> Result<Response> {
    serde_json::from_slice(bytes).context("unexpected function response")
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the function's JSON response is parsed.
    #[test]
    fn response_is_parsed() -> Result<()> {
        let response = parse_response(br#"{"message":"Hello, Ada!"}"#)?;
        assert_eq!(
            response,
            Response {
                message: "Hello, Ada!".to_owned()
            }
        );
        Ok(())
    }

    /// Verifies that malformed payloads are an error.
    #[test]
    fn malformed_response_is_an_error() {
        assert!(parse_response(b"not json").is_err());
    }

    /// Builds a client that targets LocalStack (`AWS_ENDPOINT_URL`, default
    /// `http://localhost:4566`) with dummy credentials.
    fn localstack_client() -> Client {
        let endpoint = std::env::var("AWS_ENDPOINT_URL")
            .unwrap_or_else(|_| "http://localhost:4566".to_owned());
        let config = aws_sdk_lambda::Config::builder()
            .behavior_version(aws_sdk_lambda::config::BehaviorVersion::latest())
            .region(aws_sdk_lambda::config::Region::new("us-east-1"))
            .endpoint_url(endpoint)
            .credentials_provider(aws_sdk_lambda::config::Credentials::new(
                "test", "test", None, None, "static",
            ))
            .build();
        Client::from_conf(config)
    }

    /// Invokes a function that already exists in LocalStack. Name it with
    /// `LAMBDA_FUNCTION`; it must return `{"message":"Hello, Ada!"}` for the
    /// request `{"name":"Ada"}`. Start LocalStack with the Docker socket
    /// mounted so it can run the function.
    #[tokio::test]
    #[ignore = "requires LocalStack (Lambda) on localhost:4566 and LAMBDA_FUNCTION"]
    async fn function_is_invoked() -> Result<()> {
        let client = localstack_client();
        let function = std::env::var("LAMBDA_FUNCTION")?;
        let request = Request {
            name: "Ada".to_owned(),
        };
        let response = invoke_sync(&client, &function, &request).await?;
        assert_eq!(response.message, "Hello, Ada!");
        assert_eq!(invoke_async(&client, &function, &request).await?, 202);
        Ok(())
    }
}
