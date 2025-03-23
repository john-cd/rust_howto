// ANCHOR: example
use reqwest::Error;
use reqwest::blocking::Client;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user_name = "testuser".to_string();
    let password: Option<String> = None;

    let response = client
        .get("https://httpbin.org/")
        .basic_auth(user_name, password)
        .send();

    println!("{:?}", response);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
