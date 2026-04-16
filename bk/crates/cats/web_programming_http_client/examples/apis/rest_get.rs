#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to make a GET request to a REST API and
//! deserialize the JSON response.

use reqwest::Error;
use serde::Deserialize;

/// Represents a GitHub user with their login and ID.
#[derive(Deserialize, Debug)]
struct User {
    /// The user's login name on GitHub.
    login: String,
    /// The user's unique ID on GitHub.
    id: u32,
}

/// The main function that performs the HTTP GET request and processes the
/// response.
#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "john-cd",
        repo = "rust_howto"
    );
    println!("{request_url}");

    // Create a new HTTP client with a custom User-Agent header.
    let client = reqwest::Client::builder().user_agent("Rust-test").build()?;

    // Send a GET request to the specified URL and await the response.
    let response = client.get(&request_url).send().await?;

    // Deserialize the JSON response into a vector of `User` structs.
    if let Ok(users) = response.json::<Vec<User>>().await {
        println!("{users:?}");
    } else {
        println!("Could not deserialize the response as JSON.");
    }

    Ok(())
}
// ANCHOR_END: example
#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
