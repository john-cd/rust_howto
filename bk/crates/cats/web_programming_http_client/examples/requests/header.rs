#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates how to set headers in HTTP requests using the `reqwest` crate.

use std::collections::HashMap;

use anyhow::Result;
use reqwest::header;
use serde::Deserialize;

/// Represents the structure of the JSON response from `httpbin.org/headers`.
#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    /// A map of header names to their values.
    pub headers: HashMap<String, String>,
}

/// Sends a request with default and per-request headers and parses its JSON.
///
/// # Errors
///
/// Returns an error if the request or JSON response is invalid or the server
/// returns an unsuccessful status.
fn request_headers(url: url::Url) -> Result<HeadersEcho> {
    // Define default headers for all requests:
    let mut default_headers = header::HeaderMap::new();
    default_headers
        .insert("X-MY-HEADER", header::HeaderValue::from_static("value"));

    let client = reqwest::blocking::Client::builder()
        .user_agent("Rust-test")
        .default_headers(default_headers)
        .build()?;

    // Headers for this request only:
    let mut headers = header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/png"),
    );

    let response = client
        .get(url)
        .headers(headers)
        .bearer_auth("DEadBEEfc001cAFeEDEcafBAd") // Enable HTTP bearer authentication.
        .send()?
        .error_for_status()?;

    Ok(response.json()?)
}

/// Demonstrates how to set headers in HTTP requests using the `reqwest` crate.
fn main() -> Result<()> {
    let url = url::Url::parse_with_params(
        "https://httpbin.org/headers",
        &[("lang", "rust"), ("browser", "servo")],
    )?;
    let out = request_headers(url)?;
    assert_eq!(
        out.headers["Authorization"],
        "Bearer DEadBEEfc001cAFeEDEcafBAd"
    );
    assert_eq!(out.headers["User-Agent"], "Rust-test");
    assert_eq!(out.headers["X-My-Header"], "value");
    println!("{out:?}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::io::Read as _;
    use std::io::Write as _;
    use std::net::TcpListener;
    use std::thread;

    use super::*;

    /// Marks the live HTTP echo example as an opt-in network test.
    #[test]
    #[ignore = "needs network"]
    fn require_network() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Verifies request headers and JSON parsing against a local HTTP server.
    #[test]
    fn sends_custom_headers_and_parses_json() -> anyhow::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let address = listener.local_addr()?;
        let server = thread::spawn(move || -> std::io::Result<String> {
            let (mut stream, _) = listener.accept()?;
            let mut request = Vec::new();
            let mut buffer = [0; 1024];
            loop {
                let count = stream.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                request.extend_from_slice(&buffer[..count]);
                if request.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }
            let body = b"{\"headers\":{\"Authorization\":\"Bearer DEadBEEfc001cAFeEDEcafBAd\",\"User-Agent\":\"Rust-test\",\"X-My-Header\":\"value\"}}";
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            )?;
            stream.write_all(body)?;
            String::from_utf8(request).map_err(std::io::Error::other)
        });
        let url = url::Url::parse_with_params(
            &format!("http://{address}/headers"),
            &[("lang", "rust"), ("browser", "servo")],
        )?;

        let result = request_headers(url)?;
        let request = server
            .join()
            .map_err(|_| anyhow::anyhow!("local HTTP server panicked"))??;

        assert!(
            request.starts_with(
                "GET /headers?lang=rust&browser=servo HTTP/1.1\r\n"
            )
        );
        let request = request.to_ascii_lowercase();
        assert!(
            request.contains("authorization: bearer deadbeefc001cafeedecafbad")
        );
        assert!(request.contains("x-my-header: value"));
        assert_eq!(result.headers["User-Agent"], "Rust-test");
        Ok(())
    }
}
