#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Finding the MIME type of a file from its name.

use http::header::CONTENT_TYPE;
use http::header::HeaderMap;
use http::header::HeaderValue;
use http::header::InvalidHeaderValue;
use mime::Mime;

/// Guesses the MIME type from a file name's extension.
fn find_mimetype(filename: &str) -> Mime {
    mime_guess::from_path(filename).first_or_octet_stream()
}

/// Inserts a guessed MIME type into an HTTP `Content-Type` header.
fn set_content_type(
    filename: &str,
    headers: &mut HeaderMap,
) -> Result<(), InvalidHeaderValue> {
    let mime = find_mimetype(filename);
    headers.insert(CONTENT_TYPE, HeaderValue::try_from(mime.as_ref())?);
    Ok(())
}

/// Prints the MIME type of several file names.
fn main() -> Result<(), InvalidHeaderValue> {
    let filenames = ["foobar.jpg", "foo.bar", "foobar.png"];
    for file in filenames {
        let mut headers = HeaderMap::new();
        set_content_type(file, &mut headers)?;
        println!("Content-Type for {file}: {}", headers[CONTENT_TYPE]);
    }
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), InvalidHeaderValue> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies common file extensions map to their registered MIME types.
    #[test]
    fn guesses_common_filename_types() {
        assert_eq!(find_mimetype("assets/photo.png"), mime::IMAGE_PNG);
        assert_eq!(find_mimetype("reports/data.json"), mime::APPLICATION_JSON);
        assert_eq!(find_mimetype("PHOTO.JPG"), mime::IMAGE_JPEG);
    }

    /// Verifies unknown or absent extensions use the binary fallback.
    #[test]
    fn uses_octet_stream_for_unknown_extensions() {
        assert_eq!(find_mimetype("README"), mime::APPLICATION_OCTET_STREAM);
        assert_eq!(
            find_mimetype("data.unknown-extension"),
            mime::APPLICATION_OCTET_STREAM
        );
    }

    /// Verifies the guessed MIME type can be used as an HTTP response header.
    #[test]
    fn sets_content_type_header() -> Result<(), InvalidHeaderValue> {
        let mut headers = HeaderMap::new();
        set_content_type("report.PDF", &mut headers)?;

        assert_eq!(headers[CONTENT_TYPE], "application/pdf");
        Ok(())
    }
}
