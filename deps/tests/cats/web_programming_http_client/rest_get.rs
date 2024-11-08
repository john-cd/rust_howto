#![allow(dead_code)]
// ANCHOR: example
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "john-cd",
        repo = "rust_howto"
    );
    println!("{}", request_url);

    let client = reqwest::Client::builder().user_agent("Rust-test").build()?;

    let response = client.get(&request_url).send().await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
// ANCHOR_END: example

// Requires network access
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
