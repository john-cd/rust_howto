#![allow(dead_code)]
// // ANCHOR: example

/// 1. Avoid self-referential structs.
///    Note how each field points to a portion of the `String` owned by the struct.
struct CsvRecordTricky<'a> {
    line: String,
    fields: Vec<&'a str>,
}

/// 2. Better example of a struct that points to owned data outside of the
///    struct.
///    Both the line and each field have the same lifetime.
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

fn outside() {
    let s = "Joe,Doe,1994-01-01".to_string();
    let c = load_record(&s);
    println!("{c:?}");
}

// 3. If multiple references to the same data are needed,
//    `Rc<T>` (single-threaded) or `Arc<T>` (multi-threaded)
//    can be used instead.

use std::rc::Rc;

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
    outside();
    rc();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
