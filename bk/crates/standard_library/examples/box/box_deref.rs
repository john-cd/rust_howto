#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Dereferencing a `Box`.

/// Reads the value inside a `Box` with `*`.
fn main() {
    // Dereferencing with `*`:
    let boxed: Box<u8> = Box::new(1);
    // Retrieve its inner value.
    let val: u8 = *boxed;
    println!("{val}");

    // Dereferencing with `.`: `Box<T>` behaves as if it were `T`.
    let boxed = Box::new("example");
    // Equivalent to (*boxed.deref()).len():
    let val = boxed.len();
    println!("{val}");
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
