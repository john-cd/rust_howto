#![allow(dead_code)]
// ANCHOR: example
use rayon::prelude::*;

/// This example demonstrates the use of `any` and `all` methods on a parallel
/// iterator.
fn main() {
    let mut vec = vec![2, 4, 6, 8];

    // Check if any element is odd (should be false) and if all elements are
    // even (should be true). Also check if any element is greater than 8
    // (should be false) and if all elements are less than or equal to 8 (should
    // be true).
    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8));
    assert!(vec.par_iter().all(|n| *n <= 8));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8));
    assert!(!vec.par_iter().all(|n| *n <= 8));

    println!("{:?}", vec);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
