#![allow(unused)]
// ANCHOR: example
//! Converting Custom Errors.
use std::io;

#[derive(Debug)]
pub(crate) enum MyError {
    Io(io::Error),
    Message(String),
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<&str> for MyError {
    fn from(msg: &str) -> Self {
        MyError::Message(msg.to_string())
    }
}

fn main() -> Result<(), MyError> {
    println!("Testing custom error conversion with the ? operator:");
    let _file = std::fs::File::open("missing.txt")?; // `io::Error` becomes `MyError`.
    Err("custom failure")?; // `&str` becomes `MyError`.
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), MyError> {
    main()
}

fn test_main() {
    assert!(main().is_err());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        assert!(main().is_err());
    }
}
