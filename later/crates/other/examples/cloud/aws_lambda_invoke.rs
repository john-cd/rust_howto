// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "needs AWS credentials (Lambda); the tests use mocks"
)]
// ANCHOR: example
//! Invoke an AWS Lambda function synchronously and asynchronously.

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
pub async fn invoke_async(
    client: &Client,
    function_name: &str,
    request: &Request,
) -> Result<i32> {
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
}
