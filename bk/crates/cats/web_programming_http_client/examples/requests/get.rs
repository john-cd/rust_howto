#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates a simple HTTP GET request using the `reqwest`
//! crate. It fetches data from http://<httpbin.org/get> and prints the status, headers, and body.

use anyhow::Result;

/// Sends a GET request to <http://httpbin.org/get> and prints the response.
fn main() -> Result<()> {
    let res = reqwest::blocking::get("https://httpbin.org/get")?
        .error_for_status()?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", res.text()?);

    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Marks this live-service demonstration as an opt-in network test.
    #[test]
    #[ignore = "needs network"]
    fn require_network() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
