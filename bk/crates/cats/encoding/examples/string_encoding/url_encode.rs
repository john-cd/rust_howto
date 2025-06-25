#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to URL-encode and decode strings using the
//! `url` crate.

use url::form_urlencoded::byte_serialize;
use url::form_urlencoded::parse;

fn main() {
    // Encode a string to URL-encoded format.
    // The `byte_serialize` function takes a byte slice and returns an iterator
    // over the encoded bytes.
    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded:'{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded:'{}'", decoded);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
