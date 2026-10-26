#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Wipes a secret from memory when it is dropped, using `zeroize`'s derive
//! macros.

use zeroize::Zeroize;
use zeroize::ZeroizeOnDrop;
use zeroize::Zeroizing;

/// A key whose bytes are overwritten when the value is dropped.
///
/// `ZeroizeOnDrop` implements `Drop`; `Zeroize` allows an explicit wipe.
#[derive(Zeroize, ZeroizeOnDrop)]
struct ApiKey {
    /// The raw key bytes.
    bytes: Vec<u8>,
}

fn main() {
    // Explicit wipe: the buffer is emptied.
    let mut key = ApiKey {
        bytes: b"sk-live-1234".to_vec(),
    };
    key.zeroize();
    assert!(key.bytes.is_empty());

    // `Zeroizing<T>` wraps any `Zeroize` value, wiping it on drop.
    let passphrase =
        Zeroizing::new(String::from("correct horse battery staple"));
    println!("passphrase has {} bytes", passphrase.len());
    // The `String` is zeroed here, when `passphrase` goes out of scope.

    // Limits: `zeroize` clears only the memory it owns. Copies made earlier
    // (a `clone()`, a `String` that reallocated while growing, a buffer
    // swapped to disk or captured in a core dump) are not wiped. Reserve the
    // final capacity up front and avoid `clone()` on secrets.
    let mut buffer = Zeroizing::new(Vec::with_capacity(32));
    buffer.extend_from_slice(b"token");
    // `buffer` never reallocates: 5 bytes fit in the 32 reserved.
    assert!(buffer.capacity() >= 32);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroize_clears_bytes() {
        let mut key = ApiKey {
            bytes: vec![1, 2, 3],
        };
        key.zeroize();
        assert!(key.bytes.is_empty());
    }

    #[test]
    fn test_main() {
        main();
    }
}
