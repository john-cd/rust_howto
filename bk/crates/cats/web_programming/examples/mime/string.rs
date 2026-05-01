#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates parsing MIME types from strings and handling invalid input.

use mime::APPLICATION_OCTET_STREAM;
use mime::Mime;

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

#[test]
fn test() {
    main();
}
