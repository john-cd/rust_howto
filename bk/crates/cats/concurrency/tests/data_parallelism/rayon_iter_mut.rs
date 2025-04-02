// ANCHOR: example
use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];

    // `par_iter_mut()` creates a parallel iterator over mutable references to
    // the elements of the array. `for_each()` applies a closure to each
    // element in parallel. In this case, the closure decrements each
    // element by 1. The order of execution is not guaranteed, but all
    // elements will be processed.
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
