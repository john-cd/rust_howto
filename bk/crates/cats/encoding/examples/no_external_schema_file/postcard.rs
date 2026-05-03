#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `postcard` crate for serialization
//! and deserialization without an external schema file.
//!
//! Add to your `Cargo.toml` file:
//! ```toml
//! [dependencies]
//! postcard = "1.1.3" # Or latest version.
//! ```
//!
//! By default, `serde` has the `std` feature enabled, which makes it
//! unsuitable for embedded targets. Disabling default-features fixes this:
//! ```toml
//! serde = { version = "1.0.*", default-features = false }
//! ```
//! 
//! Example adapted from <https://lib.rs/crates/postcard>.
use std::ops::Deref;

use postcard::from_bytes;
use postcard::to_stdvec;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RefStruct<'a> {
    bytes: &'a [u8], /* NOTE: postcard handles `&[u8]` and `&[u8; N]`
                      * differently. */
    str_s: &'a str,
}

fn main() {
    let message = "hElLo";

    let bytes = [0x01, 0x10, 0x02, 0x20];

    let output = to_stdvec(&RefStruct {
        bytes: &bytes,
        str_s: message,
    })
    .expect("failed to serialize");

    assert_eq!(
        &[
            0x04, 0x01, 0x10, 0x02, 0x20, 0x05, b'h', b'E', b'l', b'L', b'o',
        ],
        output.deref()
    );

    let out: RefStruct =
        from_bytes(output.deref()).expect("failed to deserialize");
    assert_eq!(
        out,
        RefStruct {
            bytes: &bytes,
            str_s: message,
        }
    );
    println!(
        "Serialized to {} bytes and deserialized successfully",
        output.len()
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
