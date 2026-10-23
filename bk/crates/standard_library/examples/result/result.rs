#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! `Result` is a type that represents either success (`Ok`) or failure (`Err`):
//!
//! ```
//! pub enum Result<T, E> {
//!     Ok(T),
//!     Err(E),
//! }
//! ```

use std::fs::File;
use std::io;
use std::io::Read as _;
use std::num::ParseIntError;

/// Opens a file and handles the `Result` where it is returned.
fn open_file(file_path: &str) {
    // Faillible functions like `File::open` return a `Result`...
    let open_result: Result<File, io::Error> = File::open(file_path);

    // We could handle their `Result` there and then...
    match open_result {
        Err(e) => eprintln!("An error occurred while opening the file: {e}"),
        Ok(file) => println!("Opened {file:?}"),
    }
}

// ...but most often you'll want to return early when encountering an error,
// and propagate the error up the call stack.
/// Reads a file into a string, returning early on errors.
#[allow(clippy::question_mark)]
fn read_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(e), // Early return.
    };

    // This said, having to use multiple `match` or `if let` is verbose
    // when chaining calls to faillible methods...
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        Err(e)
    } else {
        Ok(contents)
    }
}

// Therefore, use the `?` operator as a shortcut to return early
// in case of an error. The following is equivalent to the previous function.
/// Reads a file into a string, using the `?` operator.
fn read_file2(file_path: &str) -> Result<String, io::Error> {
    let mut file: File = File::open(file_path)?;
    // Note that `file` is of type `File`,
    // not `io::Result<File> = Result<File, io::Error>`.
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
// You can even chain method calls immediately after the `?`, e.g.
// `File::open(file_path)?.read_to_string(&mut contents)?;`.

// You will often need to return one of multiple `Result` types.
// You could create a custom `MyError` error to do so:
/// Reads a number from a file, converting I/O and parsing errors into
/// `MyError`.
fn read_and_parse_file(file_path: &str) -> Result<i32, MyError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // `read_to_string` returns `Result<_, io::Error>`.
    let number = contents.trim().parse()?; // `parse` returns `Result<_, std::num::ParseIntError>`.
    Ok(number)
}

/// An error that wraps I/O and parsing errors.
#[allow(dead_code)]
#[derive(Debug)]
enum MyError {
    /// An I/O error.
    Io(io::Error),
    /// A parsing error.
    Parse(ParseIntError),
}

impl std::error::Error for MyError {}

// Implementing the `Error` trait requires that `Debug` and `Display` are
// implemented too.
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "IO error: {err}"),
            Self::Parse(err) => write!(f, "Parse error: {err}"),
        }
    }
}

// We should also implement the `From` trait:
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        Self::Parse(err)
    }
}

// The above boilerplate can be avoided with the `thisError` crate,
// which provides a convenient `derive` macro
// for the standard library's `std::error::Error` trait.
// Use it when writing libraries, in which case custom error types are
// recommended.
/// An error that wraps I/O and parsing errors, defined with `thiserror`.
#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
enum MyError2 {
    /// An I/O error.
    #[error("Io error")]
    Io(#[from] io::Error),
    /// A parsing error.
    #[error("Parse error")]
    Parse(#[from] ParseIntError),
}

// A simpler method than returning a custom error type
// may be to return a trait object (`dyn Error`, typically wrapped in a `Box`)
// at the cost of greater opacity...
/// Reads a number from a file, boxing the errors as `dyn Error`.
fn read_and_parse_file2(
    file_path: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse()?;
    Ok(number)
}

// ...but crates like `anyhow` can simplify error management a great deal...
// `anyhow::Result<T>` is equivalent to `std::result::Result<T, anyhow::Error>`.
// Use it in applications, since custom error types are rarely needed then.
/// Reads a number from a file, using `anyhow` for errors.
fn read_and_parse_file3(file_path: &str) -> anyhow::Result<i32> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse()?;
    Ok(number)
}

// A function without return value in truth returns `Unit`:
// `fn func() {...}` is a shorthand for `fn func() -> () { ... }`
// To convert it to a faillible function, return `Result<(), SomeErrorType>`.
/// Reads a number from a file and returns `()` on success.
fn unit_return_value(file_path: &str) -> Result<(), anyhow::Error> {
    let _i: i32 = read_and_parse_file3(file_path)?;
    println!("I don't return anything!");
    // Do not forget to return a `Result` in the happy path!
    // The double parentheses are required. It is really `Ok( () )`.
    Ok(())
}

// `main()` can return a `Result`, more precisely a `Result<T, E> where T:
// Termination, E: Debug)`. `Result<(), Box<dyn std::error::Error>>` or
// `anyhow::Result<()>` are common choices.
/// Calls the functions that read files and handle their results.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "example.txt";
    open_file(file_path);
    // Here we ignore the `Result` return values
    // to avoid returning early and to exercise all the code.
    // You will rarely need to do this.
    let _ = read_file(file_path);
    let _ = read_file2(file_path);
    let _ = read_and_parse_file(file_path);
    let _ = read_and_parse_file2(file_path);
    let _ = read_and_parse_file3(file_path);
    let _ = unit_return_value(file_path);
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main().unwrap();
    }
}
