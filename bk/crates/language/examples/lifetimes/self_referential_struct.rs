#![allow(dead_code)]
// // ANCHOR: example
// use std::pin::Pin;

/// Example of a self-referential struct.
/// Each field point to a portion of the String owned by the struct.
struct CsvRecord0<'a> {
    line: String,
    fields: Vec<&'a str>,
}

/// Better example of a struct that points to owned data outside of the struct.
/// Both the line and each field have the same lifetime.
#[derive(Debug)]
struct CsvRecord<'a> {
    line: &'a str,
    fields: Vec<&'a str>,
}

fn load_record<'a>(line: &'a str) -> CsvRecord<'a> {
    let mut record = CsvRecord {
        line,
        fields: Vec::new(),
    };

    record.fields = record.line.split(',').collect();
    record
}

use std::marker::PhantomPinned;
use std::pin::Pin;

struct SelfRef {
    data: String,
    reference: *const String, // Raw pointer to avoid borrowing issues.
    _marker: PhantomPinned,   // Ensures struct implements `!Unpin`.
}

impl SelfRef {
    // Pin<Box<T>> ensures that the struct remains at a stable memory location,
    // preventing accidental moves.
    fn new(data: String) -> Pin<Box<SelfRef>> {
        let mut boxed = Box::pin(SelfRef {
            data,
            reference: std::ptr::null(),
            _marker: PhantomPinned,
        });

        let raw_ref = &boxed.data as *const String;
        unsafe { Pin::get_unchecked_mut(boxed.as_mut()).reference = raw_ref };

        boxed
    }
}

fn self_ref() {
    let self_ref = SelfRef::new("Hello, Rust!".to_string());
    println!("Data: {}", unsafe { &*self_ref.reference });
}

use std::rc::Rc;

// Or use Rc or Arc for Shared Ownership.
// If multiple references are needed, `Rc<T>` (single-threaded)
// or `Arc<T>` (multi-threaded) can be used.
struct SelfRefRc {
    data: Rc<String>,
    reference: Rc<String>,
}

impl SelfRefRc {
    fn new(data: String) -> Self {
        let shared_data = Rc::new(data);
        SelfRefRc {
            data: Rc::clone(&shared_data),
            reference: Rc::clone(&shared_data),
        }
    }
}

fn rc() {
    let self_ref = SelfRefRc::new("Hello, Rust!".to_string());
    println!("Data: {}", self_ref.reference);
}

fn main() {
    let s = "Joe,Doe,1994-01-01".to_string();
    let c = load_record(&s);
    println!("{:?}", c);

    rc();
    self_ref();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
