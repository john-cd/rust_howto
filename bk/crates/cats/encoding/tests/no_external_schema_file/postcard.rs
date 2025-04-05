// ANCHOR: example
//! This example demonstrates how to use the `postcard` crate for serialization
//! and deserialization without an external schema file.
//!
//! Add to your `Cargo.toml` file:
//! ```toml
//! [dependencies]
//! postcard = "1.0.0"
//! ```
//!
//! By default, `serde` has the `std` feature enabled, which makes it
//! unsuitable for embedded targets. Disabling default-features fixes this:
//! ```toml
//! serde = { version = "1.0.*", default-features = false }
//! ```
//! Example adapted from https://lib.rs/crates/postcard
use std::ops::Deref;

use heapless::Vec;
use postcard::from_bytes;
use postcard::to_vec;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RefStruct<'a> {
    bytes: &'a [u8],
    str_s: &'a str,
}

fn main() {
    let message = "hElLo";

    let bytes = [0x01, 0x10, 0x02, 0x20];

    let output: Vec<u8, 11> = to_vec(&RefStruct {
        bytes: &bytes,
        str_s: message,
    })
    .unwrap();

    assert_eq!(
        &[
            0x04, 0x01, 0x10, 0x02, 0x20, 0x05, b'h', b'E', b'l', b'L', b'o',
        ],
        output.deref()
    );

    let out: RefStruct = from_bytes(output.deref()).unwrap();
    assert_eq!(
        out,
        RefStruct {
            bytes: &bytes,
            str_s: message,
        }
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [review postcard example](https://github.com/john-cd/rust_howto/issues/1038)
