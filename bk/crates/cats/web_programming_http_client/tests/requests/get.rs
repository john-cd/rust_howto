// ANCHOR: example
//! This example demonstrates a simple HTTP GET request using the `reqwest`
//! crate. It fetches data from http://httpbin.org/get and prints the status, headers, and body.

use std::io::Read;

use anyhow::Result;

/// Sends a GET request to http://httpbin.org/get and prints the response.
fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
