#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Searching bytes quickly with `memchr`.

use memchr::memchr;

// Use `memchr` to efficiently search for a byte in a slice.

// Add the dependency to Cargo.toml:
// ```toml
// [dependencies]
// memchr = "2.7.4" # Or latest
// ```

/// Finds the first, last and all occurrences of a byte.
fn main() {
    let haystack = b"hello world";

    // Find first occurrence of 'o'.
    match memchr(b'o', haystack) {
        Some(index) => println!("Found 'o' at index: {index}"),
        None => println!("Character not found"),
    }

    // Using `memchr` in a function.
    /// Returns the indexes of all occurrences of `needle` in `haystack`.
    fn find_all_occurrences(needle: u8, haystack: &[u8]) -> Vec<usize> {
        let mut indices = Vec::new();
        let mut start = 0;

        while let Some(pos) = memchr(needle, &haystack[start..]) {
            let absolute_pos = start + pos;
            indices.push(absolute_pos);
            start = absolute_pos + 1;

            if start >= haystack.len() {
                break;
            }
        }

        indices
    }

    // Find all occurrences of 'l':
    let all_l_positions = find_all_occurrences(b'l', haystack);
    println!("All 'l' positions: {all_l_positions:?}");
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
