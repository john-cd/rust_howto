#![allow(dead_code)]
// ANCHOR: example
use std::io;
use std::io::Write;

/// Retrieve user input from the standard input:
fn get_and_print_input() -> Result<String, io::Error> {
    println!("Please type your first name and press Enter:");
    // Create a mutable `String` buffer:
    let mut user_input = String::new();
    // Get a handle to stdin and read a line into the buffer:
    io::stdin().read_line(&mut user_input)?;
    Ok(user_input)
}

fn main() -> io::Result<()> {
    // Instead of using `print*!` to write to the standard output,
    // you may get an explicit handle...
    let stdout = io::stdout();
    // ...and lock the handle if you want exclusive access:
    let mut handle = stdout.lock();
    // Write a byte slice to the locked handle:
    handle.write_all(b"This is written to the standard output.\n")?;
    // The lock is automatically released when `handle` goes out of scope.

    // The same applies to the standard error of the current process:
    let mut stderr = io::stderr();
    // We could lock `stderr` as well.
    stderr.write_all(b"There was an error.\n")?;
    // This is equivalent to `eprintln!`.

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
