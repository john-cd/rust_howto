#![allow(dead_code)]
// ANCHOR: example
//! Example of percent encoding and decoding strings.

use std::str::Utf8Error;

use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use percent_encoding::percent_decode;
use percent_encoding::utf8_percent_encode;

/// <https://url.spec.whatwg.org/#fragment-percent-encode-set>
const FRAGMENT: &AsciiSet =
    &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn main() -> Result<(), Utf8Error> {
    // The input string we want to encode.
    // Note that the string contains spaces, which are not allowed in the
    // fragment part of a URL. The FRAGMENT AsciiSet will encode spaces as
    // %20.
    let input = "confident, productive systems programming";

    // Encode the string using the FRAGMENT AsciiSet.
    let iter = utf8_percent_encode(input, FRAGMENT);
    let encoded: String = iter.collect();
    println!("{}", encoded);
    assert_eq!(encoded, "confident,%20productive%20systems%20programming");

    // Decode the encoded string.
    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8()?;
    println!("{}", decoded);
    assert_eq!(decoded, "confident, productive systems programming");

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
