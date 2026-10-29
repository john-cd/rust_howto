// TODO Reviewed ??-??-2026.

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

    #[test]
    fn par_sort_orders_the_values() {
        let mut v = [-5, 4, 1, -3, 2];
        v.par_sort();
        assert_eq!(v, [-5, -3, 1, 2, 4]);
    }

    #[test]
    fn par_sort_handles_empty_and_large_inputs() {
        let mut empty: [i32; 0] = [];
        empty.par_sort();
        let mut big: Vec<i32> = (0..10_000).rev().collect();
        big.par_sort();
        assert!(big.windows(2).all(|w| w[0] <= w[1]));
    }
}
