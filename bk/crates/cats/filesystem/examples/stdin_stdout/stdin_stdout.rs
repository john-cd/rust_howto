// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Reading from standard input and writing to standard output.

use std::io;
use std::io::BufRead;
use std::io::Write as _;

/// Reads a line from the provided input after prompting the user.
fn get_and_print_input<R: BufRead>(mut input: R) -> Result<String, io::Error> {
    println!("Please type your first name and press Enter:");
    let mut user_input = String::new();
    input.read_line(&mut user_input)?;
    Ok(user_input)
}

/// Writes to a locked standard output and reads a line.
fn main() -> io::Result<()> {
    let user_input = get_and_print_input(io::stdin().lock())?;
    println!("You entered: {}", user_input.trim_end());

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
    // `eprintln!` also writes a line to standard error, with formatting.

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> io::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reads supplied input without blocking on interactive standard input.
    /// Runs the stdin stdout example through its entry point.
    #[test]
    fn test_stdin_stdout_example_entry_point() -> anyhow::Result<()> {
        let input = io::Cursor::new("Ada\n");
        assert_eq!(get_and_print_input(input)?, "Ada\n");
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
