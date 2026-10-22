#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Mutating the elements of an array in parallel with `rayon`.

use rayon::prelude::*;

/// Decrements each element of an array in parallel and prints the result.
fn main() {
    let mut arr = [0, 7, 9, 11];

    // `par_iter_mut()` creates a parallel iterator over mutable references to
    // the elements of the array. `for_each()` applies a closure to each
    // element in parallel. In this case, the closure decrements each
    // element by 1. The order of execution is not guaranteed, but all
    // elements will be processed.
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{arr:?}");
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
