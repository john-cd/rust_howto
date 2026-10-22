#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use basic authentication with `reqwest`.
//!
//! TODO Reviewed ??-??-2026

use reqwest::Error;
use reqwest::blocking::Client;

/// Sends a request with basic authentication credentials and prints the
/// response status.
fn main() -> Result<(), Error> {
    // Create a new `reqwest` client:
    let client = Client::new();

    let user_name = "testuser".to_owned();
    let password: Option<String> = None;

    let response = client
        .get("https://httpbin.org/")
        .basic_auth(user_name, password)
        .send();

    println!("{response:?}");

    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn require_network() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
