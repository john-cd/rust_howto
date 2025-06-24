#![allow(dead_code)]
// ANCHOR: example
use rayon::prelude::*;

/// Calculates the sum of the squares of the elements in the input slice.
///
/// This function uses Rayon's parallel iterator to efficiently compute the sum
/// of squares across multiple threads.
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter().map(|i| i * i).sum()
}

fn increment_all(input: &mut [i32]) {
    input.par_iter_mut().for_each(|p| *p += 1);
}

fn main() {
    let mut v = [1, 2, 3];
    increment_all(&mut v[..]);
    println!("{}", sum_of_squares(&v[..]));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
