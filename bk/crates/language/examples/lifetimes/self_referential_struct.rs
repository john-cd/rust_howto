//! Alternatives to self-referential structs.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example

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
    println!("Data: {} and {}", self_ref.data, self_ref.reference);
}

/// Builds the anti-pattern, which can only be created with empty `fields`.
fn tricky() {
    let record = CsvRecordTricky {
        line: "Joe,Doe".to_owned(),
        fields: Vec::new(),
    };
    println!("{} ({} fields)", record.line, record.fields.len());
}

/// Runs both alternatives.
fn main() {
    tricky();
    outside();
    rc();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the self referential struct example through its entry point.
    #[test]
    fn test_self_referential_struct_example_entry_point() {
        main();
    }

    /// The record splits its line into comma-separated fields.
    #[test]
    fn test_load_record() {
        let line = "Joe,Doe,1994-01-01";
        let record = load_record(line);
        assert_eq!(record.line, line);
        assert_eq!(record.fields, vec!["Joe", "Doe", "1994-01-01"]);
        assert_eq!(load_record("").fields, vec![""]);
        assert_eq!(load_record("a,,b").fields, vec!["a", "", "b"]);
    }

    /// Both `Rc` handles point to the same allocation.
    #[test]
    fn test_self_ref_rc() {
        let s = SelfRefRc::new("Hello, Rust!".to_owned());
        assert!(Rc::ptr_eq(&s.data, &s.reference));
        assert_eq!(*s.reference, "Hello, Rust!");
        assert_eq!(Rc::strong_count(&s.data), 2);
    }
}
