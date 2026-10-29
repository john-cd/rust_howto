// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `HEAD` method to check if a
//! resource exists.

use std::time::Duration;

use reqwest::ClientBuilder;

/// Sends a `HEAD` request to check whether a user exists.
#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let user = "ferris-the-crab";
    let request_url = format!("https://api.github.com/users/{user}");
    println!("{request_url}");

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send().await?;

    if response.status().is_success() {
        println!("{user} is a user!");
    } else {
        println!("{user} is not a user!");
    }

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> reqwest::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    // output-only: the example only calls the live GitHub API (ignored
    // offline); see the other tests for its logic.
    use super::*;

    #[test]
    #[ignore = "needs network"]
    fn require_network() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
