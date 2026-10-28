// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates basic usage of the `bytes` crate.

use bytes::BufMut as _;
use bytes::BytesMut;

/// Builds a `BytesMut` buffer, then freezes it into an immutable `Bytes`.
fn main() {
    let mut buf = BytesMut::with_capacity(64);
    buf.put(&b"hello "[..]);
    buf.put(&b"world"[..]);

    let res = buf.freeze();
    assert_eq!(res, "hello world");
    println!(
        "{}",
        std::str::from_utf8(&res)
            .expect("sample plaintext should be valid UTF-8")
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the bytes example through its entry point.
    #[test]
    fn test_bytes_example_entry_point() {
        main();
    }
}
