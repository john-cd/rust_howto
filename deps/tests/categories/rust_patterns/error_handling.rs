// ANCHOR: example
use std::io;
use std::io::BufRead;

fn main() {
    let mut cursor = io::Cursor::new(b"foo\nbar");
    let mut buf = String::new();

    cursor
        // `read_line` puts whatever the user enters into the string we pass to it,
        // but it also returns a `Result` value.
        .read_line(&mut buf)
        // If this instance of `Result` is an `Err` value, expect will cause the program to crash
        // and display the message that you passed as an argument to expect.
        .expect("Failed to read line");

    // Alternative: `unwrap` panics if there is an error
    // let _greeting_file = std::fs::File::open("temp/hello.txt").unwrap();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO P1
