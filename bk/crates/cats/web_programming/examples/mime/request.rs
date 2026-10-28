// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to retrieve the Content-Type header from a
//! response and parse it as a MIME type.

use std::str::FromStr as _;

use anyhow::Result;
use http::HeaderMap;
use mime::Mime;
use reqwest::Response;
use reqwest::header::CONTENT_TYPE;

/// Describes the media type in a response header, if one is present.
///
/// # Errors
///
/// Returns an error if the header is not valid text or does not contain a
/// valid MIME type.
fn describe_content_type(headers: &HeaderMap) -> Result<Option<&'static str>> {
    let Some(value) = headers.get(CONTENT_TYPE) else {
        return Ok(None);
    };
    let content_type = Mime::from_str(value.to_str()?)?;
    let description = match (content_type.type_(), content_type.subtype()) {
        (mime::TEXT, mime::HTML) => "an HTML document",
        (mime::TEXT, _) => "a text document",
        (mime::IMAGE, mime::PNG) => "a PNG image",
        (mime::IMAGE, _) => "an image",
        _ => "neither text nor image",
    };

    Ok(Some(description))
}

/// Retrieves the Content-Type header from a successful HTTP response.
#[tokio::main]
async fn main() -> Result<()> {
    let response: Response =
        reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png")
            .await?
            .error_for_status()?;
    let headers: &HeaderMap = response.headers();

    match describe_content_type(headers)? {
        None => {
            println!("The response does not contain a Content-Type header.");
        }
        Some(media_type) => {
            println!("The response contains {media_type}.");
        }
    }

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use http::HeaderValue;

    use super::*;

    /// Verifies a valid response media type is classified correctly.
    #[test]
    fn describes_content_type_header() -> anyhow::Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));

        assert_eq!(describe_content_type(&headers)?, Some("a PNG image"));
        Ok(())
    }

    /// Verifies responses without a media type are reported as absent.
    #[test]
    fn handles_missing_content_type_header() -> anyhow::Result<()> {
        assert_eq!(describe_content_type(&HeaderMap::new())?, None);
        Ok(())
    }

    /// Verifies malformed MIME values are returned as errors.
    #[test]
    fn rejects_invalid_content_type_header() {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("not a mime"));

        assert!(describe_content_type(&headers).is_err());
    }

    /// Marks the live server request as an opt-in network test.
    #[test]
    #[ignore = "needs network"]
    fn require_network() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
