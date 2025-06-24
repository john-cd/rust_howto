#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the use of `unwrap_or` and `unwrap_or_else` for handling
//! `Option` and `Result` types.
//!
//! `unwrap_or` provides a default value if the `Option` is `None` or the
//! `Result` is `Err`. `unwrap_or_else` provides a closure to compute a default
//! value in the same scenarios.
#![allow(clippy::unnecessary_literal_unwrap)]
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Use `unwrap_or` to provide a default value if `maybe_value` is `None`:
    let maybe_value: Option<i32> = None;
    let value: i32 = maybe_value.unwrap_or(0);
    println!("Value: {}", value); // Prints zero.

    // Use `unwrap_or` to provide a default value if a `Result` is `Err`.
    let result: Result<i32, &str> = Err("Something went wrong");
    let value: i32 = result.unwrap_or(42);
    println!("Value: {}", value); // Prints 42.

    // Use `unwrap_or_else` to compute a default value (within a closure)
    // when an `Option` is `None` or `Result` an `Err`.
    let some_option: Option<&str> = None;
    let value: &str = some_option.unwrap_or_else(|| {
        println!("Option was None. Providing default value.");
        "default_value"
    });
    println!("Value: {}", value);

    // Fallback example: try to open a file; if the file was not found, try to
    // create the file. Otherwise, panic.
    let _greeting_file = File::open("temp/hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("temp/hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
// ANCHOR_END: example

#[test]
fn test() {
    use std::fs;
    if !fs::exists("temp").unwrap() {
        fs::create_dir("temp").unwrap();
    }
    main();
}
