// ANCHOR: example
use std::collections::HashMap;

use anyhow::Result;
use reqwest::header;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}

fn main() -> Result<()> {
    // Parse an absolute URL from a string and add params to its query string
    let url = url::Url::parse_with_params(
        "http://httpbin.org/headers",
        &[("lang", "rust"), ("browser", "servo")],
    )?;

    // Define default headers for all requests
    let mut default_headers = header::HeaderMap::new();
    default_headers
        .insert("X-MY-HEADER", header::HeaderValue::from_static("value"));

    let client = reqwest::blocking::Client::builder()
        .user_agent("Rust-test")
        .default_headers(default_headers)
        .build()?;

    // Headers for this request only
    let mut headers = header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/png"),
    );

    let response = client.get(url)
        .headers(headers)
        .bearer_auth("DEadBEEfc001cAFeEDEcafBAd") // Enable HTTP bearer authentication.
        .send()?;

    assert_eq!(
        response.url().as_str(),
        "http://httpbin.org/headers?lang=rust&browser=servo"
    );

    let out: HeadersEcho = response.json()?;
    assert_eq!(
        out.headers["Authorization"],
        "Bearer DEadBEEfc001cAFeEDEcafBAd"
    );
    assert_eq!(out.headers["User-Agent"], "Rust-test");
    assert_eq!(out.headers["X-My-Header"], "value");
    println!("{:?}", out);
    Ok(())
}
// ANCHOR_END: example

// Requires network access
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
