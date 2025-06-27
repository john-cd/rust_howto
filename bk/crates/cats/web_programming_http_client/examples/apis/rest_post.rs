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
    let gh_user = env::var("GH_USER")?;
    let gh_pass = env::var("GH_PASS")?;

    // Example POST to the GitHub gists API:
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
    let response = Client::new()
        .post(request_url)
        .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
        .json(&gist_body)
        .send()
        .await?;

    let gist: Gist = response.json().await?;
    println!("Created {gist:?}");

    let request_url = format!("{}/{}", request_url, gist.id);
    let response = Client::new()
        .delete(&request_url)
        .basic_auth(gh_user, Some(gh_pass))
        .send()
        .await?;

    println!(
        "Gist {} deleted! Status code: {}",
        gist.id,
        response.status()
    );
    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() {
    println!("{:?}", main());
}
// [review; rewrite so that a username and password are not required? NOW](https://github.com/john-cd/rust_howto/issues/178)
