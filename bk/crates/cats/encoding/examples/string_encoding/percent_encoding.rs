#![allow(dead_code)]
// ANCHOR: example
//! The following demonstrates how to use percent encoding to handle special
//! characters in URLs.
//!
//! URLs use special characters to indicate the parts of the request. For
//! example, a `?` question mark marks the end of a path and the start of a
//! query string. Percent encoding replaces reserved characters with the `%`
//! escape character followed by a byte value as two hexadecimal digits. For
//! example, an ASCII space is replaced with `%20`.
//!
//! This example uses the NON_ALPHANUMERIC set to encode everything that is not
//! an ASCII letter or digit.

use anyhow::Result;
use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use percent_encoding::NON_ALPHANUMERIC;
use percent_encoding::percent_decode_str;
use percent_encoding::percent_encode;
use percent_encoding::utf8_percent_encode;

/// Encodes a string for use as a URL component.
fn encode_url_component(input: &str) -> String {
    percent_encode(input.as_bytes(), NON_ALPHANUMERIC).to_string()
}

/// Decodes a percent-encoded string.
fn decode_url_component(encoded: &str) -> Result<String> {
    // Decode the percent-encoded string and convert it to UTF-8.
    let decoded = percent_decode_str(encoded).decode_utf8()?.to_string();
    Ok(decoded)
}

// Custom encoding with a specific ASCII set.
fn custom_encode(input: &str) -> String {
    // Create a 'fragment' custom set that only encodes spaces and some special
    // characters.
    /// See <https://url.spec.whatwg.org/#fragment-percent-encode-set>.
    const FRAGMENT: &AsciiSet =
        &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
    // Percent-encode the UTF-8 encoding of the given string.
    utf8_percent_encode(input, FRAGMENT).to_string()
}

fn main() -> Result<()> {
    // Example URL component encoding:
    let original = "Hello, World! @#$%^&*()";
    let encoded = encode_url_component(original);
    let decoded = decode_url_component(&encoded)?;

    println!("Original:  {original}");
    println!("Encoded:   {encoded}");
    println!("Decoded:   {decoded}");

    // Demonstrate custom encoding:
    let custom_input = "special chars: %+";
    let custom_encoded = custom_encode(custom_input);
    println!("\nCustom Encoding:");
    println!("Original:  {custom_input}");
    println!("Encoded:   {custom_encoded}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() -> Result<()> {
        let test_cases = vec![
            "Hello, World!",
            "https://example.com/path?key=value",
            "特殊",
            "spaces and @ symbols",
        ];

        for input in test_cases {
            let encoded = encode_url_component(input);
            let decoded = decode_url_component(&encoded)?;

            assert_eq!(
                input, decoded,
                "Encoding and decoding failed for: {}",
                input
            );
        }

        Ok(())
    }

    #[test]
    fn test_custom_encoding() {
        let input = "test with spaces";
        let custom_encoded = custom_encode(input);

        assert!(custom_encoded.contains("%20"), "Spaces should be encoded");
        assert!(
            !custom_encoded.contains("%2B"),
            "Plus should not be encoded in this test"
        );
    }
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
// [dedupe with other example in percent_encode](https://github.com/john-cd/rust_howto/issues/1353)
