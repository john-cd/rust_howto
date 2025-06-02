#![allow(dead_code)]
// // ANCHOR: example
// use std::pin::Pin;

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

// TODO cover Pin, UnPin

// // Pin<Box<T>> ensures that the struct remains at a stable memory location,
// preventing accidental moves.

// struct SelfRef {
//     data: String,
//     reference: *const String, // Raw pointer to avoid borrowing issues.
//     _marker: PhantomPinned,   // Ensures struct implements !Unpin
// }

// impl SelfRef {
//     fn new(data: String) -> Pin<Box<SelfRef>> {
//         let mut boxed = Box::pin(SelfRef {
//             data,
//             reference: std::ptr::null(),
//             _marker: PhantomPinned,
//         });

//         let raw_ref = &boxed.data as *const String;
//         unsafe { Pin::get_unchecked_mut(boxed.as_mut()).reference = raw_ref
// };

//         boxed
//     }
// }

// fn self_ref() {
//     let self_ref = SelfRef::new("Hello, Rust!".to_string());
//     println!("Data: {}", unsafe { &*self_ref.reference });
// }

// // Using Rc or Arc for Shared Ownership
// // If multiple references are needed, Rc<T> (single-threaded) or Arc<T>
// (multi-threaded) can be used.

// use std::rc::Rc;

// struct SelfRef {
//     data: Rc<String>,
//     reference: Rc<String>,
// }

// impl SelfRef {
//     fn new(data: String) -> Self {
//         let shared_data = Rc::new(data);
//         SelfRef {
//             data: Rc::clone(&shared_data),
//             reference: Rc::clone(&shared_data),
//         }
//     }
// }

// fn rc() {
//     let self_ref = SelfRef::new("Hello, Rust!".to_string());
//     println!("Data: {}", self_ref.reference);
// }

fn main() {
    let s = "Joe,Doe,1994-01-01".to_string();
    let c = load_record(&s);
    println!("{:?}", c);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
