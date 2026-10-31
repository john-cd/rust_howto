// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of percent encoding and decoding strings.
//!
//! Percent-encoding makes text safe for URLs by replacing unsafe characters
//! with `%` and their hex value (a space becomes `%20`). Which characters need
//! encoding depends on the part of the URL: a path, a query string or a
//! fragment.

use std::str::Utf8Error;

use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use percent_encoding::percent_decode;
use percent_encoding::utf8_percent_encode;

/// <https://url.spec.whatwg.org/#fragment-percent-encode-set>.
const FRAGMENT: &AsciiSet =
    &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// Percent-encodes a string, then decodes it.
fn main() -> Result<(), Utf8Error> {
    // The input string we want to encode.
    // The FRAGMENT AsciiSet encodes spaces as %20 while leaving the plus sign
    // unchanged.
    let input = "confident, +productive systems programming";

    // Encode the string using the FRAGMENT AsciiSet.
    let iter = utf8_percent_encode(input, FRAGMENT);
    let encoded: String = iter.collect();
    println!("{encoded}");
    assert_eq!(encoded, "confident,%20+productive%20systems%20programming");

    // Decode the encoded string.
    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8()?;
    println!("{decoded}");
    assert_eq!(decoded, "confident, +productive systems programming");

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Utf8Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the percent encode example through its entry point.
    #[test]
    fn test_percent_encode_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Reserved characters are escaped; invalid UTF-8 is an error.
    #[test]
    fn escapes_and_rejects_invalid_utf8() -> Result<(), Utf8Error> {
        let encoded: String =
            utf8_percent_encode("a <b>\"c\"", FRAGMENT).collect();
        assert_eq!(encoded, "a%20%3Cb%3E%22c%22");
        assert_eq!(
            percent_decode(b"%41%20%e2%9d%a4").decode_utf8()?,
            "A \u{2764}"
        );
        assert!(percent_decode(b"%ff").decode_utf8().is_err());
        Ok(())
    }
}
