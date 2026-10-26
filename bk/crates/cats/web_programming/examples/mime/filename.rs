#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Finding the MIME type of a file from its name.

use mime::Mime;

/// Guesses the MIME type from a file name's extension.
fn find_mimetype(filename: &str) -> Mime {
    mime_guess::from_path(filename).first_or_octet_stream()
}

/// Prints the MIME type of several file names.
fn main() {
    let filenames = ["foobar.jpg", "foo.bar", "foobar.png"];
    for file in filenames {
        let mime = find_mimetype(file);
        println!("MIME for {file}: {mime}");
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
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
}
