#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the use of `?`.

use std::fs::File;
use std::io;
use std::io::Read;

/// Reads the username from the file "temp/hello.txt".
///
/// # Returns
///
/// Returns `Ok(String)` containing the username if successful, or
/// `Err(io::Error)` if an error occurs.
fn read_username_from_file() -> Result<String, io::Error> {
    // Note the `?`. If `open` fails, `read_username_from_file` returns its
    // error, short-circuiting the rest of the function. This function does
    // not panic on I/O failure.
    let mut username_file = File::open("temp/hello.txt")?;
    let mut username = String::new();
    // Multiple `?` can be used within a function, but see the example below if
    // the error types are not the same.
    username_file.read_to_string(&mut username)?;
    // Note the final `Ok`. This is required by the return type of the function.
    Ok(username)
}

/// The main function demonstrates how to use `read_username_from_file` and
/// handle its result.
///
/// It either prints the username if the file is read successfully, or prints an
/// error message if an error occurs.
fn main() {
    match read_username_from_file() {
        Ok(name) => println!("User name: {}", name),
        Err(err) => println!("Error: {}", err),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    // Make sure the `temp` folder exists.
    if !std::fs::exists("temp").unwrap() {
        std::fs::create_dir("temp").unwrap();
    }
    main();
}
