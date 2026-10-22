#![allow(dead_code)]

// TODO Reviewed ??-??-2026
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
    println!("{}", std::str::from_utf8(&res).unwrap());
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
}
