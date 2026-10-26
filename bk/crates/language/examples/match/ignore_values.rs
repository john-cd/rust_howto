#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Ignoring values in patterns with `_` and `..`.

/// Ignores parts of a value when destructuring.
fn main() {
    // `_unused` is bound to the value `42`, but the compiler does not complain
    // if it is not used later.
    let _unused = 42;

    let numbers = (2, 4, 8, 16, 32);

    // Use `..` to ignore e.g. the middle elements:
    let (first, .., last) = numbers;
    println!("Some numbers: first = {first}, last = {last}");

    // Use `_` to ignore a value completely:
    /// Prints the second argument while ignoring the first.
    fn foo(_: i32, y: i32) {
        // The first parameter is ignored.
        println!("y is: {y}");
    }

    foo(42, 43);

    let values: &[i32] = &[1, 2, 3, 4];
    // Match the first and last elements while ignoring the middle elements.
    if let [first, .., last] = values {
        assert_eq!((*first, *last), (1, 4));
    }

    // Bind the remainder after the first element with `rest @ ..`.
    if let [first, rest @ ..] = values {
        assert_eq!(*first, 1);
        assert_eq!(rest, &[2, 3, 4]);
    }
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
