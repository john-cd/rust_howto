#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to retrieve the Content-Type header from a
//! response and parse it as a MIME type.

use std::str::FromStr;

use anyhow::Result;
use http::HeaderMap;
use mime::Mime;
use reqwest::Response;
use reqwest::header::CONTENT_TYPE;

/// Retrieves the Content-Type header from a response and parses it as a MIME
/// type.
#[tokio::main]
async fn main() -> Result<()> {
    let response: Response =
        reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png")
            .await?;
    let headers: &HeaderMap = response.headers();

    match headers.get(CONTENT_TYPE) {
        None => {
            println!("The response does not contain a Content-Type header.");
        }
        Some(content_type) => {
            let content_type = Mime::from_str(content_type.to_str()?)?;
            let media_type =
                match (content_type.type_(), content_type.subtype()) {
                    (mime::TEXT, mime::HTML) => "a HTML document",
                    (mime::TEXT, _) => "a text document",
                    (mime::IMAGE, mime::PNG) => "a PNG image",
                    (mime::IMAGE, _) => "an image",
                    _ => "neither text nor image",
                };

            println!("The reponse contains {media_type}.");
        }
    };

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
