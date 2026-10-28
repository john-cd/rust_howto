// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of percent encoding and decoding strings.

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
}
