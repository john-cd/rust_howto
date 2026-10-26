#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Scope and drop.

/// Creates a string in an inner scope, where it is dropped at the end.
fn main() {
    // `s` is not valid here, it is not yet declared.

    {
        // Start of the scope.

        // When a String is created, it requests memory from the heap.
        let s = String::from("hello");
        // `s` is valid from this point forward.

        println!("{s}");
    }
    // The scope is now over, and `s` is no longer valid.
    // Rust automatically returns this memory to the allocator.
    // This prevents memory leaks.

    // ERROR: println!("{s}");
}

// ANCHOR_END: example

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
