#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Parallel iterators with `rayon`: a sum of squares and an in-place update.

use rayon::prelude::*;

/// Calculates the sum of the squares of the elements in the input slice.
///
/// This function uses Rayon's parallel iterator to efficiently compute the sum
/// of squares across multiple threads.
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter().map(|i| i * i).sum()
}

/// Adds one to each element of `input`, in parallel.
fn increment_all(input: &mut [i32]) {
    input.par_iter_mut().for_each(|p| *p += 1);
}

/// Applies [`sum_of_squares`] and [`increment_all`] to sample data.
fn main() {
    let mut v = [1, 2, 3];
    increment_all(&mut v[..]);
    println!("{}", sum_of_squares(&v[..]));
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
