// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Alternatives to `Cow`.

use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

/// Shows alternatives to `Cow`: shared immutable strings and strings behind a
/// lock.
fn main() {
    // When working with multiple threads, `Arc<str>` provides shared ownership
    // of an immutable string slice.
    let string_slice: Arc<str> = Arc::from("Slice.");

    // When needing to mutate a shared string, use interior mutability.
    let shared_mutable_string: Arc<RwLock<String>> =
        Arc::new(RwLock::new("String".to_owned()));

    let mut handles = vec![];
    for i in 0..2 {
        // We clone `Arc`, not the underlying string slice or lock!
        let string_slice_clone = Arc::clone(&string_slice);
        let shared_mutable_string_clone = Arc::clone(&shared_mutable_string);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50 * i));
            println!(
                "Thread {i} - {string_slice_clone} {}",
                shared_mutable_string_clone.read().expect(
                    "the cow alternatives example invariant should hold"
                )
            );
            // Edit the `String`.
            shared_mutable_string_clone
                .write()
                .expect("the cow alternatives example invariant should hold")
                .push('!');
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("worker thread should not panic");
    }
    // `Rc<str>` is similar to `Arc<str>`, but optimized for single-threaded
    // environments.
    let shared_string: Rc<str> = Rc::from("Hello, Rust!");
    let shared_string_clone = Rc::clone(&shared_string);
    println!("{shared_string_clone}");

    // For heap-allocated, owned, and immutable data, `Box<str>` can be
    // more efficient than `String`.
    let boxed_string: Box<str> = "Hello, Rust!".into();
    println!("{boxed_string}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cow_alternatives_example_entry_point() {
        main();
    }

    #[test]
    fn test_arc_str_is_shared_not_copied() {
        let s: Arc<str> = Arc::from("Slice.");
        let clone = Arc::clone(&s);
        assert_eq!(Arc::strong_count(&s), 2);
        assert!(std::ptr::eq(&*s, &*clone));
    }

    #[test]
    fn test_rwlock_string_is_mutated_by_all_threads()
    -> Result<(), Box<dyn std::error::Error>> {
        let shared = Arc::new(RwLock::new("String".to_owned()));
        let handles: Vec<_> = (0..2)
            .map(|_| {
                let shared = Arc::clone(&shared);
                thread::spawn(move || {
                    shared
                        .write()
                        .map(|mut s| s.push('!'))
                        .map_err(|e| e.to_string())
                })
            })
            .collect();
        for handle in handles {
            handle.join().map_err(|_| "thread panicked")??;
        }
        assert_eq!(*shared.read().map_err(|e| e.to_string())?, "String!!");
        Ok(())
    }

    #[test]
    fn test_rc_str_and_box_str() {
        let rc: Rc<str> = Rc::from("Hello, Rust!");
        let rc2 = Rc::clone(&rc);
        assert_eq!(Rc::strong_count(&rc), 2);
        assert_eq!(&*rc2, "Hello, Rust!");
        let b: Box<str> = "Hello, Rust!".into();
        assert_eq!(&*b, &*rc);
    }
}
