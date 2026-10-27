#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Printing the type of a value with `type_name_of_val`.

/// Returns an iterator over a range of numbers.
fn get_iter() -> impl Iterator<Item = i32> {
    [1, 2, 3].into_iter()
}

/// Prints the type name of the iterator.
fn main() {
    let iter = get_iter();
    // NEW in Rust 1.76
    // Returns the type name of the pointed-to value as a string slice.
    let iter_name = std::any::type_name_of_val(&iter);
    let sum: i32 = iter.sum();
    println!("The sum of the `{iter_name}` is {sum}.");
    // prints: The sum of the `core::array::iter::IntoIter<i32, 3>` is 6.
    println!("A Vec<u8> has type `{}`.", std::any::type_name::<Vec<u8>>());
    println!("{}:{} in {}", file!(), line!(), module_path!());
    let backtrace = std::backtrace::Backtrace::capture();
    println!("Backtrace status: {:?}", backtrace.status());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the type and source-location diagnostics can be evaluated.
    /// Runs the type name of val example through its entry point.
    #[test]
    fn test_type_name_of_val_example_entry_point() {
        main();
    }
}
