// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Parallel iterators with `rayon`: a sum of squares and an in-place update.
//!
//! Rayon turns a sequential iterator into a parallel one by changing `iter()`
//! to `par_iter()`. It splits the work across a pool of threads, one per CPU
//! core, and balances the load by letting idle threads steal work. It pays off
//! for CPU-bound work on large collections.

use rayon::prelude::*;
/// Calculates the sum of the squares of the elements in the input slice.
///
/// This function uses Rayon's parallel iterator to efficiently compute the sum
/// of squares across multiple threads.
///
/// # Arguments
/// * `input` - Input data whose fields determine the requested multithreading rayon operation.
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter().map(|i| i * i).sum()
}
/// Adds one to each element of `input`, in parallel.
///
/// # Arguments
/// * `input` - Mutable values to increment concurrently.
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

/// Runs the example entry point.
/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the multithreading rayon example through its entry point.
    #[test]
    fn test_multithreading_rayon_example_entry_point() {
        main();
    }

    /// Verifies that the increment then sum of squares.
    #[test]
    fn increment_then_sum_of_squares() {
        let mut v = [1, 2, 3];
        increment_all(&mut v);
        assert_eq!(v, [2, 3, 4]);
        assert_eq!(sum_of_squares(&v), 4 + 9 + 16);
    }

    /// Handles empty input gives zero input.
    #[test]
    fn empty_input_gives_zero() {
        assert_eq!(sum_of_squares(&[]), 0);
        increment_all(&mut []);
    }

    /// Verifies that the large inputs match the sequential result.
    #[test]
    fn large_inputs_match_the_sequential_result() {
        let v: Vec<i32> = (0..10_000).map(|i| i % 100).collect();
        let expected: i32 = v.iter().map(|i| i * i).sum();
        assert_eq!(sum_of_squares(&v), expected);
    }
}
