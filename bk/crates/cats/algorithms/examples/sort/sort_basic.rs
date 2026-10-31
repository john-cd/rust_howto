// Reviewed 10-27-2026.

// ANCHOR: example
//! This example demonstrates the basic usage of the `sort` method on a vector.
//!
//! Sorting orders a collection so that searching and comparing become faster.
//! Rust's `sort` is stable, which means equal elements keep their original
//! relative order, and runs in O(n log n) time.

fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];

    // Sort the vector in ascending order.
    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    println!("{vec:?}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the sort basic example through its entry point.
    #[test]
    fn test_sort_basic_example_entry_point() {
        main();
    }
}
