use std::io;
use std::fs::File;

fn main() {
    let mut guess = String::new();

    io::stdin()
        // `read_line` puts whatever the user enters into the string we pass to it, but it also returns a Result value.
        .read_line(&mut guess)
        // If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect.
        .expect("Failed to read line");

    // Alternative: `unwrap` panics if there is an error - without a custom message.
    let _greeting_file = File::open("hello.txt").unwrap();
}
