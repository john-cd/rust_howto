// TODO Reviewed ??-??-2026
#![allow(dead_code)]
// ANCHOR: example
//! Example of using the `reqwest` crate to interact with a REST API.

use std::collections::HashMap;
use std::env;

use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

/// Represents the structure of a GitHub Gist to be created.
#[derive(Deserialize, Serialize, Debug)]
struct Post<'a> {
    description: &'a str,
    public: bool,
    files: HashMap<&'a str, Content<'a>>,
}

/// Represents the content of a file within a Gist.
#[derive(Deserialize, Serialize, Debug)]
struct Content<'a> {
    content: &'a str,
}

/// Represents the structure of a Gist as returned by the GitHub API.
#[derive(Deserialize, Debug)]
struct Gist {
    id: String,
    html_url: String,
}

/// Main function to demonstrate creating and deleting a Gist on GitHub.
#[tokio::main]
async fn main() -> Result<()> {
    let gh_token = env::var("GH_TOKEN")?;

    // Create a short-lived gist with a fine-grained token that has the
    // `gists` permission. Basic authentication with a GitHub password is not
    // supported by the REST API.
    let gist_body = Post {
        description: "the description for this gist",
        public: true,
        files: {
            let mut h = HashMap::new();
            h.insert(
                "main.rs",
                Content {
                    content: r#"
fn main() { println!("hello world!"); }
"#,
                },
            );
            h
        },
    };

    let request_url = "https://api.github.com/gists";
    let client = Client::new();
    let response = client
        .post(request_url)
        .bearer_auth(&gh_token)
        .json(&gist_body)
        .send()
        .await?
        .error_for_status()?;

    let gist: Gist = response.json().await?;
    println!("Created {gist:?}");

    let request_url = format!("{request_url}/{}", gist.id);
    let response = client
        .delete(&request_url)
        .bearer_auth(gh_token)
        .send()
        .await?
        .error_for_status()?;

    println!(
        "Gist {} deleted! Status code: {}",
        gist.id,
        response.status()
    );
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "requires GH_TOKEN and access to the GitHub API"]
    fn require_network() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(main()).unwrap();
    }
}
// [review; rewrite so that a username and password are not required?](https://github.com/john-cd/rust_howto/issues/178)
