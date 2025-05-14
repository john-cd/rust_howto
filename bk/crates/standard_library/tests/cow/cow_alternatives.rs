// ANCHOR: example
//! Alternatives to `Cow`.
//!
//! If you don't need Cow's flexibility, using `&str` for borrowing and `String`
//! for owned data might be sufficient.

use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

fn main() {
    // When working with multiple threads, `Arc<str>` provides shared ownership
    // of an immutable string slice.
    let string_slice: Arc<str> = Arc::from("Slice.");

    // If you need to mutate a shared string, use interior mutability.
    let shared_mutable_string: Arc<RwLock<String>> =
        Arc::new(RwLock::new("String".to_string()));

    let mut handles = vec![];
    for i in 0..2 {
        // We clone `Arc`, not the underlying string slice or lock!
        let string_slice_clone = Arc::clone(&string_slice);
        let shared_mutable_string_clone = Arc::clone(&shared_mutable_string);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50 * i));
            println!(
                "Thread {} - {} {}",
                i,
                string_slice_clone,
                shared_mutable_string_clone.read().unwrap()
            );
            // Edit the `String`.
            shared_mutable_string_clone.write().unwrap().push('!');
        });
        handles.push(handle);
    }

    // `Rc<str>` is similar to `Arc<str>`, but optimized for single-threaded
    // environments.
    let shared_string: Rc<str> = Rc::from("Hello, Rust!");
    let shared_string_clone = Rc::clone(&shared_string);
    println!("{}", shared_string_clone);

    // For heap-allocated, owned, and immutable data, `Box<str>` can be
    // more efficient than `String`.
    let boxed_string: Box<str> = "Hello, Rust!".into();
    println!("{}", boxed_string);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
