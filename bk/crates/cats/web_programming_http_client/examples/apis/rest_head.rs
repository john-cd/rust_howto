// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `HEAD` method to check if a
//! resource exists.
//!
//! `HEAD` works like `GET` but the server returns only the status line and
//! headers, not the body. It is a cheap way to check that a URL exists, or to
//! read its size and type, before downloading it.

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
    let exists = user_exists(&client, &request_url).await?;

    if exists {
        println!("{user} is a user!");
    } else {
        println!("{user} is not a user!");
    }

    Ok(())
}

/// Sends a `HEAD` request and reports whether the endpoint returned success.
async fn user_exists(client: &reqwest::Client, request_url: &str) -> reqwest::Result<bool> {
    Ok(client.head(request_url).send().await?.status().is_success())
}
// ANCHOR_END: example

/// Sends a `HEAD` request to check whether a user exists.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> reqwest::Result<()> {
    main()
}

/// if a
#[cfg(test)]
mod tests {
    use super::*;

    /// Requires a live network service to run the example.
    #[test]
    #[ignore = "needs network"]
    fn require_network() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// A successful HEAD response means the resource exists.
    #[tokio::test]
    async fn successful_head_response_means_user_exists() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("HEAD", "/users/ferris-the-crab")
            .with_status(200)
            .create_async()
            .await;
        let client = reqwest::Client::new();

        let exists =
            user_exists(&client, &format!("{}/users/ferris-the-crab", server.url())).await?;

        mock.assert_async().await;
        assert!(exists);
        Ok(())
    }

    /// A not-found HEAD response means the resource does not exist.
    #[tokio::test]
    async fn not_found_head_response_means_user_does_not_exist() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("HEAD", "/users/missing")
            .with_status(404)
            .create_async()
            .await;
        let client = reqwest::Client::new();

        let exists = user_exists(&client, &format!("{}/users/missing", server.url())).await?;

        mock.assert_async().await;
        assert!(!exists);
        Ok(())
    }
}
