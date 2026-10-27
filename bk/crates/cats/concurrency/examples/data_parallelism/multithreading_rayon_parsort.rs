#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Sorts the vector in parallel using Rayon's `par_sort` method.

use rayon::prelude::*;

/// Sorts the vector in parallel using Rayon's `par_sort` method.
fn main() {
    let mut v = [-5, 4, 1, -3, 2];
    v.par_sort();
    println!("{v:#?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the multithreading rayon parsort example through its entry point.
    #[test]
    fn test_multithreading_rayon_parsort_example_entry_point() {
        main();
    }
}
