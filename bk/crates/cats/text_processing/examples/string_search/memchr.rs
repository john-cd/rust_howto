// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Searching bytes quickly with `memchr`.

use memchr::memchr;
use memchr::memchr_iter;

// Use `memchr` to efficiently search for a byte in a slice.

/// Demonstrates finding the first and all occurrences of a byte.
fn main() {
    let haystack = b"hello world";

    // Find first occurrence of 'o'.
    match memchr(b'o', haystack) {
        Some(index) => println!("Found 'o' at index: {index}"),
        None => println!("Character not found"),
    }

    // Find all occurrences of 'l':
    let all_l_positions: Vec<usize> = memchr_iter(b'l', haystack).collect();
    println!("All 'l' positions: {all_l_positions:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the memchr example through its entry point.
    #[test]
    fn test_memchr_example_entry_point() {
        main();
    }

    /// The first and all occurrences of a byte are found.
    #[test]
    fn finds_bytes() {
        let haystack = b"hello world";
        assert_eq!(memchr(b'o', haystack), Some(4));
        assert_eq!(memchr_iter(b'l', haystack).collect::<Vec<_>>(), [2, 3, 9]);
    }

    /// A missing byte and an empty haystack give no matches.
    #[test]
    fn missing_byte_is_not_found() {
        assert_eq!(memchr(b'z', b"hello"), None);
        assert_eq!(memchr(b'a', b""), None);
        assert_eq!(memchr_iter(b'z', b"hello").count(), 0);
    }
}
