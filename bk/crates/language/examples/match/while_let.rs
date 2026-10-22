#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Looping with `while let`.

/// Pops the elements of a vector with `while let`.
fn main() {
    let mut stack = vec![1, 2, 3];

    // `while let` is a control flow construct that allows you to run a loop as
    // long as a pattern matches. We remove the last element of the vector
    // at each iteration, until the vector is empty and `pop` returns `None`,
    // which stops the loop.
    while let Some(top) = stack.pop() {
        println!("{top}");
    }
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
