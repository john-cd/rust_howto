#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! String data types: `&str`, `String` and string literals.

/// Creates and prints strings of different types.
fn main() {
    // Basic string types:
    let u: &str = "Hello"; // String slice.
    let v: String = String::from("World"); // Owned string.

    println!("String slice: {u}");
    println!("Owned string: {v}");

    println!(
        "String concatenation: {} + {} = {}",
        u,
        v,
        u.to_owned() + &v
    );
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
