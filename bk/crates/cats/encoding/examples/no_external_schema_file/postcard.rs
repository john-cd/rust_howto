#![allow(dead_code)]
// TODO Reviewed ??-??-2026

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

use postcard::from_bytes;
use postcard::to_allocvec;
use serde::Deserialize;
use serde::Serialize;

/// A struct that borrows its data.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RefStruct<'a> {
    /// The bytes.
    bytes: &'a [u8], /* NOTE: postcard handles `&[u8]` and `&[u8; N]`
                      * differently. */
    /// The string.
    str_s: &'a str,
}

/// Serializes a struct with `postcard`, then deserializes it.
fn main() {
    let message = "hElLo";

    let bytes = [0x01, 0x10, 0x02, 0x20];

    let output = to_allocvec(&RefStruct {
        bytes: &bytes,
        str_s: message,
    })
    .expect("failed to serialize");

    let expected: &[u8] = &[
        0x04, 0x01, 0x10, 0x02, 0x20, 0x05, b'h', b'E', b'l', b'L', b'o',
    ];
    assert_eq!(expected, output.as_slice());

    let out: RefStruct = from_bytes(&output).expect("failed to deserialize");
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

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    /// `to_slice` writes into a caller-provided buffer: no allocation, which
    /// is what `no_std` targets need.
    #[test]
    fn to_slice_encodes_into_a_fixed_buffer() {
        let mut buf = [0_u8; 16];
        let used = postcard::to_slice(&(1_u8, "hi"), &mut buf)
            .expect("the buffer is large enough");
        assert_eq!(used, &[1, 2, b'h', b'i']);
    }

    #[test]
    fn a_buffer_that_is_too_small_is_an_error() {
        let mut buf = [0_u8; 2];
        assert!(postcard::to_slice(&(1_u8, "hi"), &mut buf).is_err());
    }

    #[test]
    fn truncated_input_is_an_error() {
        let output = to_allocvec(&RefStruct {
            bytes: &[1, 2],
            str_s: "abc",
        })
        .expect("failed to serialize");
        let truncated = &output[..output.len() - 1];
        assert!(from_bytes::<RefStruct>(truncated).is_err());
    }
}
