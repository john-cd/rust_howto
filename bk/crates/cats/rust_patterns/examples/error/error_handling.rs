#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates basic error handling in Rust using the `Result`
//! type.

use std::io;
use std::io::BufRead;

fn main() {
    // A `Cursor` behaves like a `File` but wraps an in-memory buffer.
    let mut my_cursor = io::Cursor::new(b"foo\nbar");
    let mut my_string = String::new();

    // `read_line` reads all bytes until a newline (the 0xA byte) is reached,
    // and append them to the provided String. `read_line` returns a
    // `Result` value, which is either success (`Ok`) or failure (`Err`).
    let result: Result<usize, std::io::Error> =
        my_cursor.read_line(&mut my_string);

    // We can process the `Result` as needed.
    match result {
        Ok(0) => println!("End of file reached."),
        Ok(n) => println!("The total number of bytes read is {n}."),
        Err(ref e) => eprintln!("Error: {}", e), /* We could also retry `read_line` to attempt to recover from the error. */
    }
    assert_eq!(my_string, "foo\n");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
