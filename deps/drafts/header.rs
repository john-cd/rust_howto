

use std::collections::HashMap;

use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use tower_http::auth::require_authorization::Bearer;
use url::Url;

header! { (XPoweredBy, "X-Powered-By") => [String] }

#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}

fn main() -> Result<()> {
    let url = Url::parse_with_params(
        "http://httpbin.org/headers",
        &[("lang", "rust"), ("browser", "servo")],
    )?;

    let mut response = Client::new()
        .get(url)
        .header(UserAgent::new("Rust-test"))
        .header(Authorization(Bearer {
            token: "DEadBEEfc001cAFeEDEcafBAd".to_owned(),
        }))
        .header(XPoweredBy("Guybrush Threepwood".to_owned()))
        .send()?;

    let out: HeadersEcho = response.json()?;
    assert_eq!(
        out.headers["Authorization"],
        "Bearer DEadBEEfc001cAFeEDEcafBAd"
    );
    assert_eq!(out.headers["User-Agent"], "Rust-test");
    assert_eq!(out.headers["X-Powered-By"], "Guybrush Threepwood");
    assert_eq!(
        response.url().as_str(),
        "http://httpbin.org/headers?lang=rust&browser=servo"
    );

    println!("{:?}", out);
    Ok(())
}
