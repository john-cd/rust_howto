//! Alternatives to self-referential structs.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// // ANCHOR: example

/// 1. Avoid self-referential structs. Note how each field points to a portion
///    of the `String` owned by the struct.
struct CsvRecordTricky<'a> {
    /// The line, owned by the struct.
    line: String,
    /// The fields, which would borrow from `line`.
    fields: Vec<&'a str>,
}

/// 2. Better example of a `struct` that points to owned data outside of the
///    `struct`. Both the line and each field have the same lifetime.
#[derive(Debug)]
struct CsvRecord<'a> {
    /// The line, borrowed from outside the struct.
    line: &'a str,
    /// The fields, borrowed from the same line.
    fields: Vec<&'a str>,
}

/// Splits a comma-separated line into a record that borrows from the line.
fn load_record(line: &str) -> CsvRecord<'_> {
    let mut record = CsvRecord {
        line,
        fields: Vec::new(),
    };

    record.fields = record.line.split(',').collect();
    record
}

/// Builds a [`CsvRecord`] that borrows from a string outside the struct.
fn outside() {
    let s = "Joe,Doe,1994-01-01".to_owned();
    let c = load_record(&s);
    println!("{c:?}");
}

// 3. If multiple references to the same data are needed, `Rc<T>`
//    (single-threaded) or `Arc<T>` (multi-threaded) can be used instead.

use std::rc::Rc;

/// A struct whose fields share ownership of the same string with `Rc`.
struct SelfRefRc {
    /// The shared string.
    data: Rc<String>,
    /// A second handle to the same string.
    reference: Rc<String>,
}

impl SelfRefRc {
    /// Creates a struct whose two fields share `data`.
    fn new(data: String) -> Self {
        let shared_data = Rc::new(data);
        Self {
            data: Rc::clone(&shared_data),
            reference: Rc::clone(&shared_data),
        }
    }
}

/// Builds and prints a [`SelfRefRc`].
fn rc() {
    let self_ref = SelfRefRc::new("Hello, Rust!".to_owned());
    println!("Data: {}", self_ref.reference);
}

/// Runs both alternatives.
fn main() {
    outside();
    rc();
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
