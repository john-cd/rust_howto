// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to make a GET request to a REST API and
//! deserialize the JSON response.

use reqwest::Client;
use reqwest::Error;
use serde::Deserialize;

/// Represents a GitHub user with their login and ID.
#[derive(Deserialize, Debug, Eq, PartialEq)]
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
    let client = Client::builder().user_agent("Rust-test").build()?;

    // Send a GET request, reject HTTP error statuses, and deserialize JSON.
    let users = get_users(&client, &request_url).await?;
    println!("{users:?}");

    Ok(())
}

/// Fetches users from the requested REST endpoint and rejects unsuccessful
/// responses.
async fn get_users(
    client: &Client,
    request_url: &str,
) -> Result<Vec<User>, Error> {
    client
        .get(request_url)
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<User>>()
        .await
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the example sends its request and deserializes the local
    /// response.
    #[tokio::test]
    async fn returns_deserialized_users() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/repos/john-cd/rust_howto/stargazers")
            .match_header("user-agent", "Rust-test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"login":"octocat","id":42}]"#)
            .create_async()
            .await;
        let client = Client::builder().user_agent("Rust-test").build()?;
        let request_url =
            format!("{}/repos/john-cd/rust_howto/stargazers", server.url());

        let users = get_users(&client, &request_url).await?;

        mock.assert_async().await;
        assert_eq!(
            users,
            [User {
                login: "octocat".to_owned(),
                id: 42
            }]
        );
        Ok(())
    }

    /// Queries the live GitHub API.
    #[test]
    #[ignore = "needs network"]
    fn main_fetches_stargazers() -> Result<(), Error> {
        main()
    }

    /// An HTTP error status is reported as an error.
    #[tokio::test]
    async fn error_status_is_an_error() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/x")
            .with_status(500)
            .create_async()
            .await;
        let client = Client::builder().user_agent("Rust-test").build()?;
        let result = get_users(&client, &format!("{}/x", server.url())).await;
        assert!(result.is_err());
        Ok(())
    }
}
