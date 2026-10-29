// TODO Reviewed ??-??-2026.

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

    /// Runs the string data types example through its entry point.
    #[test]
    fn test_string_data_types_example_entry_point() {
        main();
    }

    // Output-only example: `main` only prints values; there is nothing else to
    // check.
}
