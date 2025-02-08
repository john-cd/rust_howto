// ANCHOR: example
use std::io;
use std::io::BufRead;

fn main() {
    let mut my_cursor = io::Cursor::new(b"foo\nbar"); // A `Cursor` behaves like a `File` (but wraps a in-memory buffer).
    let mut my_string = String::new();

    // `readline` reads all bytes until a newline (the 0xA byte) is reached, and
    // append them to the provided String. `readline` returns a `Result`
    // value, which is either success (`Ok`) or failure (`Err`).
    let result: Result<usize, std::io::Error> =
        my_cursor.read_line(&mut my_string);
    // We can process the Result as needed - here we just print the status.
    match result {
        Ok(0) => println!("End of file reached."),
        Ok(n) => println!("The total number of bytes read is {n}."),
        Err(ref e) => eprintln!("Error: {}", e), /* We could also retry `read_line` to attempt to recover from the error. */
    }
    assert_eq!(my_string, "foo\n");

    // Alternatively, if the error is truly not recoverable, we could cause the
    // program to crash if this instance of `Result` is an `Err` value.
    // `expect` panics and displays the message that you passed as an argument
    // to.
    result.expect("Failed to read line");
    // You may also use `unwrap`, which panics if there is an error but does not
    // display a custom message. result.unwrap();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
