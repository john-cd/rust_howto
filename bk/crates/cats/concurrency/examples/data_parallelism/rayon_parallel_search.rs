#![allow(dead_code)]
// ANCHOR: example
use rayon::prelude::*;

/// Demonstrates parallel searching using Rayon's `par_iter().find_any()`.
fn main() {
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    // Search for specific elements in parallel.
    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
    println!("{v:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
