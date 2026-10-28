// TODO Reviewed ??-??-2026.

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

/// Serializes and deserializes a borrowed structure with `postcard`.
///
/// # Errors
///
/// Returns an error if serialization or deserialization fails.
fn main() -> anyhow::Result<()> {
    let message = "hElLo";

    let bytes = [0x01, 0x10, 0x02, 0x20];

    let output = to_allocvec(&RefStruct {
        bytes: &bytes,
        str_s: message,
    })?;

    let expected: &[u8] = &[
        0x04, 0x01, 0x10, 0x02, 0x20, 0x05, b'h', b'E', b'l', b'L', b'o',
    ];
    assert_eq!(
        expected,
        output.as_slice(),
        "unexpected postcard wire format"
    );

    let out: RefStruct = from_bytes(&output)?;
    assert_eq!(
        out,
        RefStruct {
            bytes: &bytes,
            str_s: message,
        },
        "deserialization must preserve both borrowed fields"
    );
    println!(
        "Serialized to {} bytes and deserialized successfully",
        output.len()
    );
    Ok(())
}

// ANCHOR_END: example

/// # Errors
///
/// Returns an error if serialization or deserialization fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The borrowed sample can be serialized and deserialized unchanged.
    #[test]
    fn borrowed_fields_round_trip() -> anyhow::Result<()> {
        main()
    }

    /// `to_slice` writes into a caller-provided buffer: no allocation, which
    /// is useful on `no_std` targets and when the caller owns the output
    /// buffer.
    #[test]
    fn to_slice_encodes_into_a_fixed_buffer() -> anyhow::Result<()> {
        let mut buf = [0_u8; 16];
        let used = postcard::to_slice(&(1_u8, "hi"), &mut buf)?;
        assert_eq!(used, &[1, 2, b'h', b'i'], "unexpected postcard bytes");
        Ok(())
    }

    /// A fixed buffer that cannot hold the encoded value returns an error.
    #[test]
    fn a_buffer_that_is_too_small_is_an_error() {
        let mut buf = [0_u8; 2];
        assert!(
            postcard::to_slice(&(1_u8, "hi"), &mut buf).is_err(),
            "encoding should report insufficient output space"
        );
    }

    /// Truncated serialized data is rejected instead of partially decoding.
    #[test]
    fn truncated_input_is_an_error() -> anyhow::Result<()> {
        let output = to_allocvec(&RefStruct {
            bytes: &[1, 2],
            str_s: "abc",
        })?;
        let truncated = &output[..output.len() - 1];
        assert!(
            from_bytes::<RefStruct>(truncated).is_err(),
            "deserialization should report truncated input"
        );
        Ok(())
    }
}
