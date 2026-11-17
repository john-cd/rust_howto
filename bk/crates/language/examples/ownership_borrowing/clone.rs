// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Compares cloning owned contents and cloning shared ownership.
//!
//! Assigning a non-`Copy` value such as `String` moves ownership. Calling
//! `clone()` duplicates its buffer. Cloning an `Arc<String>` creates a new
//! handle and shares the buffer.

use std::sync::Arc;

/// Checks that cloning a string copies its contents and cloning an Arc shares them.
fn main() {
    // Create a String on the heap.
    let mut s1 = String::from("hello");

    // Clone the String, creating a new String with the same content on the
    // heap.
    let s2 = s1.clone();
    // The implementation of `Clone` for `String` allocates a new memory chunk
    // on the heap, deep copies the pointed-to string buffer of `s1` into
    // it, and stores a pointer to the new heap location.

    // Both `s1` and `s2` remain accessible after the clone.
    println!("{s1}");
    println!("{s2}");

    assert_eq!(s1, s2); // `s1` and `s2` have the same contents...

    // ...but `s1` and `s2` do NOT point to the same heap memory location.
    assert!(!std::ptr::eq(s1.as_ptr(), s2.as_ptr()));

    // `s1` and `s2` are independent of each other. Let's modify `s1`:
    s1.push('!');
    assert_ne!(s1, s2);

    let shared = Arc::new(s1);
    let second_owner = Arc::clone(&shared);
    assert!(Arc::ptr_eq(&shared, &second_owner));
}
// ANCHOR_END: example

/// Compares cloning a string and cloning a shared handle.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the clone example through its entry point.
    #[test]
    fn test_clone_example_entry_point() {
        main();
    }
}
