// ANCHOR: example
use rayon::prelude::*;

/// Sorts the vector in parallel using Rayon's `par_sort` method.
fn main() {
    let mut v = [-5, 4, 1, -3, 2];
    v.par_sort();
    println!("{:#?}", v);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
