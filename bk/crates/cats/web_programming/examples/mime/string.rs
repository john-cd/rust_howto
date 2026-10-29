// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates parsing MIME types from strings and handling invalid input.

use mime::APPLICATION_OCTET_STREAM;
use mime::Mime;

/// Parses MIME types from strings, using a default value when parsing fails.
fn main() {
    let invalid_mime_type = "i n v a l i d";
    let default_mime = invalid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "MIME for {invalid_mime_type:?} used default value {default_mime:?}"
    );

    let valid_mime_type = "TEXT/PLAIN";
    let parsed_mime = valid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!("MIME for {valid_mime_type:?} was parsed as {parsed_mime:?}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the string example through its entry point.
    #[test]
    fn test_string_example_entry_point() {
        main();
    }

    /// A valid MIME type is parsed, case-insensitively.
    #[test]
    fn parses_a_valid_mime_type() {
        let parsed = "TEXT/PLAIN".parse::<Mime>().unwrap();
        assert_eq!(parsed, mime::TEXT_PLAIN);
        assert_eq!(parsed.type_(), mime::TEXT);
        assert_eq!(parsed.subtype(), mime::PLAIN);
    }

    /// An invalid MIME type falls back to the default.
    #[test]
    fn invalid_mime_type_uses_the_default() {
        assert!("i n v a l i d".parse::<Mime>().is_err());
        let fallback = "i n v a l i d"
            .parse::<Mime>()
            .unwrap_or(APPLICATION_OCTET_STREAM);
        assert_eq!(fallback, APPLICATION_OCTET_STREAM);
    }
}
